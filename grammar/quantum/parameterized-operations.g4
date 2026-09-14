/*
 * ============================================================================
 * Zamani Universal Programming Language
 * grammar/quantum/parameterized-operations.g4
 * ============================================================================
 *
 * PURPOSE
 * -------
 * Canonical reusable parser fragment for parameterized quantum-operation
 * syntax.
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 * safe Rust only
 * no unsafe Rust
 *
 * ============================================================================
 * ARCHITECTURAL OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - quantum operation parameter declarations;
 *   - quantum operation parameter references;
 *   - quantum operation parameter binding syntax;
 *   - positional parameter bindings;
 *   - named parameter bindings;
 *   - parameter defaults;
 *   - parameter lists;
 *   - parameter argument lists;
 *   - parameter argument expressions;
 *   - parameter packs where explicitly supported by the language;
 *   - parameter-binding ordering syntax;
 *   - parameter-binding source structure;
 *   - parameter-related syntactic attributes;
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - general expressions;
 *   - general types;
 *   - generic type arguments;
 *   - gate definitions;
 *   - gate names;
 *   - generic operation invocation;
 *   - operation targets;
 *   - qubits;
 *   - quantum registers;
 *   - controlled operations;
 *   - measurement;
 *   - reset;
 *   - observables;
 *   - dynamic-circuit control;
 *   - QEC;
 *   - ZQN;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - hardware;
 *   - physical qubits;
 *   - calibration;
 *   - pulse definitions;
 *   - runtime parameter storage;
 *   - parameter optimization algorithms;
 *   - parameter-shift algorithms;
 *   - gradient computation;
 *   - canonical quantum IR implementation.
 *
 * ============================================================================
 * CANONICAL SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Source
 *   |
 *   v
 * Canonical lexer
 *   |
 *   v
 * Canonical parser
 *   |
 *   v
 * Frontend AST
 *   |
 *   v
 * Name/type/effect/capability/resource analysis
 *   |
 *   v
 * Canonical semantic representation
 *   |
 *   +---- quantum::ir::core::parameter
 *   |
 *   +---- quantum::ir
 *   |
 *   +---- optimization
 *   +---- routing
 *   +---- scheduling
 *   +---- ZQN
 *   +---- QEC
 *   +---- resilience
 *   +---- hardware HAL
 *   |
 *   v
 * Target lowering
 *   |
 *   v
 * Execution
 *
 * This grammar MUST NOT construct or define a second Parameter,
 * ParameterExpression, GateParameter, or quantum IR representation.
 *
 * ============================================================================
 * PARAMETER MODEL
 * ============================================================================
 *
 * The canonical semantic parameter model already supports:
 *
 *   - concrete scalar values;
 *   - symbolic parameters;
 *   - parameter expressions;
 *   - deterministic binding;
 *   - partial binding;
 *   - expression inspection;
 *   - constant folding;
 *   - deterministic symbol collection.
 *
 * This file only describes their SOURCE SYNTAX.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Parameter syntax MUST NOT encode:
 *
 *   - number of qubits;
 *   - number of processors;
 *   - hardware topology;
 *   - native gate sets;
 *   - device identifiers;
 *   - pulse channels;
 *   - calibration values;
 *   - hardware parameter limits;
 *   - simulator dimensions;
 *   - backend-specific parameter counts;
 *   - fixed machine sizes.
 *
 * A parameter count appearing in a source operation is part of that operation's
 * semantic contract. It is NOT a machine-size restriction.
 *
 * Therefore:
 *
 *     RX(theta)
 *
 * is valid because RX semantically has one parameter, while:
 *
 *     U(theta, phi, lambda)
 *
 * has three semantic parameters.
 *
 * The grammar itself does not impose a global maximum.
 *
 * ============================================================================
 * IMPORTANT INTEGRATION RULE
 * ============================================================================
 *
 * The existing repository currently has parameter syntax duplicated in:
 *
 *     grammar/quantum/gates.g4
 *     grammar/quantum/operations.g4
 *     grammar/quantum/quantum.g4
 *
 * Production architecture must converge on THIS FILE as the owner.
 *
 * Those files should consume:
 *
 *     quantumOperationParameterList
 *     quantumOperationParameter
 *     quantumOperationArgumentList
 *     quantumOperationArgument
 *
 * rather than defining competing parameter grammars.
 *
 * ============================================================================
 */


/* ============================================================================
 * 1. PARAMETER DECLARATION LIST
 * ========================================================================== */

/*
 * Formal parameters of a quantum operation.
 *
 * Examples:
 *
 *     (theta)
 *
 *     (theta, phi, lambda)
 *
 *     (theta: Angle, phi: Angle)
 *
 *     (theta: Angle = pi / 2)
 *
 * No finite number of parameters is imposed.
 */
quantumOperationParameterList
    : LPAREN
      quantumOperationParameterDeclarations?
      RPAREN
    ;


