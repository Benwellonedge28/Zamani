/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/neural-symbolic.g4
 *
 * Grammar:
 *     AINeuralSymbolic
 *
 * Status:
 *     CANONICAL NEURAL-SYMBOLIC COMPOSITION GRAMMAR
 *
 * Baseline:
 *     Rust 1.97+
 *     Rust 2021 edition
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
 * This file is the source-level composition boundary for neural and symbolic
 * computation.
 *
 * It describes how:
 *
 *     learned/neural computation
 *
 * may be composed with:
 *
 *     symbolic computation
 *
 * while remaining part of the single Zamani language.
 *
 * The grammar intentionally does NOT define a second neural language or a
 * second symbolic language.
 *
 *
 * ARCHITECTURAL INTENT
 * --------------------
 *
 * Neural-symbolic computation is composition, not a third independent
 * computational universe.
 *
 * Conceptually:
 *
 *     neural computation
 *            |
 *            +------------------+
 *            |                  |
 *            v                  v
 *        learned values     learned models
 *            |                  |
 *            +--------+---------+
 *                     |
 *                     v
 *             neural-symbolic
 *                     |
 *             +-------+-------+
 *             |               |
 *             v               v
 *       symbolic rules    symbolic reasoning
 *             |               |
 *             +-------+-------+
 *                     |
 *                     v
 *             canonical semantics
 *                     |
 *          +----------+----------+
 *          |                     |
 *          v                     v
 *      classical             quantum
 *                              |
 *                              v
 *                         quantum::ir
 *
 *
 * The neural-symbolic grammar therefore provides composition syntax only.
 *
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     - the neural-symbolic source construct;
 *     - neural-symbolic named bindings;
 *     - symbolic named bindings within a neural-symbolic region;
 *     - neural-symbolic composition relationships;
 *     - neural-symbolic region structure;
 *     - neural-symbolic source-level clauses;
 *     - neural-symbolic parser-level integration with ordinary Zamani
 *       statements.
 *
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - annotations;
 *     - general expressions;
 *     - general types;
 *     - general statements;
 *     - neural architecture semantics;
 *     - neural model semantics;
 *     - tensor semantics;
 *     - training;
 *     - inference;
 *     - differentiation;
 *     - optimization;
 *     - symbolic algebra;
 *     - symbolic reasoning;
 *     - knowledge representation;
 *     - learning algorithms;
 *     - adaptation algorithms;
 *     - probability;
 *     - uncertainty;
 *     - evidence;
 *     - provenance semantics;
 *     - agents;
 *     - actors;
 *     - concurrency;
 *     - distributed execution;
 *     - resource discovery;
 *     - capability discovery;
 *     - resource allocation;
 *     - security authorization;
 *     - policy enforcement;
 *     - hardware selection;
 *     - hardware discovery;
 *     - HDL semantics;
 *     - quantum operation semantics;
 *     - quantum routing;
 *     - quantum scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution;
 *     - vendor APIs;
 *     - framework APIs;
 *     - model loading;
 *     - model execution;
 *     - IR construction.
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/ai/neural.g4
 *     grammar/ai/symbolic.g4
 *     grammar/types/*
 *     grammar/expressions/*
 *     grammar/statements/*
 *
 * Direct parser dependencies:
 *
 *     Types
 *     Expressions
 *     Statements
 *
 *
 * IMPORTANT:
 *
 * `neural.g4` and `symbolic.g4` remain the semantic/source owners of their
 * respective domains.
 *
 * This file does not import them merely to duplicate their productions.
 *
 * The composition grammar instead uses the existing lexical distinctions:
 *
 *     NEURAL
 *     SYMBOLIC
 *     NEURAL_SYMBOLIC
 *
 * and delegates actual expressions/types/statements to the universal grammar.
 *
 *
 * ============================================================================
 * EXPORTS
 * ============================================================================
 *
 * Public parser rule:
 *
 *     neuralSymbolicConstruct
 *
 * Supporting public/reusable rules:
 *
 *     neuralSymbolicDeclaration
 *     neuralSymbolicBody
 *     neuralSymbolicMember
 *     neuralSymbolicBinding
 *     symbolicBinding
 *     neuralSymbolicComposition
 *
 *
 * ============================================================================
 * CONSUMED BY
 * ============================================================================
 *
 * Primary consumer:
 *
 *     grammar/ai/ai.g4
 *
 * The AI composition grammar must add:
 *
 *     import AINeuralSymbolic
 *
 * and:
 *
 *     | neuralSymbolicConstruct
 *
 * to its `aiConstruct` dispatcher.
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates ANTLR parser contexts only.
 *
 * It MUST NOT require a dedicated:
 *
 *     NeuralSymbolicAst
 *     NeuralSymbolicIR
 *     NeuralSymbolicProgram
 *
 * representation.
 *
 * The frontend AST remains domain-neutral.
 *
 * The semantic representation should preserve at least:
 *
 *     - source span;
 *     - declaration identity;
 *     - neural/symbolic role;
 *     - referenced expressions;
 *     - type expressions;
 *     - composition relationships;
 *     - source ordering;
 *     - source-level clauses;
 *     - nested statements.
 *
 * Exact Rust AST ownership remains with the existing frontend AST subsystem.
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes structure.
 *
 * Semantic analysis establishes meaning.
 *
 * Semantic analysis must determine:
 *
 *     - whether a neural binding resolves to a valid neural value/model;
 *     - whether a symbolic binding resolves to a valid symbolic value/rule;
 *     - whether composition is semantically legal;
 *     - type compatibility;
 *     - tensor compatibility;
 *     - shape compatibility;
 *     - value compatibility;
 *     - symbolic/neural interface compatibility;
 *     - differentiability where required;
 *     - effect compatibility;
 *     - capability requirements;
 *     - resource requirements;
 *     - contract requirements;
 *     - policy restrictions;
 *     - provenance requirements;
 *     - security restrictions;
 *     - portability;
 *     - dialect/version compatibility.
 *
 * The parser does none of these checks.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * All types come from the canonical type grammar.
 *
 * This file MUST NOT define:
 *
 *     NeuralType
 *     SymbolicType
 *     NeuralSymbolicType
 *
 * as a second type system.
 *
 * A neural binding and a symbolic binding both use:
 *
 *     typeExpression
 *
 * Semantic analysis determines whether the selected types are compatible.
 *
 * This permits:
 *
 *     scalar values
 *     tensors
 *     records
 *     functions
 *     symbolic values
 *     model values
 *     probabilistic values
 *     quantum-derived values
 *     distributed values
 *     future domain values
 *
 * without modifying this grammar.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO effects.
 *
 * Neural-symbolic programs may semantically require effects such as:
 *
 *     learning
 *     adaptation
 *     randomness
 *     measurement
 *     IO
 *     network
 *     distributed
 *     foreign
 *     native
 *     reflection
 *
 * Those effects remain owned by the universal effect subsystem.
 *
 * A parsed construct does not automatically receive authority to perform an
 * effect.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability requirements remain semantic.
 *
 * Examples include:
 *
 *     capability("tensor.compute")
 *     capability("neural.inference")
 *     capability("symbolic.reasoning")
 *     capability("quantum.compute")
 *
 * The grammar does not resolve capabilities.
 *
 * A capability requirement does NOT select a physical device.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource requirements remain open-ended expressions.
 *
 * This grammar introduces no resource limits.
 *
 * It MUST NOT define:
 *
 *     MAX_LAYERS
 *     MAX_NEURONS
 *     MAX_PARAMETERS
 *     MAX_TENSORS
 *     MAX_TENSOR_RANK
 *     MAX_SYMBOLIC_TERMS
 *     MAX_RULES
 *     MAX_WORKERS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_NETWORK_SIZE
 *
 * Concrete resource availability is determined downstream.
 *
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Requirements and guarantees remain universal Zamani semantics.
 *
 * A neural-symbolic body may contain ordinary universal statements such as:
 *
 *     requires ...
 *     ensures ...
 *     invariant ...
 *     assume ...
 *     guarantee ...
 *     property ...
 *
 * provided by the canonical statement/validation subsystem.
 *
 * This grammar does not redefine those constructs.
 *
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policy syntax remains owned by the policy/security/execution subsystem.
 *
 * A neural-symbolic construct may be subject to:
 *
 *     security policies
 *     execution policies
 *     adaptation policies
 *     resource policies
 *     deployment policies
 *     reproducibility policies
 *     provenance policies
 *
 * This grammar does not grant permissions merely because a construct parses.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The parser preserves source structure required for provenance.
 *
 * Semantic analysis may record:
 *
 *     source construct
 *     neural component
 *     symbolic component
 *     composition relationship
 *     evidence
 *     transformation
 *     decision
 *     verification
 *
 * Provenance ownership remains outside this grammar.
 *
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar owns NO IR.
 *
 * The semantic model determines whether a component lowers to:
 *
 *     classical semantic representation
 *     tensor computation
 *     symbolic computation
 *     distributed computation
 *     accelerator computation
 *     hybrid computation
 *     quantum computation
 *
 * When quantum computation is present, the canonical path remains:
 *
 *     neural-symbolic source
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum semantic model
 *          |
 *          v
 *     quantum::ir
 *
 * No neural-symbolic quantum IR is permitted.
 *
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum computation may participate through ordinary expressions or
 * semantically recognized values.
 *
 * For example, a semantic implementation may permit:
 *
 *     neural encoder -> quantum feature map -> symbolic decision
 *
 * but this grammar does not define:
 *
 *     qubits
 *     gates
 *     coupling maps
 *     physical qubits
 *     routing
 *     scheduling
 *     calibration
 *     QEC
 *     ZQN
 *     QPU selection
 *
 * Those remain quantum subsystem responsibilities.
 *
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Neural-symbolic computation may eventually be lowered toward:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     heterogeneous hardware
 *
 * but this grammar does not select any of them.
 *
 * Hardware realization is determined by:
 *
 *     capabilities
 *     resources
 *     policies
 *     target constraints
 *     compiler lowering
 *     scheduling
 *     backend realization
 *
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * The grammar describes computational intent rather than machine topology.
 *
 * A neural-symbolic source program must not need to be rewritten merely
 * because the realization changes from:
 *
 *     tiny system
 *     embedded system
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     HPC system
 *     cluster
 *     distributed system
 *     cloud
 *     future target
 *
 * "Infinity" means:
 *
 *     no artificial finite language-level capacity ceiling.
 *
 * Every actual compilation/execution remains finite because actual programs,
 * compiler processes, memory and execution targets are finite.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source tokens
 *     grammar
 *     lexer vocabulary
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware availability
 *     filesystem state
 *     network state
 *     environment variables
 *     wall-clock time
 *     randomness
 *     runtime state
 *     model files
 *     device discovery
 *
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This file contains:
 *
 *     - no Rust actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no process execution;
 *     - no hardware access;
 *     - no environment access;
 *     - no secret access;
 *     - no unsafe implementation.
 *
 * Generated compiler/runtime code must remain safe Rust.
 *
 * Required baseline:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     #![forbid(unsafe_code)]
 *
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * The lexer already provides:
 *
 *     NEURAL
 *     SYMBOLIC
 *     NEURAL_SYMBOLIC
 *
 * Therefore this file introduces NO new lexer tokens.
 *
 * This is important because:
 *
 *     new neural algorithm
 *     new symbolic algorithm
 *     new model architecture
 *     new reasoning system
 *
 * must not require global keyword expansion.
 *
 * Future constructs should use:
 *
 *     identifiers
 *     qualified names
 *     expressions
 *     dialects
 *     capabilities
 *     libraries
 *     semantic registration
 *
 * where appropriate.
 *
 *
 * ============================================================================
 * ANTLR DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     AINeuralSymbolic
 *          |
 *          +--> Types
 *          +--> Expressions
 *          +--> Statements
 *
 * It MUST NOT import:
 *
 *     AI
 *
 * because that would create:
 *
 *     AI -> AINeuralSymbolic -> AI
 *
 * It MUST NOT import:
 *
 *     quantum grammar
 *     hardware grammar
 *     runtime grammar
 *     backend grammar
 *
 * because those are semantic/lowering concerns.
 *
 *
 * ============================================================================
 * IMPORTANT COMPOSITION RULE
 * ============================================================================
 *
 * Existing neural.g4 and symbolic.g4 are independent leaf grammars.
 *
 * This file does NOT attempt to combine:
 *
 *     neuralConstruct
 *
 * and:
 *
 *     aiSymbolicConstruct
 *
 * directly as alternatives.
 *
 * That would create an ambiguous annotation-led dispatch boundary because
 * both existing grammars use open annotation structure.
 *
 * Instead, this grammar introduces one explicit lexical composition boundary:
 *
 *     NEURAL_SYMBOLIC
 *
 * followed by a named composition region.
 *
 * This gives the AI aggregate grammar a deterministic entry point.
 *
 *
 * ============================================================================
 * SOURCE MODEL
 * ============================================================================
 *
 * Canonical source shape:
 *
 *     neural_symbolic Name {
 *         neural encoder : SomeType = model;
 *         symbolic rule : SomeType = rules;
 *
 *         neural_symbolic encoder -> rule;
 *
 *         ...
 *     }
 *
 * The precise semantic interpretation of:
 *
 *     neural
 *     symbolic
 *     neural_symbolic
 *
 * is established downstream.
 *
 * The grammar only establishes structural composition.
 *
 *
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 */

