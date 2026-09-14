/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/differentiation.g4
 *
 * Status:
 *     Production AI differentiation-domain parser grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust integration baseline:
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
 * This file owns the SOURCE-LEVEL SYNTAX for differentiation intent in Zamani.
 *
 * Differentiation is deliberately represented as a semantic operation rather
 * than as an implementation algorithm.
 *
 * The grammar can express differentiation involving:
 *
 *     - scalar functions;
 *     - vector-valued functions;
 *     - tensor-valued functions;
 *     - model expressions;
 *     - loss/objective expressions;
 *     - symbolic expressions;
 *     - numerical expressions;
 *     - parameterized computations;
 *     - classical computations;
 *     - quantum-derived values;
 *     - hardware-backed computations;
 *     - distributed computations;
 *     - user-defined differentiable operations;
 *     - future differentiable domains.
 *
 * The grammar does NOT select:
 *
 *     - reverse mode;
 *     - forward mode;
 *     - symbolic differentiation;
 *     - numerical differentiation;
 *     - finite differences;
 *     - parameter shift;
 *     - adjoint differentiation;
 *     - automatic differentiation implementation;
 *     - a particular differentiation engine;
 *     - a particular tensor library;
 *     - a particular accelerator;
 *     - a particular device;
 *     - a particular machine.
 *
 * Semantic analysis and later compilation/lowering stages determine whether
 * and how a differentiation request can be realized.
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
 *     Canonical parser
 *          |
 *          +------------------------------+
 *          |                              |
 *          v                              v
 *     Types / Expressions             AI domain
 *          |                              |
 *          +---------------+--------------+
 *                          |
 *                          v
 *                    Frontend AST
 *                          |
 *                          v
 *                 Semantic analysis
 *                          |
 *              +-----------+-----------+
 *              |                       |
 *              v                       v
 *       Differentiation model     Resource metadata
 *              |                       |
 *              +-----------+-----------+
 *                          |
 *                          v
 *                 Canonical semantic IR
 *                          |
 *          +---------------+----------------+
 *          |               |                |
 *          v               v                v
 *      Classical        Quantum         Accelerator
 *         IR              IR             lowering
 *          |               |                |
 *          +---------------+----------------+
 *                          |
 *                          v
 *                     Optimization
 *                          |
 *                          v
 *                      Scheduling
 *                          |
 *                          v
 *                   Target realization
 *                          |
 *                          v
 *                       Runtime
 *
 * This grammar MUST NOT construct IR directly.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - differentiation declaration syntax;
 *     - differentiation expression syntax;
 *     - differentiation region syntax;
 *     - differentiation target syntax;
 *     - differentiation variable syntax;
 *     - differentiation result-kind syntax;
 *     - differentiation mode/strategy REQUEST syntax;
 *     - differentiation order syntax;
 *     - differentiation resource/capability requirement boundaries;
 *     - differentiation constraint/preference boundaries;
 *     - differentiation metadata boundaries;
 *     - differentiation composition syntax;
 *     - differentiation interoperability boundaries.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer definitions;
 *     - identifiers;
 *     - general expressions;
 *     - general types;
 *     - tensors;
 *     - tensor storage;
 *     - tensor layouts;
 *     - numerical kernels;
 *     - automatic-differentiation algorithms;
 *     - symbolic algebra implementation;
 *     - finite-difference implementation;
 *     - parameter-shift implementation;
 *     - adjoint implementation;
 *     - optimizer algorithms;
 *     - training algorithms;
 *     - model implementation;
 *     - quantum IR;
 *     - classical IR;
 *     - HDL IR;
 *     - hardware discovery;
 *     - accelerator discovery;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - QEC;
 *     - ZQN;
 *     - backend selection;
 *     - device selection;
 *     - runtime execution.
 *
 * ============================================================================
 *
 * LEXER POLICY
 * ============================================================================
 *
 * Differentiation vocabulary MUST NOT require an ever-growing collection of
 * reserved lexer keywords.
 *
 * The canonical lexer provides generic annotation syntax through:
 *
 *     NANO_ANNOTATION
 *
 * Therefore the following may be represented semantically without becoming
 * permanent lexer keywords:
 *
 *     @differentiate
 *     @gradient
 *     @jacobian
 *     @hessian
 *     @derivative
 *     @differentiate_function
 *
 * The semantic layer validates annotation spelling and meaning.
 *
 * This preserves extensibility for:
 *
 *     - future differentiation methods;
 *     - user libraries;
 *     - domain-specific differentiation;
 *     - vendor-independent dialects;
 *     - experimental features.
 *
 * ============================================================================
 *
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Differentiation syntax describes WHAT derivative computation is requested.
 *
 * It MUST NOT silently encode:
 *
 *     - number of CPU cores;
 *     - number of GPU devices;
 *     - GPU model;
 *     - accelerator ID;
 *     - memory capacity;
 *     - tensor storage capacity;
 *     - thread count;
 *     - SIMD width;
 *     - cluster size;
 *     - network topology;
 *     - quantum processor identity;
 *     - number of physical qubits;
 *     - hardware topology;
 *     - vendor implementation.
 *
 * If a differentiation operation requires resources, those requirements are
 * represented separately as semantic requirements, capabilities, constraints,
 * preferences, or hints.
 *
 * ============================================================================
 *
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There are NO grammar-level finite limits for:
 *
 *     - differentiation order;
 *     - number of differentiation variables;
 *     - number of outputs;
 *     - number of inputs;
 *     - number of parameters;
 *     - tensor rank;
 *     - tensor dimensions;
 *     - model size;
 *     - expression size;
 *     - number of derivative operations;
 *     - number of nested differentiation operations;
 *     - number of resource requirements;
 *     - number of capabilities;
 *     - number of constraints;
 *     - number of preferences;
 *     - number of differentiation regions.
 *
 * Repetition is structural.
 *
 * Practical limits belong to:
 *
 *     parser resource policy;
 *     semantic analysis;
 *     compiler policy;
 *     memory/resource management;
 *     target capabilities;
 *     runtime resources.
 *
 * ============================================================================
 *
 * TYPE CONTRACT
 * ============================================================================
 *
 * Type syntax belongs to the canonical type grammar.
 *
 * This file therefore consumes:
 *
 *     typeExpression
 *
 * and does NOT define another type system.
 *
 * Differentiation results may semantically have:
 *
 *     scalar type;
 *     vector type;
 *     matrix type;
 *     tensor type;
 *     structured derivative type;
 *     user-defined derivative type;
 *     future derivative representation.
 *
 * The actual representation is determined downstream.
 *
 * ============================================================================
 *
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * General expression syntax belongs to:
 *
 *     grammar/expressions/expressions.g4
 *
 * This grammar consumes:
 *
 *     expression
 *
 * rather than redefining arithmetic, calls, indexing, operators, lambdas,
 * member access, tensor indexing, quantum expressions, or other expression
 * constructs.
 *
 * ============================================================================
 *
 * DIFFERENTIATION SEMANTICS
 * ============================================================================
 *
 * A differentiation request has four conceptually distinct components:
 *
 *     1. SUBJECT
 *        The computation/function being differentiated.
 *
 *     2. VARIABLES
 *        The independent variables with respect to which differentiation is
 *        requested.
 *
 *     3. ORDER / RESULT KIND
 *        The mathematical derivative requested.
 *
 *     4. EXECUTION PREFERENCES
 *        Optional semantic requests such as an implementation mode or
 *        numerical/symbolic preference.
 *
 * These must remain distinct.
 *
 * In particular:
 *
 *     "gradient"
 *
 * does not mean:
 *
 *     "use reverse-mode automatic differentiation on GPU X".
 *
 * ============================================================================
 *
 * MODE CONTRACT
 * ============================================================================
 *
 * A differentiation mode is a REQUESTED semantic strategy, not a guaranteed
 * implementation.
 *
 * Examples include:
 *
 *     automatic;
 *     symbolic;
 *     numerical;
 *     forward;
 *     reverse;
 *     adjoint;
 *     parameter_shift;
 *     finite_difference;
 *     custom.
 *
 * The grammar accepts extensible identifiers for strategies.
 *
 * Semantic analysis determines:
 *
 *     - whether the strategy is supported;
 *     - whether it is compatible with the subject;
 *     - whether conversion is possible;
 *     - whether a different strategy may satisfy the program contract;
 *     - whether the request is a hard requirement or merely a preference.
 *
 * ============================================================================
 *
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Differentiation may apply to computations containing quantum operations.
 *
 * This grammar does NOT implement:
 *
 *     - parameter-shift rules;
 *     - adjoint differentiation;
 *     - quantum gradient circuits;
 *     - quantum measurements;
 *     - QEC;
 *     - ZQN;
 *     - pulse execution;
 *     - physical qubit selection.
 *
 * Those are downstream semantic/compiler concerns.
 *
 * The differentiation subject may therefore contain a quantum expression if
 * the canonical expression grammar permits it.
 *
 * The eventual lowering path remains:
 *
 *     source
 *       -> AST
 *       -> semantic differentiation representation
 *       -> quantum::ir / classical IR as appropriate
 *       -> differentiation lowering
 *       -> optimization
 *       -> scheduling
 *       -> hardware realization
 *       -> runtime
 *
 * ============================================================================
 *
 * CLASSICAL / SYMBOLIC CONTRACT
 * ============================================================================
 *
 * Differentiation may apply to ordinary classical expressions.
 *
 * Symbolic differentiation remains semantically distinct from numerical
 * differentiation.
 *
 * This grammar does not implement algebraic simplification.
 *
 * ============================================================================
 *
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource declarations in this file describe requirements, constraints,
 * capabilities, preferences, or hints.
 *
 * They do NOT allocate resources.
 *
 * The grammar intentionally accepts expressions for resource values so that
 * resource quantities can remain dynamic and target-independent.
 *
 * ============================================================================
 *
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must be deterministic for the same token stream.
 *
 * No semantic predicates or target-specific parser decisions are used.
 *
 * Semantic strategy selection MUST NOT occur during parsing.
 *
 * ============================================================================
 *
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This grammar uses generic lexical infrastructure and does not require new
 * lexer keywords for differentiation.
 *
 * Existing programs remain unaffected because the new differentiation
 * constructs are entered through explicit differentiation annotations.
 *
 * ============================================================================
 *
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It imports:
 *
 *     Types
 *     Expressions
 *
 * It can be imported by:
 *
 *     AI
 *
 * without requiring the differentiation grammar to import AI itself.
 *
 * This direction is intentional:
 *
 *     AI -> differentiation
 *
 * rather than:
 *
 *     AI <-> differentiation
 *
 * which would create a circular grammar dependency.
 *
 * ============================================================================
 */