/*
 * Comma-separated formal parameters.
 *
 * A trailing comma is accepted deliberately because it makes generated source
 * and source formatting easier without changing semantic meaning.
 */
quantumOperationParameterDeclarations
    : quantumOperationParameterDeclaration
      (COMMA quantumOperationParameterDeclaration)*
      COMMA?
    ;


/* ============================================================================
 * 2. SINGLE PARAMETER DECLARATION
 * ========================================================================== */

/*
 * A parameter declaration consists of:
 *
 *     name
 *
 * optionally:
 *
 *     : type
 *
 * optionally:
 *
 *     = default-expression
 *
 * Examples:
 *
 *     theta
 *
 *     theta: Angle
 *
 *     theta: Angle = pi / 2
 *
 * The grammar does not decide whether Angle is a valid semantic parameter type.
 */
quantumOperationParameterDeclaration
    : identifier
      quantumOperationParameterTypeAnnotation?
      quantumOperationParameterDefault?
    ;


quantumOperationParameterTypeAnnotation
    : COLON typeExpression
    ;


quantumOperationParameterDefault
    : ASSIGN expression
    ;


/* ============================================================================
 * 3. PARAMETER REFERENCE
 * ========================================================================== */

/*
 * A parameter reference is syntactically an ordinary expression.
 *
 * The semantic layer determines whether an identifier resolves to:
 *
 *     - a quantum operation parameter;
 *     - a local variable;
 *     - a constant;
 *     - a module member;
 *     - another valid expression source.
 *
 * Do NOT create a separate semantic parameter identifier type here.
 */
quantumOperationParameterReference
    : expression
    ;


/* ============================================================================
 * 4. PARAMETER ARGUMENT LIST
 * ========================================================================== */

/*
 * Actual parameter bindings supplied when invoking a quantum operation.
 *
 * Canonical forms:
 *
 *     RX(theta)
 *
 *     U(theta, phi, lambda)
 *
 *     U(theta = a, phi = b, lambda = c)
 *
 * Positional and named arguments are structurally distinguishable.
 *
 * Semantic analysis determines:
 *
 *     - whether the parameter exists;
 *     - whether a named parameter exists;
 *     - whether an argument is duplicated;
 *     - whether a required parameter is missing;
 *     - whether defaults apply;
 *     - whether types are compatible.
 */
quantumOperationArgumentList
    : LPAREN
      quantumOperationArguments?
      RPAREN
    ;


quantumOperationArguments
    : quantumOperationPositionalArguments
    | quantumOperationNamedArguments
    | quantumOperationMixedArguments
    ;


/* ============================================================================
 * 5. POSITIONAL ARGUMENTS
 * ========================================================================== */

/*
 * Examples:
 *
 *     RX(theta)
 *
 *     U(theta, phi, lambda)
 *
 * Positional ordering is preserved exactly.
 */
quantumOperationPositionalArguments
    : quantumOperationPositionalArgument
      (COMMA quantumOperationPositionalArgument)*
      COMMA?
    ;


quantumOperationPositionalArgument
    : expression
    ;


/* ============================================================================
 * 6. NAMED ARGUMENTS
 * ========================================================================== */

/*
 * Examples:
 *
 *     RX(theta = pi / 2)
 *
 *     U(theta = a, phi = b, lambda = c)
 *
 * Parameter names are source-level identifiers.
 *
 * Name resolution is semantic.
 */
quantumOperationNamedArguments
    : quantumOperationNamedArgument
      (COMMA quantumOperationNamedArgument)*
      COMMA?
    ;


quantumOperationNamedArgument
    : identifier
      ASSIGN
      expression
    ;


/* ============================================================================
 * 7. MIXED ARGUMENTS
 * ========================================================================== */

/*
 * Mixed form permits positional arguments before named arguments:
 *
 *     operation(a, b, theta = c, phi = d)
 *
 * Positional arguments AFTER a named argument are deliberately not accepted.
 *
 * This makes binding deterministic and avoids ambiguous positional assignment.
 *
 * Semantic validation still checks whether the operation declaration permits
 * the supplied combination.
 */
quantumOperationMixedArguments
    : quantumOperationPositionalArgument
      (COMMA quantumOperationPositionalArgument)*
      COMMA
      quantumOperationNamedArgument
      (COMMA quantumOperationNamedArgument)*
      COMMA?
    ;


/* ============================================================================
 * 8. UNIFIED PARAMETERIZED OPERATION ARGUMENT
 * ========================================================================== */

/*
 * Generic reusable argument rule for downstream grammar consumers.
 */
quantumOperationArgument
    : quantumOperationNamedArgument
    | quantumOperationPositionalArgument
    ;


/* ============================================================================
 * 9. PARAMETERIZED OPERATION REFERENCE
 * ========================================================================== */

/*
 * This rule is deliberately independent of operation invocation.
 *
 * It represents:
 *
 *     operation
 *
 * plus optional parameter bindings.
 *
 * The operation itself is resolved by operations.g4 / semantic analysis.
 */