parser grammar AINeuralSymbolic;

options {
    tokenVocab = ZamaniLexer;
}

import
    Types,
    Expressions,
    Statements
;


/* ============================================================================
 * 1. PUBLIC ENTRY
 * ========================================================================== */

/**
 * Canonical neural-symbolic composition construct.
 *
 * Example:
 *
 *     neural_symbolic Classifier {
 *         neural encoder = model;
 *         symbolic decision = rules;
 *
 *         neural_symbolic encoder -> decision;
 *     }
 *
 * `NEURAL_SYMBOLIC` is already part of the canonical lexer vocabulary.
 *
 * No new lexer token is required.
 */
neuralSymbolicConstruct
    : neuralSymbolicDeclaration
    ;


/* ============================================================================
 * 2. DECLARATION
 * ========================================================================== */

/**
 * Declares a reusable neural-symbolic computation region.
 *
 * The name is an ordinary Zamani identifier.
 *
 * No finite number of members is imposed.
 */
neuralSymbolicDeclaration
    : NEURAL_SYMBOLIC
      identifier
      neuralSymbolicHeader*
      neuralSymbolicBody
    ;


/* ============================================================================
 * 3. DECLARATION HEADERS
 * ========================================================================== */

/**
 * Header properties remain open-ended.
 *
 * Examples:
 *
 *     neural_symbolic Model,
 *         domain = classification
 *     {
 *         ...
 *     }
 *
 *     neural_symbolic Model,
 *         policy = execution_policy
 *     {
 *         ...
 *     }
 *
 * Property meaning is semantic.
 */