parser grammar AIDifferentiation;

options {
    tokenVocab = ZamaniLexer;
}

import Types, Expressions;


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ========================================================================== */

/**
 * Public differentiation syntax boundary.
 *
 * Semantic analysis determines the precise differentiation construct.
 */
aiDifferentiationConstruct
    : aiDifferentiationDeclaration
    | aiDifferentiationExpression
    | aiDifferentiationRegion
    ;


/* ============================================================================
 * 2. DIFFERENTIATION DECLARATION
 * ========================================================================== */

/**
 * Declarative differentiation binding.
 *
 * Example:
 *
 *     @differentiate gradient_of_loss = derivative(loss, weights);
 *
 * The annotation is semantically validated.
 */
aiDifferentiationDeclaration
    : differentiationAnnotation
      identifier
      differentiationDeclarationOperator
      aiDifferentiationExpression
      SEMICOLON
    ;


/**
 * Differentiation declaration operator.
 *
 * The canonical assignment token is reused.
 */
differentiationDeclarationOperator
    : ASSIGN
    ;


/**
 * Differentiation annotation.
 *
 * The semantic layer MUST validate the normalized annotation as
 * `@differentiate`.
 */
differentiationAnnotation
    : NANO_ANNOTATION
    ;


/* ============================================================================
 * 3. DIFFERENTIATION EXPRESSION
 * ========================================================================== */