parameterizedQuantumOperationReference
    : quantumOperationDesignatorReference
      quantumOperationArgumentList?
    ;


/*
 * Integration seam.
 *
 * operations.g4 owns operation designators and qualified operation names.
 *
 * This file MUST NOT redefine those rules.
 */
quantumOperationDesignatorReference
    : qualifiedName
    ;


/* ============================================================================
 * 10. PARAMETERIZED OPERATION INVOCATION
 * ========================================================================== */

/*
 * Complete parameterized invocation before target operands are attached.
 *
 * Examples:
 *
 *     RX(theta)
 *
 *     RX(theta + phi)
 *
 *     U(theta, phi, lambda)
 *
 *     library::rotation(angle = theta)
 *
 * The target list belongs to operations.g4.
 */
parameterizedQuantumOperationInvocation
    : quantumOperationDesignatorReference
      quantumOperationArgumentList
    ;


/* ============================================================================
 * 11. PARAMETER EXPRESSION
 * ========================================================================== */

/*
 * Parameters deliberately reuse the canonical Zamani expression grammar.
 *
 * This permits:
 *
 *     theta
 *
 *     theta + phi
 *
 *     2 * theta
 *
 *     theta / 2
 *
 *     sin(theta)
 *
 *     configuration.angle
 *
 *     compile_time_value
 *
 *     f(x)
 *
 * without creating a second mathematical expression grammar.
 */
quantumParameterExpression
    : expression
    ;


/* ============================================================================
 * 12. CONSTANT PARAMETER EXPRESSION
 * ========================================================================== */

/*
 * This is a syntactic category for consumers that require a parameter known
 * at compile time.
 *
 * IMPORTANT:
 *
 * The grammar does not attempt to prove constancy.
 *
 * Semantic analysis / compile-time evaluation determines whether an expression
 * is actually constant.
 *
 * This avoids coupling the grammar to constant-folding implementation.
 */
quantumCompileTimeParameterExpression
    : expression
    ;


/* ============================================================================
 * 13. SYMBOLIC PARAMETER EXPRESSION
 * ========================================================================== */

/*
 * Symbolic parameters are not restricted to a single identifier.
 *
 * The semantic parameter system may represent:
 *
 *     theta
 *
 *     theta + phi
 *
 *     2 * theta
 *
 *     sin(theta)
 *
 *     theta / (phi + 1)
 *
 * etc.
 *
 * This rule therefore intentionally delegates to expression.
 */
quantumSymbolicParameterExpression
    : expression
    ;


/* ============================================================================
 * 14. PARAMETER DEFAULTS
 * ========================================================================== */

/*
 * Reusable default-value syntax.
 *
 * Example:
 *
 *     theta: Angle = pi / 2
 */
quantumParameterDefaultValue
    : ASSIGN expression
    ;


/* ============================================================================
 * 15. PARAMETER CONSTRAINT SYNTAX
 * ========================================================================== */

/*
 * Parameter constraints are source-level declarations.
 *
 * Examples:
 *
 *     theta: Angle where theta >= 0
 *
 *     theta: Angle where theta < 2 * pi
 *
 * The semantic layer decides whether a constraint is satisfiable.
 *
 * This grammar does not execute or validate constraints.
 */
quantumParameterConstraint
    : K_WHERE expression
    ;


/*
 * Parameter declaration with an optional semantic constraint.
 *
 * This rule is available to operation declarations that want explicit
 * parameter-domain contracts.
 */
quantumConstrainedOperationParameterDeclaration
    : identifier
      quantumOperationParameterTypeAnnotation?
      quantumOperationParameterDefault?
      quantumParameterConstraint?
    ;


/* ============================================================================
 * 16. PARAMETER DECLARATION WITH ATTRIBUTES
 * ========================================================================== */

/*
 * Parameter attributes remain ordinary source metadata.
 *
 * Examples:
 *
 *     @trainable theta: Angle
 *
 *     @constant theta: Angle
 *
 *     @compile_time theta: Angle
 *
 * Semantic analysis determines the meaning of the attribute.
 *
 * This file does not define optimization/training semantics.
 */
quantumOperationParameterWithAttributes
    : attributes*
      quantumOperationParameterDeclaration
    ;


/* ============================================================================
 * 17. PARAMETER LIST WITH ATTRIBUTES
 * ========================================================================== */

quantumAttributedOperationParameterDeclarations
    : quantumOperationParameterWithAttributes
      (COMMA quantumOperationParameterWithAttributes)*
      COMMA?
    ;


/* ============================================================================
 * 18. CANONICAL FORMAL PARAMETER LIST
 * ========================================================================== */

/*
 * This is the preferred production rule for operation declarations.
 *
 * It is separated from the historical parameter rule so downstream grammar
 * consumers can migrate without changing the semantic model.
 */
canonicalQuantumOperationParameterList
    : LPAREN
      quantumAttributedOperationParameterDeclarations?
      RPAREN
    ;