neuralSymbolicHeader
    : COMMA
      neuralSymbolicClause
    ;


/* ============================================================================
 * 4. BODY
 * ========================================================================== */

/**
 * The body is an unbounded ordered sequence.
 *
 * Source order is preserved by the parser/AST layer.
 */
neuralSymbolicBody
    : LBRACE
      neuralSymbolicMember*
      RBRACE
    ;


/* ============================================================================
 * 5. BODY MEMBER DISPATCH
 * ========================================================================== */

/**
 * The first token deliberately determines the specialized construct:
 *
 *     NEURAL
 *         -> neural binding
 *
 *     SYMBOLIC
 *         -> symbolic binding
 *
 *     NEURAL_SYMBOLIC
 *         -> composition relationship or nested composition
 *
 * Anything else is delegated to the universal statement grammar.
 *
 * This avoids a second statement language while retaining deterministic
 * parser dispatch.
 */
neuralSymbolicMember
    : neuralSymbolicBinding
    | symbolicBinding
    | neuralSymbolicComposition
    | statement
    ;


/* ============================================================================
 * 6. NEURAL BINDING
 * ========================================================================== */

/**
 * Binds a neural value/model/component into the neural-symbolic region.
 *
 * Examples:
 *
 *     neural encoder = model;
 *
 *     neural encoder : Encoder = model;
 *
 *     neural representation = embedding;
 *
 * The identifier is ordinary source data.
 *
 * The grammar does not enumerate neural architectures.
 */