/**
 * General differentiation expression.
 *
 * Supported forms intentionally overlap semantically rather than introducing
 * a separate mathematical expression language.
 */
aiDifferentiationExpression
    : derivativeOperator
      LPAREN
      differentiationSubject
      differentiationVariableClause?
      differentiationOrderClause?
      differentiationModeClause?
      differentiationOptionsClause?
      RPAREN
    ;


/**
 * The differentiation subject is an ordinary Zamani expression.
 */
differentiationSubject
    : expression
    ;


/**
 * Explicit derivative variable selection.
 *
 * Examples:
 *
 *     @gradient(f, x)
 *     @jacobian(f, x, y)
 *
 * The number of variables is unbounded by grammar.
 */
differentiationVariableClause
    : differentiationVariableList
    ;


differentiationVariableList
    : COMMA
      differentiationVariable
      (COMMA differentiationVariable)*
    ;


differentiationVariable
    : expression
    ;


/* ============================================================================
 * 4. DERIVATIVE OPERATORS
 * ========================================================================== */

/**
 * Differentiation operation annotations.
 *
 * The token is generic because NANO_ANNOTATION contains the full annotation.
 *
 * Semantic analysis validates the spelling:
 *
 *     @derivative
 *     @differentiate
 *     @gradient
 *     @jacobian
 *     @hessian
 *
 * and may permit registered dialect-specific derivative operators.
 */