/* ============================================================================
 * 19. CANONICAL ACTUAL PARAMETER LIST
 * ========================================================================== */

/*
 * Preferred production rule for operation invocation.
 */
canonicalQuantumOperationArgumentList
    : LPAREN
      quantumOperationArguments?
      RPAREN
    ;


/* ============================================================================
 * 20. PARAMETER BINDING
 * ========================================================================== */

/*
 * A binding is syntactic structure only.
 *
 * Examples:
 *
 *     theta
 *
 *     theta = pi / 2
 *
 *     phi = theta + delta
 */
quantumParameterBinding
    : quantumOperationPositionalArgument
    | quantumOperationNamedArgument
    ;


/* ============================================================================
 * 21. PARAMETER BINDING LIST
 * ========================================================================== */

/*
 * Generic binding list for semantic consumers.
 *
 * No finite binding count is imposed.
 */
quantumParameterBindingList
    : quantumParameterBinding
      (COMMA quantumParameterBinding)*
      COMMA?
    ;


/* ============================================================================
 * 22. PARAMETER PACK / EXPANSION
 * ========================================================================== */

/*
 * Parameter packs are useful for generic/future quantum operations where a
 * parameter sequence is forwarded without requiring a fixed source-level
 * arity.
 *
 * The ellipsis token is intentionally represented using the canonical lexical
 * token if available.
 *
 * Semantic analysis determines whether the referenced value is actually a
 * parameter pack.
 *
 * Example:
 *
 *     operation(...parameters)
 *
 * If the canonical lexer does not expose ELLIPSIS, the integration layer must
 * map this syntax to the canonical range/variadic punctuation representation.
 */
quantumParameterPackArgument
    : ELLIPSIS expression
    ;


/*
 * Pack-aware argument.
 */
quantumOperationPackArgument
    : quantumParameterPackArgument
    | quantumOperationPositionalArgument
    | quantumOperationNamedArgument
    ;


/* ============================================================================
 * 23. PACK-AWARE ARGUMENT LIST
 * ========================================================================== */

quantumOperationPackArgumentList
    : LPAREN
      quantumOperationPackArguments?
      RPAREN
    ;


quantumOperationPackArguments
    : quantumOperationPackArgument
      (COMMA quantumOperationPackArgument)*
      COMMA?
    ;


/* ============================================================================
 * 24. PARAMETER REBINDING
 * ========================================================================== */

/*
 * Explicit rebinding is useful for reusable parameterized operation values.
 *
 * Example:
 *
 *     bind operation theta = angle
 *
 * This rule is deliberately a syntax-level extension point. The surrounding
 * language decides whether `bind` is a declaration, expression, or statement.
 */
quantumParameterRebinding
    : K_BIND
      identifier
      ASSIGN
      expression
    ;


/* ============================================================================
 * 25. PARAMETER SPECIALIZATION
 * ========================================================================== */

/*
 * Parameter specialization expresses compile-time/source-level specialization
 * intent.
 *
 * It does NOT perform optimization.
 *
 * Example:
 *
 *     specialize operation(theta = pi / 2)
 */
quantumParameterSpecialization
    : K_SPECIALIZE
      quantumOperationDesignatorReference
      quantumOperationArgumentList?
    ;


/* ============================================================================
 * 26. PARAMETER SUBSTITUTION
 * ========================================================================== */

/*
 * Explicit symbolic substitution syntax.
 *
 * Example:
 *
 *     substitute theta = phi + delta
 *
 * The actual substitution algorithm belongs to quantum::ir parameter
 * semantics, not the grammar.
 */
quantumParameterSubstitution
    : K_SUBSTITUTE
      quantumOperationParameterReference
      ASSIGN
      expression
    ;


/* ============================================================================
 * 27. PARAMETER SET
 * ========================================================================== */

/*
 * A parameter set is an ordered collection of bindings.
 *
 * Ordering is retained because positional parameters are semantically ordered.
 */
quantumParameterSet
    : LBRACE
      quantumParameterBindingList?
      RBRACE
    ;


/* ============================================================================
 * 28. PARAMETER SWEEP / COLLECTION
 * ========================================================================== */

/*
 * Parameter sweeps are represented syntactically as expressions.
 *
 * The grammar does not impose:
 *
 *     number of sweep points;
 *     maximum parameter values;
 *     maximum dimensionality;
 *     backend-specific limits.
 *
 * Examples:
 *
 *     theta in range(...)
 *
 *     theta in values
 *
 *     (theta, phi) in parameter_grid
 */
quantumParameterSweep
    : identifier
      K_IN
      expression
    ;


/* ============================================================================
 * 29. PARAMETER SWEEP LIST
 * ========================================================================== */

quantumParameterSweepList
    : quantumParameterSweep
      (COMMA quantumParameterSweep)*
      COMMA?
    ;


/* ============================================================================
 * 30. PARAMETERIZED OPERATION SWEEP
 * ========================================================================== */