neuralSymbolicBinding
    : NEURAL
      identifier
      neuralSymbolicTypeClause?
      neuralSymbolicInitializer?
      neuralSymbolicClause*
      SEMICOLON?
    ;


/* ============================================================================
 * 7. SYMBOLIC BINDING
 * ========================================================================== */

/**
 * Binds a symbolic value/rule/knowledge representation into the region.
 *
 * Examples:
 *
 *     symbolic rules = knowledge;
 *
 *     symbolic decision : DecisionRule = policy;
 *
 *     symbolic relation = relation_set;
 *
 * The symbolic meaning remains open-ended.
 */
symbolicBinding
    : SYMBOLIC
      identifier
      neuralSymbolicTypeClause?
      neuralSymbolicInitializer?
      neuralSymbolicClause*
      SEMICOLON?
    ;


/* ============================================================================
 * 8. NEURAL-SYMBOLIC COMPOSITION
 * ========================================================================== */

/**
 * Explicitly connects two source-level computations.
 *
 * Examples:
 *
 *     neural_symbolic encoder -> rules;
 *
 *     neural_symbolic features -> decision;
 *
 *     neural_symbolic model -> explanation;
 *
 * The left and right operands are ordinary expressions.
 *
 * Therefore composition can eventually connect:
 *
 *     neural value
 *     symbolic value
 *     tensor
 *     function
 *     model
 *     classical computation
 *     quantum-derived value
 *     distributed value
 *     future-domain value
 *
 * without changing this grammar.
 */