derivativeOperator
    : NANO_ANNOTATION
    ;


/* ============================================================================
 * 5. ORDER
 * ========================================================================== */

/**
 * Explicit differentiation order.
 *
 * Example:
 *
 *     @derivative(f, x, order = 2)
 *
 * The order is an expression rather than a grammar-level integer literal,
 * allowing symbolic or compile-time-resolved order when the semantic system
 * permits it.
 */
differentiationOrderClause
    : COMMA
      orderLabel
      ASSIGN
      expression
    ;


orderLabel
    : NANO_ANNOTATION
    | identifier
    ;


/* ============================================================================
 * 6. MODE / STRATEGY
 * ========================================================================== */

/**
 * Requested differentiation strategy.
 *
 * Example:
 *
 *     mode = reverse
 *     mode = forward
 *     mode = symbolic
 *     mode = numerical
 *     mode = automatic
 *     mode = parameter_shift
 *
 * Strategy names are identifiers and therefore extensible.
 */
differentiationModeClause
    : COMMA
      modeLabel
      ASSIGN
      differentiationMode
    ;


modeLabel
    : NANO_ANNOTATION
    | identifier
    ;


differentiationMode
    : identifier
    | qualifiedDifferentiationName
    ;


qualifiedDifferentiationName
    : identifier
      DOUBLE_COLON
      identifier
      (DOUBLE_COLON identifier)*
    ;


/* ============================================================================
 * 7. DIFFERENTIATION OPTIONS
 * ========================================================================== */

/**
 * Extensible option list.
 *
 * This intentionally accepts generic named values rather than creating a
 * fixed list of implementation-specific options.
 */
differentiationOptionsClause
    : COMMA
      optionsLabel
      ASSIGN
      differentiationOptionValue
      (COMMA differentiationOptionValue)*
    ;


optionsLabel
    : NANO_ANNOTATION
    | identifier
    ;


differentiationOptionValue
    : expression
    ;