/*
 * Source-level parameter sweep intent.
 *
 * This does NOT execute a batch.
 *
 * Runtime execution, batching, vectorization, parameter binding strategy and
 * hardware execution remain downstream concerns.
 */
quantumParameterizedOperationSweep
    : K_SWEEP
      quantumOperationDesignatorReference
      quantumOperationArgumentList?
      K_OVER
      LBRACE
      quantumParameterSweepList?
      RBRACE
    ;


/* ============================================================================
 * 31. PARAMETER GRADIENT INTENT
 * ========================================================================== */

/*
 * Parameter-gradient syntax is intentionally only an intent marker.
 *
 * It does not define:
 *
 *     parameter-shift;
 *     finite differences;
 *     adjoint differentiation;
 *     automatic differentiation;
 *     hardware gradient execution.
 *
 * Those belong to semantic/compiler/runtime systems.
 */
quantumParameterGradientRequest
    : K_GRADIENT
      quantumOperationParameterReference
    ;


/* ============================================================================
 * 32. PARAMETER GROUP
 * ========================================================================== */

/*
 * Groups permit explicit source-level grouping without introducing a second
 * expression grammar.
 */
quantumParameterGroup
    : LPAREN
      expression
      RPAREN
    ;


/* ============================================================================
 * 33. PARAMETER ARRAY / COLLECTION EXPRESSION
 * ========================================================================== */

/*
 * Collection syntax delegates to the canonical expression grammar.
 *
 * No fixed collection size is encoded.
 */
quantumParameterCollectionExpression
    : expression
    ;


/* ============================================================================
 * 34. PARAMETERIZED OPERATION BODY CONTRACT
 * ========================================================================== */

/*
 * Gate/operation declarations consume the formal parameter list and then
 * provide a normal operation body.
 *
 * The body is NOT owned here.
 *
 * This rule exists only as an integration contract.
 */
parameterizedQuantumOperationDeclarationSignature
    : quantumOperationDesignatorReference
      canonicalQuantumOperationParameterList?
    ;


/* ============================================================================
 * 35. PARAMETERIZED OPERATION APPLICATION CONTRACT
 * ========================================================================== */

/*
 * Operations.g4 should use this rule when it needs a parameterized operation.
 *
 * Example:
 *
 *     apply RX(theta)(q);
 *
 * The target list remains owned by operations.g4.
 */
parameterizedQuantumOperationApplication
    : quantumOperationDesignatorReference
      canonicalQuantumOperationArgumentList
    ;


/* ============================================================================
 * 36. PARAMETERIZED OPERATION + TARGET INTEGRATION CONTRACT
 * ========================================================================== */

/*
 * This rule is intentionally NOT the owner of target syntax.
 *
 * It is a structural seam for operations.g4.
 *
 * The target clause MUST be supplied by operations.g4.
 */
parameterizedQuantumOperationCore
    : quantumOperationDesignatorReference
      quantumOperationArgumentList?
    ;


/* ============================================================================
 * 37. PARAMETERIZED OPERATION METADATA
 * ========================================================================== */

/*
 * Parameter metadata remains syntactic metadata.
 *
 * Examples:
 *
 *     @trainable
 *     @constant
 *     @compile_time
 *     @runtime
 *
 * Semantic analysis determines whether the metadata is valid.
 */
quantumParameterMetadata
    : attributes*
    ;


/* ============================================================================
 * 38. PARAMETERIZED OPERATION CONTRACT
 * ========================================================================== */

/*
 * Complete reusable source-level parameterized operation contract.
 *
 * This rule intentionally stops before targets.
 *
 * Therefore:
 *
 *     parameterized operations
 *
 * remain independent from:
 *
 *     qubits
 *     registers
 *     physical placement
 *     routing
 *     scheduling
 */
quantumParameterizedOperationContract
    : quantumParameterMetadata
      quantumOperationDesignatorReference
      quantumOperationArgumentList
    ;


/* ============================================================================
 * 39. PARAMETER TYPE CONTRACT
 * ========================================================================== */

/*
 * Quantum parameter types are ordinary Zamani types.
 *
 * Examples may include:
 *
 *     Angle
 *     Float
 *     Real
 *     Complex
 *     Integer
 *     Tensor<...>
 *     custom parameter types
 *
 * This grammar does not enumerate those types.
 */
quantumParameterType
    : typeExpression
    ;


/* ============================================================================
 * 40. PARAMETER VALIDATION BOUNDARY
 * ========================================================================== */

/*
 * The following are deliberately NOT parser errors:
 *
 *     wrong parameter count
 *     duplicate named parameter
 *     missing required parameter
 *     unknown parameter name
 *     invalid parameter type
 *     invalid parameter domain
 *     unbound symbolic parameter
 *     incompatible parameter expression
 *     invalid unit
 *     non-finite runtime value
 *
 * They are semantic/IR validation responsibilities.
 *
 * This separation is essential because parameter meaning can depend on:
 *
 *     operation declaration
 *     generic specialization
 *     dialect
 *     type system
 *     compile-time evaluation
 *     target-independent semantic analysis
 */