neuralSymbolicComposition
    : NEURAL_SYMBOLIC
      expression
      ARROW
      expression
      neuralSymbolicClause*
      SEMICOLON?
    ;


/* ============================================================================
 * 9. TYPE CLAUSE
 * ========================================================================== */

/**
 * Reuses the universal type grammar.
 */
neuralSymbolicTypeClause
    : COLON
      typeExpression
    ;


/* ============================================================================
 * 10. INITIALIZER
 * ========================================================================== */

/**
 * Reuses the universal expression grammar.
 */
neuralSymbolicInitializer
    : ASSIGN
      expression
    ;


/* ============================================================================
 * 11. OPEN-ENDED CLAUSE
 * ========================================================================== */

/**
 * Neural-symbolic clauses deliberately use identifiers rather than a closed
 * keyword catalogue.
 *
 * Examples:
 *
 *     policy = execution_policy
 *     evidence = evidence_source
 *     confidence = confidence_value
 *     context = reasoning_context
 *     differentiable = property
 *     capability = capability_value
 *     requirement = requirement_value
 *     provenance = provenance_value
 *
 * Semantic analysis determines which clauses are legal for each construct.
 */
neuralSymbolicClause
    : identifier
      ASSIGN
      expression
    ;


/* ============================================================================
 * 12. EXPRESSION BRIDGE
 * ========================================================================== */

/**
 * Explicit reusable bridge for downstream composition grammars.
 *
 * This is intentionally just the canonical expression.
 */
neuralSymbolicExpression
    : expression
    ;


/* ============================================================================
 * 13. TYPE BRIDGE
 * ========================================================================== */

/**
 * Explicit reusable bridge for downstream composition grammars.
 */
neuralSymbolicType
    : typeExpression
    ;


/* ============================================================================
 * 14. ARGUMENT BRIDGE
 * ========================================================================== */

/**
 * Reuses the canonical argument-list implementation.
 *
 * This rule does not create an AI-specific argument system.
 */
neuralSymbolicArguments
    : argumentList
    ;


/* ============================================================================
 * 15. STATEMENT BRIDGE
 * ========================================================================== */

/**
 * Explicit reusable statement bridge.
 */
neuralSymbolicStatement
    : statement
    ;


/* ============================================================================
 * 16. NESTED COMPOSITION
 * ========================================================================== */

/**
 * A nested neural-symbolic construct is already represented by:
 *
 *     neuralSymbolicMember
 *
 * through:
 *
 *     neuralSymbolicComposition
 *
 * and ordinary statements.
 *
 * A full nested declaration is deliberately not added here.
 *
 * This avoids creating recursive declaration structures that have no
 * independent semantic requirement.
 *
 * If nested declarations become necessary, they should be introduced as a
 * separate language-specification change with an explicit AST/semantic
 * contract rather than silently added to this grammar.
 */


/* ============================================================================
 * 17. RESOURCE / CAPABILITY BRIDGE
 * ========================================================================== */

/**
 * Resource and capability values are ordinary expressions.
 *
 * This file does not duplicate:
 *
 *     grammar/resources/
 *
 * or:
 *
 *     grammar/ai/capabilities.g4
 *
 * Examples may be carried through clauses or universal statements:
 *
 *     requires capability("tensor.compute");
 *
 *     requires capability("symbolic.reasoning");
 *
 *     requires memory >= required_memory;
 *
 *     requires capability("quantum.compute");
 */
neuralSymbolicResourceExpression
    : expression
    ;


/* ============================================================================
 * 18. EFFECT BRIDGE
 * ========================================================================== */

/**
 * Effects are semantic properties.
 *
 * The grammar exposes no effect-specific syntax here.
 */
neuralSymbolicEffectExpression
    : expression
    ;


/* ============================================================================
 * 19. POLICY BRIDGE
 * ========================================================================== */

/**
 * Policy references remain ordinary expressions.
 */
neuralSymbolicPolicyExpression
    : expression
    ;


/* ============================================================================
 * 20. PROVENANCE BRIDGE
 * ========================================================================== */

/**
 * Provenance references remain ordinary expressions.
 */
neuralSymbolicProvenanceExpression
    : expression
    ;


/* ============================================================================
 * 21. CROSS-DOMAIN VALUE BRIDGES
 * ========================================================================== */