/* ============================================================================
 * 8. RESULT-KIND BOUNDARIES
 * ========================================================================== */

/**
 * Explicit derivative-result declaration.
 *
 * This is useful where the source program wants to distinguish the desired
 * mathematical object without prescribing implementation.
 *
 * Example:
 *
 *     @gradient(...)
 *     @jacobian(...)
 *     @hessian(...)
 *
 * The derivative operator itself establishes the semantic result kind.
 *
 * This auxiliary rule exists as a stable semantic boundary for future
 * extensions.
 */
aiDifferentiationResultKind
    : gradientResultKind
    | jacobianResultKind
    | hessianResultKind
    | derivativeResultKind
    ;


gradientResultKind
    : NANO_ANNOTATION
    ;


jacobianResultKind
    : NANO_ANNOTATION
    ;


hessianResultKind
    : NANO_ANNOTATION
    ;


derivativeResultKind
    : NANO_ANNOTATION
    ;


/* ============================================================================
 * 9. DIFFERENTIATION REGION
 * ========================================================================== */

/**
 * A differentiation region contains ordinary Zamani statements.
 *
 * Example:
 *
 *     @differentiate {
 *         ...
 *     }
 *
 * The region does not establish a separate statement language.
 */
aiDifferentiationRegion
    : differentiationRegionAnnotation
      LBRACE
      differentiationStatement*
      RBRACE
    ;


differentiationRegionAnnotation
    : NANO_ANNOTATION
    ;


differentiationStatement
    : statement
    ;


/* ============================================================================
 * 10. NAMED DIFFERENTIATION FUNCTION
 * ========================================================================== */

/**
 * Named differentiation function declaration.
 *
 * This allows differentiation to be treated as a reusable semantic operation.
 *
 * Example:
 *
 *     @differentiate_function gradient_loss(x) {
 *         ...
 *     }
 *
 * The exact annotation spelling is validated semantically.
 */
aiDifferentiationFunction
    : differentiationFunctionAnnotation
      identifier
      differentiationParameterList?
      differentiationReturnClause?
      LBRACE
      differentiationStatement*
      RBRACE
    ;


differentiationFunctionAnnotation
    : NANO_ANNOTATION
    ;


differentiationParameterList
    : LPAREN
      differentiationParameter*
      RPAREN
    ;


differentiationParameter
    : identifier
      differentiationParameterType?
      differentiationParameterInitializer?
      COMMA?
    ;


differentiationParameterType
    : COLON
      typeExpression
    ;


differentiationParameterInitializer
    : ASSIGN
      expression
    ;


differentiationReturnClause
    : THIN_ARROW
      typeExpression
    ;


/* ============================================================================
 * 11. DIFFERENTIATION REQUIREMENTS
 * ========================================================================== */

/**
 * Semantic requirement boundary.
 *
 * Examples may express:
 *
 *     differentiability requirements;
 *     precision requirements;
 *     symbolic support;
 *     numerical support;
 *     quantum-gradient support;
 *     capability requirements.
 *
 * This rule does not decide whether the requirement can be satisfied.
 */
aiDifferentiationRequirement
    : differentiationRequirementAnnotation
      differentiationRequirementBody
    ;


differentiationRequirementAnnotation
    : NANO_ANNOTATION
    ;


differentiationRequirementBody
    : expression
    | LBRACE
      differentiationRequirementItem*
      RBRACE
    ;


differentiationRequirementItem
    : expression
      SEMICOLON?
    ;


/* ============================================================================
 * 12. CAPABILITY REQUEST
 * ========================================================================== */

/**
 * Capability request.
 *
 * The capability name is not hard-coded.
 *
 * Examples:
 *
 *     automatic differentiation
 *     symbolic differentiation
 *     quantum differentiation
 *     higher-order differentiation
 *
 * are semantic capabilities rather than machine identities.
 */
aiDifferentiationCapability
    : differentiationCapabilityAnnotation
      differentiationCapabilityValue
    ;