/* ============================================================================
 * 41. SCALABILITY CONTRACT
 * ========================================================================== */

/*
 * No grammar rule in this file imposes a fixed maximum for:
 *
 *     parameters
 *     arguments
 *     symbols
 *     expression size
 *     operation count
 *     operation nesting
 *     sweep points
 *     parameter collections
 *
 * Any implementation/security limits must be supplied externally as explicit
 * parser/compiler policies.
 *
 * Such limits MUST NOT be confused with language semantics.
 */


/* ============================================================================
 * 42. DETERMINISM CONTRACT
 * ========================================================================== */

/*
 * Parameter argument order MUST be preserved by the parser.
 *
 * The grammar does not sort:
 *
 *     positional arguments
 *
 * or:
 *
 *     named arguments.
 *
 * Named arguments are resolved deterministically by semantic analysis.
 *
 * No hidden global parameter environment exists at grammar level.
 */


/* ============================================================================
 * 43. ERROR-RECOVERY CONTRACT
 * ========================================================================== */

/*
 * The parser implementation must recover from malformed parameter lists
 * without silently converting malformed syntax into a different operation.
 *
 * Examples that must generate structured diagnostics:
 *
 *     RX(;
 *
 *     RX(theta,;
 *
 *     RX(theta = );
 *
 *     RX(theta = a, b,);
 *
 *     RX(theta = a, theta = b);
 *
 *     RX(unknown = value);
 *
 * The first group is syntactic recovery.
 *
 * The latter semantic cases must survive parsing and be diagnosed later.
 */


/* ============================================================================
 * 44. AST CONTRACT
 * ========================================================================== */

/*
 * The frontend AST must preserve enough information to distinguish:
 *
 *     positional argument
 *
 * from:
 *
 *     named argument
 *
 * and preserve:
 *
 *     source order
 *     source span
 *     expression
 *     optional declared type
 *     optional default expression
 *     optional metadata
 *
 * The grammar must not lower these structures directly into:
 *
 *     quantum::ir::Parameter
 *
 * because parsing and semantic lowering are separate stages.
 */


/* ============================================================================
 * 45. QUANTUM IR CONTRACT
 * ========================================================================== */

/*
 * Semantic lowering must map parameter syntax to the canonical quantum IR
 * parameter system.
 *
 * The canonical downstream concepts include:
 *
 *     Parameter
 *     ParameterExpression
 *     GateParameter
 *     BoundGateParameter
 *
 * No frontend-local parameter representation may become a competing canonical
 * quantum parameter model.
 */


/* ============================================================================
 * 46. GATES.G4 INTEGRATION
 * ========================================================================== */

/*
 * gates.g4 currently defines:
 *
 *     gateParameterList
 *     gateParameter
 *
 * Those rules must be migrated to this canonical parameter grammar.
 *
 * Production replacement:
 *
 *     gateParameterList
 *         -> quantumOperationArgumentList
 *
 * or, where formal declarations are required:
 *
 *     canonicalQuantumOperationParameterList
 *
 * Gate syntax remains owned by gates.g4.
 *
 * Parameter syntax is owned here.
 */


/* ============================================================================
 * 47. OPERATIONS.G4 INTEGRATION
 * ========================================================================== */

/*
 * operations.g4 currently defines:
 *
 *     quantumOperationArguments
 *
 * and:
 *
 *     quantumOperationTargetClause
 *
 * Parameterized-operation responsibility must be split:
 *
 * THIS FILE:
 *
 *     quantumOperationArgumentList
 *
 *     quantumOperationArgument
 *
 *     quantumOperationParameterDeclaration
 *
 *     quantumParameterBinding
 *
 * operations.g4:
 *
 *     operation designator
 *     operation target clause
 *     operation statement
 *
 * Canonical resulting syntax:
 *
 *     apply RX(theta)(q);
 *
 *     apply U(theta, phi, lambda)(q0, q1);
 *
 *     apply library::operation(angle = theta)(q);
 */


/* ============================================================================
 * 48. CONTROLLED-OPERATIONS.G4 INTEGRATION
 * ========================================================================== */

/*
 * controlled-operations.g4 may wrap a parameterized operation:
 *
 *     controlled(RX(theta))
 *
 * or:
 *
 *     control(RX(theta))(control, target)
 *
 * depending on the canonical controlled-operation syntax.
 *
 * It MUST consume:
 *
 *     quantumOperationArgumentList
 *
 * rather than defining another parameter grammar.
 *
 * Controlled-operation semantics remain owned by controlled-operations.g4.
 */


/* ============================================================================
 * 49. PARAMETERIZED OPERATIONS AND MEASUREMENT
 * ========================================================================== */

/*
 * Measurement does not use this parameter grammar merely because measurement
 * may have configuration expressions.
 *
 * Measurement parameter semantics remain owned by measurement.g4.
 *
 * This prevents:
 *
 *     measurement parameters
 *
 * from being confused with:
 *
 *     operation parameters.
 */