/**
 * Classical values remain ordinary expressions.
 */
neuralSymbolicClassicalExpression
    : expression
    ;


/**
 * Quantum values remain ordinary expressions.
 *
 * Semantic analysis is responsible for identifying quantum computation and
 * lowering it through quantum::ir.
 */
neuralSymbolicQuantumExpression
    : expression
    ;


/**
 * Hybrid values remain ordinary expressions.
 */
neuralSymbolicHybridExpression
    : expression
    ;


/**
 * Data values remain ordinary expressions.
 */
neuralSymbolicDataExpression
    : expression
    ;


/**
 * Distributed values remain ordinary expressions.
 */
neuralSymbolicDistributedExpression
    : expression
    ;


/**
 * Hardware-related values remain ordinary expressions.
 */
neuralSymbolicHardwareExpression
    : expression
    ;


/* ============================================================================
 * 22. PORTABILITY
 * ========================================================================== */

/*
 * There are deliberately no grammar rules such as:
 *
 *     gpu
 *     cpu
 *     qpu
 *     fpga
 *     node
 *     device
 *
 * inside this grammar.
 *
 * Target realization is downstream.
 */


/* ============================================================================
 * 23. ERROR CONTRACT
 * ============================================================================
 *
 * Parser errors include:
 *
 *     missing neural-symbolic name
 *     missing body
 *     missing closing brace
 *     missing neural binding name
 *     missing symbolic binding name
 *     malformed type expression
 *     malformed initializer
 *     malformed composition arrow
 *     malformed clause
 *     missing expression
 *     malformed statement
 *
 * Semantic errors include:
 *
 *     unknown neural value
 *     unknown symbolic value
 *     invalid neural/symbolic relationship
 *     incompatible types
 *     incompatible tensor shapes
 *     invalid differentiability requirement
 *     unsatisfied capability
 *     unsatisfied resource requirement
 *     conflicting policy
 *     invalid effect
 *     invalid provenance requirement
 *     unsupported target
 *     invalid quantum boundary
 *
 * Parser and semantic diagnostics MUST remain separate.
 */


/* ============================================================================
 * 24. POSITIVE TEST CONTRACT
 * ========================================================================== */

/*
 * The following forms must parse:
 *
 *     neural_symbolic Classifier {
 *         neural encoder = model;
 *         symbolic rules = knowledge;
 *         neural_symbolic encoder -> rules;
 *     }
 *
 *     neural_symbolic Classifier {
 *         neural encoder : Encoder = model;
 *         symbolic decision : Decision = rules;
 *         neural_symbolic encoder -> decision;
 *     }
 *
 *     neural_symbolic Hybrid {
 *         neural model = learned_model;
 *         symbolic rules = rule_set;
 *
 *         neural_symbolic model -> rules,
 *             confidence = confidence_value;
 *     }
 *
 *     neural_symbolic QuantumDecision {
 *         neural encoder = quantum_feature_model;
 *         symbolic decision = reasoning_model;
 *
 *         neural_symbolic encoder -> decision;
 *
 *         requires capability("quantum.compute");
 *         requires capability("symbolic.reasoning");
 *         requires capability("tensor.compute");
 *     }
 *
 *     neural_symbolic DistributedDecision {
 *         neural model = distributed_model;
 *         symbolic policy = policy_value;
 *
 *         neural_symbolic model -> policy;
 *
 *         requires capability("distributed.compute");
 *         requires memory >= required_memory;
 *     }
 *
 *     neural_symbolic Computation {
 *         neural encoder = model;
 *         symbolic rules = rules;
 *
 *         value = encoder(input);
 *         result = rules(value);
 *     }
 */


/* ============================================================================
 * 25. NEGATIVE TEST CONTRACT
 * ========================================================================== */

/*
 * The following forms must be rejected structurally:
 *
 *     neural_symbolic;
 *
 *     neural_symbolic {
 *     }
 *
 *     neural_symbolic Name
 *
 *     neural_symbolic Name {
 *
 *     neural;
 *
 *     symbolic;
 *
 *     neural encoder;
 *
 *     symbolic rules;
 *
 *     neural encoder : ;
 *
 *     symbolic rules = ;
 *
 *     neural_symbolic encoder;
 *
 *     neural_symbolic encoder -> ;
 *
 *     neural_symbolic -> decision;
 *
 *     neural_symbolic encoder -> ;
 *
 *     neural encoder = model
 *         <malformed enclosing syntax>
 */


/* ============================================================================
 * 26. BOUNDARY TEST CONTRACT
 * ========================================================================== */