differentiationCapabilityAnnotation
    : NANO_ANNOTATION
    ;


differentiationCapabilityValue
    : expression
    ;


/* ============================================================================
 * 13. CONSTRAINT
 * ========================================================================== */

/**
 * Differentiation constraint.
 *
 * Constraints describe what must hold, not how the compiler achieves it.
 */
aiDifferentiationConstraint
    : differentiationConstraintAnnotation
      expression
      SEMICOLON?
    ;


differentiationConstraintAnnotation
    : NANO_ANNOTATION
    ;


/* ============================================================================
 * 14. PREFERENCE
 * ========================================================================== */

/**
 * Differentiation preference.
 *
 * A preference MUST NOT be interpreted as an unconditional implementation
 * requirement.
 */
aiDifferentiationPreference
    : differentiationPreferenceAnnotation
      expression
      SEMICOLON?
    ;


differentiationPreferenceAnnotation
    : NANO_ANNOTATION
    ;


/* ============================================================================
 * 15. RESOURCE REQUIREMENT
 * ========================================================================== */

/**
 * Resource requirement.
 *
 * This is intentionally expressed as a generic expression so that resource
 * quantities may be dynamic, symbolic, negotiated, or target-dependent.
 */
aiDifferentiationResource
    : differentiationResourceAnnotation
      expression
      SEMICOLON?
    ;


differentiationResourceAnnotation
    : NANO_ANNOTATION
    ;


/* ============================================================================
 * 16. DIFFERENTIATION COMPOSITION
 * ========================================================================== */

/**
 * Composition of multiple differentiation requests.
 *
 * Examples include:
 *
 *     derivative of derivative;
 *     gradient of a model transformation;
 *     Jacobian of a vector-valued function;
 *     Hessian of an objective;
 *     derivative through a differentiable pipeline.
 *
 * No fixed nesting depth is encoded.
 */
aiDifferentiationComposition
    : differentiationCompositionOperator
      LPAREN
      aiDifferentiationExpression
      COMMA
      aiDifferentiationExpression
      (COMMA aiDifferentiationExpression)*
      RPAREN
    ;


differentiationCompositionOperator
    : NANO_ANNOTATION
    | identifier
    ;


/* ============================================================================
 * 17. HIGHER-ORDER DIFFERENTIATION
 * ========================================================================== */

/**
 * Higher-order differentiation is represented structurally.
 *
 * Example:
 *
 *     @derivative(
 *         @derivative(f, x),
 *         x
 *     )
 *
 * No maximum order is encoded.
 */
aiHigherOrderDifferentiation
    : aiDifferentiationExpression
    ;


/* ============================================================================
 * 18. SYMBOLIC DIFFERENTIATION
 * ========================================================================== */

/**
 * Symbolic differentiation boundary.
 *
 * The grammar does not perform symbolic algebra.
 */
aiSymbolicDifferentiation
    : symbolicDifferentiationAnnotation
      LPAREN
      differentiationSubject
      differentiationVariableClause?
      differentiationOrderClause?
      RPAREN
    ;


symbolicDifferentiationAnnotation
    : NANO_ANNOTATION
    ;


/* ============================================================================
 * 19. NUMERICAL DIFFERENTIATION
 * ========================================================================== */

/**
 * Numerical differentiation boundary.
 *
 * Step size, approximation order, stability policy, and numerical method are
 * expressions/semantic configuration rather than hard-coded machine facts.
 */
aiNumericalDifferentiation
    : numericalDifferentiationAnnotation
      LPAREN
      differentiationSubject
      differentiationVariableClause?
      differentiationOrderClause?
      differentiationNumericalOptions?
      RPAREN
    ;


numericalDifferentiationAnnotation
    : NANO_ANNOTATION
    ;


differentiationNumericalOptions
    : COMMA
      differentiationNumericalOption
      (COMMA differentiationNumericalOption)*
    ;