/* ============================================================================
 * 50. PARAMETERIZED OPERATIONS AND OBSERVABLES
 * ========================================================================== */

/*
 * Observables may themselves contain parameter expressions, but observable
 * syntax remains owned by observables.g4.
 *
 * observables.g4 should consume canonical expression rules rather than
 * duplicating this file's operation parameter grammar unless an observable is
 * explicitly an operation invocation.
 */


/* ============================================================================
 * 51. PARAMETERIZED OPERATIONS AND DYNAMIC CIRCUITS
 * ========================================================================== */

/*
 * A parameter expression may depend on values permitted by the language's
 * execution model.
 *
 * This file does not decide whether a parameter is:
 *
 *     compile-time
 *     runtime
 *     measurement-derived
 *     distributed
 *     adaptive
 *
 * That determination belongs to semantic/effect analysis and the dynamic
 * circuit subsystem.
 */


/* ============================================================================
 * 52. PARAMETERIZED OPERATIONS AND HARDWARE
 * ========================================================================== */

/*
 * Hardware parameters must not be silently confused with source operation
 * parameters.
 *
 * For example:
 *
 *     RX(theta)
 *
 * describes a logical operation parameter.
 *
 * It does NOT mean:
 *
 *     pulse_amplitude = theta
 *
 * or:
 *
 *     device_parameter = theta
 *
 * unless a later target-lowering stage explicitly defines such a mapping.
 */


/* ============================================================================
 * 53. PARAMETERIZED OPERATIONS AND OPTIMIZATION
 * ========================================================================== */

/*
 * Optimization may:
 *
 *     fold constants;
 *     simplify expressions;
 *     bind symbols;
 *     specialize operations;
 *     eliminate redundant parameters.
 *
 * None of those transformations occur in this grammar.
 */


/* ============================================================================
 * 54. PARAMETERIZED OPERATIONS AND DIFFERENTIATION
 * ========================================================================== */

/*
 * The presence of:
 *
 *     quantumParameterGradientRequest
 *
 * does not define a differentiation algorithm.
 *
 * Implementations may later choose:
 *
 *     parameter shift
 *     adjoint differentiation
 *     analytic differentiation
 *     automatic differentiation
 *     finite differences
 *     symbolic differentiation
 *
 * without changing this syntax.
 */


/* ============================================================================
 * 55. PARAMETERIZED OPERATIONS AND QEC
 * ========================================================================== */

/*
 * QEC may consume parameterized operations after semantic lowering.
 *
 * This grammar does not define:
 *
 *     code distance
 *     syndrome parameters
 *     decoder parameters
 *     logical error thresholds
 *     correction strategies.
 *
 * Those remain QEC responsibilities.
 */


/* ============================================================================
 * 56. PARAMETERIZED OPERATIONS AND ZQN
 * ========================================================================== */

/*
 * ZQN may describe parameter-dependent noise or fault behavior downstream.
 *
 * This grammar does not define:
 *
 *     noise channels
 *     fault probabilities
 *     correlations
 *     leakage
 *     loss
 *     calibration
 *
 * Those remain ZQN responsibilities.
 */


/* ============================================================================
 * 57. PARAMETERIZED OPERATIONS AND RESILIENCE
 * ========================================================================== */

/*
 * Resilience may decide to:
 *
 *     retry
 *     recompile
 *     reroute
 *     reschedule
 *     change mitigation
 *     change QEC
 *
 * based on runtime state.
 *
 * Parameterized-operation syntax must remain independent of those decisions.
 */


/* ============================================================================
 * 58. COMPATIBILITY
 * ========================================================================== */

/*
 * Existing valid forms MUST remain representable:
 *
 *     apply RX(theta)(q);
 *
 *     apply U(theta, phi, lambda)(q);
 *
 *     apply library::operation(angle)(q);
 *
 *     apply operation<T>(theta)(q);
 *
 * The migration must not silently change their semantic interpretation.
 *
 * Legacy aliases should be handled by compatibility/migration rules rather
 * than by creating permanently duplicated parameter grammars.
 */


/* ============================================================================
 * 59. NEGATIVE TEST CONTRACT
 * ========================================================================== */

/*
 * The following must be rejected syntactically:
 *
 *     RX(
 *
 *     RX(theta
 *
 *     RX(theta,)
 *         [only if trailing comma policy is disabled by canonical syntax]
 *
 *     RX(theta = )
 *
 *     RX(= theta)
 *
 *     RX(theta = a, = b)
 *
 *     RX(theta = a,,b)
 *
 * Semantic diagnostics must cover:
 *
 *     duplicate named parameters
 *     unknown parameter names
 *     missing required parameters
 *     excessive positional arguments
 *     invalid default ordering
 *     incompatible parameter types
 *     invalid parameter domains
 */