/*
 * Test:
 *
 *     one neural binding
 *     one symbolic binding
 *     one composition
 *
 * then progressively increase:
 *
 *     declaration count
 *     binding count
 *     composition count
 *     clause count
 *     expression size
 *     type complexity
 *     nesting of ordinary statements
 *     qualified-name depth
 *     cross-domain references
 *
 * No test value becomes a language-level maximum.
 */


/* ============================================================================
 * 27. SCALABILITY TEST CONTRACT
 * ========================================================================== */

/*
 * The grammar must remain structurally open for:
 *
 *     arbitrarily many neural bindings
 *     arbitrarily many symbolic bindings
 *     arbitrarily many composition relationships
 *     arbitrarily many statements
 *     arbitrarily large expressions
 *     arbitrarily large symbolic structures
 *     arbitrarily large neural structures
 *     arbitrary tensor rank
 *     arbitrary tensor dimensions
 *     arbitrary model sizes
 *     arbitrary symbolic knowledge sizes
 *     arbitrary resource requirements
 *
 * subject only to actual compiler/runtime/resource limits.
 *
 * No universal capacity constants may be added.
 */


/* ============================================================================
 * 28. CROSS-DOMAIN TEST CONTRACT
 * ========================================================================== */

/*
 * Neural-symbolic composition must be tested with:
 *
 *     classical
 *     tensor
 *     data
 *     reasoning
 *     knowledge
 *     learning
 *     adaptation
 *     uncertainty
 *     probability
 *     evidence
 *     provenance
 *     contracts
 *     policies
 *     concurrency
 *     distributed
 *     networking
 *     security
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     simulation
 *     FFI
 *     ABI
 *     metaprogramming
 *
 * The neural-symbolic grammar must not create specialized versions of those
 * domains.
 */


/* ============================================================================
 * 29. QUANTUM INTEGRATION TEST
 * ========================================================================== */

/*
 * At least one semantic integration test must verify:
 *
 *     neural computation
 *          |
 *          v
 *     quantum-derived value
 *          |
 *          v
 *     symbolic computation
 *          |
 *          v
 *     semantic model
 *          |
 *          v
 *     quantum::ir
 *
 * The grammar itself must remain unaware of:
 *
 *     physical qubits
 *     topology
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 */


/* ============================================================================
 * 30. POCO-REAF TEST
 * ========================================================================== */

/*
 * The same source program must remain semantically valid when the compiler
 * evaluates different compatible realization classes, for example:
 *
 *     CPU
 *     multicore
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     HPC
 *     cluster
 *     distributed system
 *     cloud
 *     future target
 *
 * where the required capabilities exist.
 *
 * Target absence must produce a semantic/resource diagnostic rather than
 * forcing source-language rewriting.
 */


/* ============================================================================
 * 31. DETERMINISM TEST
 * ========================================================================== */

/*
 * Repeated parsing of identical source under identical grammar/lexer versions
 * must produce structurally equivalent parser results.
 *
 * Parsing must not depend on:
 *
 *     target hardware
 *     model files
 *     filesystem
 *     network
 *     environment
 *     wall clock
 *     randomness
 */


/* ============================================================================
 * 32. COMPATIBILITY CONTRACT
 * ========================================================================== */

/*
 * This file introduces:
 *
 *     parser grammar AINeuralSymbolic
 *
 * and:
 *
 *     neuralSymbolicConstruct
 *
 * It does not rename:
 *
 *     neural.g4
 *     symbolic.g4
 *     ai.g4
 *
 * It does not change the public entry rules:
 *
 *     neuralConstruct
 *     aiSymbolicConstruct
 *
 * Existing neural and symbolic consumers therefore remain source-compatible.
 *
 * The only required AI aggregate integration is:
 *
 *     AI
 *       |
 *       +--> AINeuralSymbolic
 *
 * No reverse dependency is introduced.
 */


/* ============================================================================
 * 33. HARD-CODING AUDIT
 * ========================================================================== */

/*
 * This grammar contains:
 *
 *     NO maximum neural count
 *     NO maximum symbolic count
 *     NO maximum composition count
 *     NO maximum model size
 *     NO maximum layer count
 *     NO maximum tensor rank
 *     NO maximum tensor dimension
 *     NO maximum rule count
 *     NO maximum term count
 *     NO maximum argument count
 *     NO maximum worker count
 *     NO maximum CPU count
 *     NO maximum GPU count
 *     NO maximum FPGA count
 *     NO maximum QPU count
 *     NO maximum node count
 *     NO maximum device count
 *     NO maximum memory capacity
 *     NO maximum network size
 *     NO fixed register width
 *     NO fixed topology
 *     NO vendor catalogue
 *     NO framework catalogue
 *     NO model catalogue
 *     NO algorithm catalogue
 *
 * It also does not select:
 *
 *     GPU 0
 *     CPU 0
 *     FPGA 0
 *     QPU 0
 *     node 0
 *     device 0
 *
 * Physical realization remains downstream.
 */