differentiationNumericalOption
    : identifier
      ASSIGN
      expression
    ;


/* ============================================================================
 * 20. AUTOMATIC DIFFERENTIATION
 * ========================================================================== */

/**
 * Automatic differentiation boundary.
 *
 * This syntax expresses intent only.
 *
 * The compiler may later choose:
 *
 *     forward mode;
 *     reverse mode;
 *     mixed mode;
 *     checkpointed mode;
 *     distributed mode;
 *     accelerator implementation;
 *     another valid realization.
 */
aiAutomaticDifferentiation
    : automaticDifferentiationAnnotation
      LPAREN
      differentiationSubject
      differentiationVariableClause?
      differentiationModeClause?
      differentiationOptionsClause?
      RPAREN
    ;


automaticDifferentiationAnnotation
    : NANO_ANNOTATION
    ;


/* ============================================================================
 * 21. QUANTUM DIFFERENTIATION
 * ========================================================================== */

/**
 * Quantum differentiation boundary.
 *
 * This does NOT define parameter-shift or adjoint algorithms.
 *
 * Those belong to quantum semantic/lowering infrastructure.
 */
aiQuantumDifferentiation
    : quantumDifferentiationAnnotation
      LPAREN
      differentiationSubject
      differentiationVariableClause?
      differentiationModeClause?
      differentiationOptionsClause?
      RPAREN
    ;


quantumDifferentiationAnnotation
    : NANO_ANNOTATION
    ;


/* ============================================================================
 * 22. DIFFERENTIATION METADATA
 * ========================================================================== */

/**
 * Metadata attached to a differentiation request.
 *
 * Metadata has no execution semantics unless semantic analysis explicitly
 * assigns them.
 */
aiDifferentiationMetadata
    : metadataAnnotation
      LBRACE
      differentiationMetadataEntry*
      RBRACE
    ;


metadataAnnotation
    : NANO_ANNOTATION
    ;


differentiationMetadataEntry
    : identifier
      ASSIGN
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * 23. DIFFERENTIATION INVOCATION
 * ========================================================================== */

/**
 * Reuse an existing differentiation value/function.
 */
aiDifferentiationInvocation
    : identifier
      LPAREN
      optionalExpressionList
      RPAREN
    ;


/* ============================================================================
 * 24. DIFFERENTIATION TYPE BOUNDARY
 * ========================================================================== */

/**
 * Optional source-level differentiation type boundary.
 *
 * The actual type expression remains owned by the canonical type grammar.
 */
aiDifferentiationType
    : typeExpression
    ;


/* ============================================================================
 * 25. DIFFERENTIATION VALUE BOUNDARY
 * ========================================================================== */

/**
 * Differentiation results are ordinary Zamani values.
 */
aiDifferentiationValue
    : expression
    ;


/* ============================================================================
 * 26. DIFFERENTIATION STATEMENT
 * ========================================================================== */

/**
 * Differentiation can appear as a statement-level semantic operation.
 */
aiDifferentiationStatement
    : aiDifferentiationExpression
      SEMICOLON?
    ;


/* ============================================================================
 * 27. EXTENSIBLE STRATEGY REFERENCE
 * ========================================================================== */

/**
 * Strategy references may be namespaced.
 *
 * Example:
 *
 *     standard::reverse
 *     quantum::parameter_shift
 *     custom::my_method
 *
 * Namespace meaning is semantic.
 */
differentiationStrategyReference
    : identifier
    | qualifiedDifferentiationName
    ;


/* ============================================================================
 * 28. EXTENSIBLE METHOD DECLARATION
 * ========================================================================== */

/**
 * User/domain-defined differentiation method reference.
 *
 * The grammar does not require Zamani itself to know every possible future
 * differentiation algorithm.
 */
aiDifferentiationMethod
    : methodAnnotation
      identifier
      differentiationMethodBody?
    ;


methodAnnotation
    : NANO_ANNOTATION
    ;


differentiationMethodBody
    : LBRACE
      differentiationMethodMember*
      RBRACE
    ;


differentiationMethodMember
    : expression
      SEMICOLON?
    ;


/* ============================================================================
 * 29. DIFFERENTIATION CONTRACT
 * ========================================================================== */

/**
 * Differentiation contract boundary.
 *
 * A contract can describe semantic properties such as:
 *
 *     differentiable;
 *     continuously differentiable;
 *     differentiable with respect to selected parameters;
 *     derivative availability;
 *     derivative precision;
 *     derivative domain.
 *
 * The actual mathematical validation belongs to semantic analysis.
 */
aiDifferentiationContract
    : contractAnnotation
      LBRACE
      differentiationContractItem*
      RBRACE
    ;


contractAnnotation
    : NANO_ANNOTATION
    ;


differentiationContractItem
    : expression
      SEMICOLON?
    ;


/* ============================================================================
 * 30. DIFFERENTIATION TARGET SELECTION
 * ========================================================================== */

/**
 * Select a semantic differentiation target.
 *
 * This is NOT a hardware target selector.
 *
 * The target is an expression/name representing a differentiable value,
 * function, parameter set, model component, or other semantic object.
 */
aiDifferentiationTarget
    : targetAnnotation
      expression
    ;


targetAnnotation
    : NANO_ANNOTATION
    ;


/* ============================================================================
 * 31. DIFFERENTIATION VARIABLE DECLARATION
 * ========================================================================== */

/**
 * Named differentiation variable.
 */
aiDifferentiationVariable
    : variableAnnotation
      identifier
      differentiationVariableType?
    ;


variableAnnotation
    : NANO_ANNOTATION
    ;


differentiationVariableType
    : COLON
      typeExpression
    ;


/* ============================================================================
 * 32. DIFFERENTIATION OUTPUT DECLARATION
 * ========================================================================== */

/**
 * Named derivative output.
 */
aiDifferentiationOutput
    : outputAnnotation
      identifier
      differentiationOutputType?
    ;


outputAnnotation
    : NANO_ANNOTATION
    ;


differentiationOutputType
    : COLON
      typeExpression
    ;


/* ============================================================================
 * 33. DIFFERENTIATION PIPELINE
 * ========================================================================== */

/**
 * Differentiation may be composed into a semantic pipeline.
 *
 * Each stage remains an expression or differentiation operation.
 *
 * No fixed stage count is imposed.
 */
aiDifferentiationPipeline
    : pipelineAnnotation
      LBRACE
      differentiationPipelineStage*
      RBRACE
    ;


pipelineAnnotation
    : NANO_ANNOTATION
    ;


differentiationPipelineStage
    : aiDifferentiationExpression
      SEMICOLON?
    | expression
      SEMICOLON?
    ;


/* ============================================================================
 * 34. DIFFERENTIATION ERROR / FAILURE DECLARATION
 * ========================================================================== */

/**
 * Source-level semantic failure handling boundary.
 *
 * This does not implement runtime error handling.
 */
aiDifferentiationFailurePolicy
    : failureAnnotation
      expression
    ;


failureAnnotation
    : NANO_ANNOTATION
    ;


/* ============================================================================
 * 35. GENERIC DIFFERENTIATION ATTRIBUTE
 * ========================================================================== */

/**
 * Future-proof extension point for semantic differentiation attributes.
 *
 * Unknown attributes are syntax-valid and must be validated by semantic
 * analysis or dialect registration.
 */
aiDifferentiationAttribute
    : NANO_ANNOTATION
      (expression)?
    ;


/* ============================================================================
 * 36. IDENTIFIER BOUNDARY
 * ========================================================================== */

/**
 * Canonical identifier spelling is owned here only as a parser boundary.
 *
 * The lexical spelling remains owned by ZamaniLexer.
 */
identifier
    : IDENT
    ;