/* ============================================================================
 * 60. BOUNDARY TEST CONTRACT
 * ========================================================================== */

/*
 * Tests MUST include:
 *
 *     zero parameters
 *     one parameter
 *     many parameters
 *     very large parameter lists
 *     deeply nested parameter expressions
 *     very large symbolic expressions
 *     large named-binding sets
 *     large positional-binding sets
 *     mixed positional/named bindings
 *     nested parameterized operations
 *
 * No test may encode an architectural maximum such as:
 *
 *     8 parameters
 *     32 parameters
 *     64 parameters
 *     1024 parameters
 *
 * unless the test is explicitly testing an external resource/security policy.
 */


/* ============================================================================
 * 61. ROUND-TRIP CONTRACT
 * ========================================================================== */

/*
 * Where a canonical formatter/printer exists:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     formatter
 *       |
 *       v
 *     parser
 *
 * must preserve:
 *
 *     positional vs named binding
 *     parameter ordering
 *     default values
 *     expressions
 *     source semantics
 */


/* ============================================================================
 * 62. DETERMINISM TEST CONTRACT
 * ========================================================================== */

/*
 * Parsing identical source with identical language-version configuration must
 * produce structurally identical parameter syntax.
 *
 * No:
 *
 *     global mutable parameter state
 *
 * may influence parsing.
 */


/* ============================================================================
 * 63. SECURITY / RESOURCE POLICY
 * ========================================================================== */

/*
 * This grammar has no architectural parameter-size limit.
 *
 * Parser/compiler infrastructure MAY apply explicit resource policies for:
 *
 *     source bytes
 *     token count
 *     AST nodes
 *     expression depth
 *     parameter count
 *
 * Those policies belong outside the language grammar.
 *
 * A resource-policy rejection must be distinguishable from:
 *
 *     invalid Zamani syntax.
 */


/* ============================================================================
 * 64. FINAL OWNERSHIP CONTRACT
 * ========================================================================== */

/*
 * After integration, the ownership graph is:
 *
 *     lexer/
 *         |
 *         +--> tokens / identifiers / punctuation
 *
 *     expressions/
 *         |
 *         +--> expression syntax
 *
 *     types/
 *         |
 *         +--> type syntax
 *
 *     quantum/qubits.g4
 *         |
 *         +--> qubit/register references
 *
 *     quantum/gates.g4
 *         |
 *         +--> gate definitions / gate vocabulary
 *
 *     quantum/parameterized-operations.g4
 *         |
 *         +--> PARAMETER DECLARATION/BINDING SYNTAX
 *         |
 *         v
 *     quantum/operations.g4
 *         |
 *         +--> operation invocation / targets
 *         |
 *         v
 *     quantum/controlled-operations.g4
 *         |
 *         +--> control semantics
 *         |
 *         v
 *     frontend AST
 *         |
 *         v
 *     semantic analysis
 *         |
 *         v
 *     quantum::ir::core::parameter
 *         |
 *         v
 *     quantum::ir
 *         |
 *         +--> optimization
 *         +--> routing
 *         +--> scheduling
 *         +--> ZQN
 *         +--> QEC
 *         +--> resilience
 *         +--> hardware
 *         +--> runtime
 *
 * This file therefore remains a syntax-layer component and does not become
 * another quantum semantic implementation.
 */


/* ============================================================================
 * 65. COMPLETION CRITERIA
 * ========================================================================== */

/*
 * This file is COMPLETE when:
 *
 * [x] Parameter declarations have one canonical owner.
 *
 * [x] Parameter bindings have one canonical owner.
 *
 * [x] Positional arguments are supported.
 *
 * [x] Named arguments are supported.
 *
 * [x] Mixed positional/named arguments are deterministic.
 *
 * [x] Default values are representable.
 *
 * [x] Parameter types delegate to the canonical type grammar.
 *
 * [x] Parameter expressions delegate to the canonical expression grammar.
 *
 * [x] No fixed parameter count exists.
 *
 * [x] No machine-size assumption exists.
 *
 * [x] No hardware dependency exists.
 *
 * [x] No QEC dependency exists.
 *
 * [x] No ZQN dependency exists.
 *
 * [x] No routing dependency exists.
 *
 * [x] No scheduling dependency exists.
 *
 * [x] No optimization implementation exists.
 *
 * [x] No duplicate quantum IR exists.
 *
 * [x] Canonical quantum::ir parameter semantics remain downstream.
 *
 * [x] Existing RX(theta)(q)-style syntax remains representable.
 *
 * [x] User-defined and future operations remain possible.
 *
 * [x] Parameter semantics remain independent of physical hardware.
 *
 * [x] Rust 1.97 / 1.97.1 compatibility is preserved by the consuming frontend.
 *
 * [x] No unsafe Rust is required.
 *
 * [x] Parameter syntax is deterministic.
 *
 * [x] Source information can be preserved for AST diagnostics.
 *
 * [x] Resource/security limits are external policies rather than grammar
 *     semantics.
 */