/* ============================================================================
 * 34. ANTLR / RUST SAFETY AUDIT
 * ========================================================================== */

/*
 * ANTLR:
 *
 *     - parser grammar only;
 *     - tokenVocab only;
 *     - no lexer rules;
 *     - no actions;
 *     - no semantic predicates;
 *     - no embedded target-language code.
 *
 * Rust:
 *
 *     - Rust 1.97+;
 *     - Rust 2021;
 *     - safe Rust;
 *     - no unsafe;
 *     - no unsafe dependency introduced by this grammar.
 *
 * Any compiler crate consuming generated parser code should enforce:
 *
 *     #![forbid(unsafe_code)]
 *
 * independently at its Rust crate boundary.
 */


/* ============================================================================
 * 35. COMPLETION CRITERIA
 * ========================================================================== */

/*
 * This file is DONE when:
 *
 * [ ] `AINeuralSymbolic` is the grammar identity.
 *
 * [ ] `ZamaniLexer` is the only lexer vocabulary.
 *
 * [ ] No new lexer token is required.
 *
 * [ ] `NEURAL_SYMBOLIC` is the explicit composition boundary.
 *
 * [ ] `Types` is reused.
 *
 * [ ] `Expressions` is reused.
 *
 * [ ] `Statements` is reused.
 *
 * [ ] No duplicate neural grammar is created.
 *
 * [ ] No duplicate symbolic grammar is created.
 *
 * [ ] No duplicate type system is created.
 *
 * [ ] No duplicate expression system is created.
 *
 * [ ] No duplicate statement system is created.
 *
 * [ ] No AI-specific IR is created.
 *
 * [ ] No neural-symbolic IR is created.
 *
 * [ ] Quantum semantics remain outside this grammar.
 *
 * [ ] Quantum computation reaches `quantum::ir` downstream.
 *
 * [ ] Hardware selection remains downstream.
 *
 * [ ] Resource allocation remains downstream.
 *
 * [ ] Capability authorization remains downstream.
 *
 * [ ] Policy evaluation remains downstream.
 *
 * [ ] Provenance remains downstream.
 *
 * [ ] No hard-coded machine capacity exists.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Compatibility tests exist.
 *
 * [ ] AI aggregate integration is complete.
 *
 * [ ] ANTLR generation succeeds.
 *
 * [ ] Rust 1.97+ compilation succeeds.
 *
 * [ ] No unsafe Rust is required.
 */


/* ============================================================================
 * 36. FINAL ARCHITECTURAL INVARIANT
 * ========================================================================== */

/*
 *
 *                 NEURAL-SYMBOLIC SOURCE
 *                         |
 *                         v
 *                   Zamani parser
 *                         |
 *                         v
 *                 Domain-neutral AST
 *                         |
 *             +-----------+-----------+
 *             |                       |
 *             v                       v
 *        neural semantics       symbolic semantics
 *             |                       |
 *             +-----------+-----------+
 *                         |
 *                         v
 *                  semantic model
 *                         |
 *        +----------------+----------------+
 *        |                |                |
 *        v                v                v
 *    classical         quantum          hybrid
 *                         |
 *                         v
 *                    quantum::ir
 *                         |
 *                         v
 *                 optimization/lowering
 *                         |
 *                         v
 *                routing/scheduling
 *                         |
 *                         v
 *                    resilience
 *                         |
 *                         v
 *                       ZQN
 *                         |
 *                         v
 *                       HAL
 *                         |
 *                         v
 *                  target realization
 *
 *
 * The neural-symbolic grammar therefore expresses:
 *
 *     WHAT is composed
 *
 * while downstream systems determine:
 *
 *     HOW it is represented;
 *     WHERE it runs;
 *     WHICH resources realize it;
 *     WHICH compatible target is selected;
 *     HOW it is optimized;
 *     HOW it is routed;
 *     HOW it is scheduled;
 *     HOW it is made resilient.
 *
 * This preserves POCO-REAF and prevents present-day hardware limitations from
 * becoming permanent language limitations.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */