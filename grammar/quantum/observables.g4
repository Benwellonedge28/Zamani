/*
 * ============================================================================
 * Zamani Universal Programming Language
 * Quantum Observable Grammar
 * ============================================================================
 *
 * File:
 *     grammar/quantum/observables.g4
 *
 * Role:
 *     Canonical reusable parser fragment for SOURCE-LEVEL quantum observable
 *     syntax.
 *
 * Language:
 *     Zamani
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * This file defines HOW Zamani SOURCE CODE expresses an observable.
 *
 * It does NOT define the semantic implementation of an observable.
 *
 * The intended pipeline is:
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
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          +--> optimization
 *          +--> routing
 *          +--> scheduling
 *          +--> QEC
 *          +--> ZQN
 *          +--> resilience
 *          +--> hardware HAL
 *          +--> simulator
 *          |
 *          v
 *     target realization
 *
 * This file MUST remain above the semantic IR boundary.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - observable source syntax;
 *   - observable expressions;
 *   - observable references;
 *   - Pauli-axis syntax;
 *   - Pauli-product syntax;
 *   - generalized-observable references;
 *   - observable composition syntax;
 *   - observable tensor/product syntax;
 *   - observable target association;
 *   - observable modifiers/options;
 *   - observable declarations;
 *   - observable aliases;
 *   - observable annotations;
 *   - source-level observable intent.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - qubit identity;
 *   - physical-qubit identity;
 *   - QubitId;
 *   - ClassicalBitId;
 *   - measurement implementation;
 *   - probability calculation;
 *   - expectation-value calculation;
 *   - sampling;
 *   - state-vector simulation;
 *   - density-matrix simulation;
 *   - observable matrices;
 *   - hardware readout;
 *   - detector configuration;
 *   - ADC/DAC configuration;
 *   - calibration;
 *   - pulse generation;
 *   - scheduling;
 *   - routing;
 *   - optimization;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - backend selection;
 *   - physical placement;
 *   - machine topology;
 *   - machine-size limits;
 *   - canonical quantum IR.
 *
 * ============================================================================
 * CANONICAL IR BOUNDARY
 * ============================================================================
 *
 * The existing canonical quantum IR already owns semantic concepts including:
 *
 *     MeasurementObservable
 *     MeasurementBasis
 *     PauliAxis
 *     PauliFactor
 *     PauliProduct
 *     MeasurementKind
 *
 * See:
 *
 *     src/quantum/ir/quantum/measurement.rs
 *
 * This grammar MUST NOT define source-level replacements for those Rust types.
 *
 * Instead:
 *
 *     observable syntax
 *             |
 *             v
 *     frontend AST
 *             |
 *             v
 *     semantic lowering
 *             |
 *             v
 *     quantum::ir::quantum::measurement
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Observable syntax MUST NOT encode the physical realization.
 *
 * It MUST NOT impose limits on:
 *
 *   - number of qubits;
 *   - number of observable factors;
 *   - number of observables;
 *   - number of measurements;
 *   - number of shots;
 *   - number of devices;
 *   - number of readout channels;
 *   - number of classical results;
 *   - number of nodes;
 *   - matrix dimensions;
 *   - tensor dimensions;
 *   - circuit depth;
 *   - backend size.
 *
 * There is deliberately no:
 *
 *     MAX_QUBITS
 *     MAX_OBSERVABLES
 *     MAX_PAULI_WEIGHT
 *     MAX_FACTORS
 *
 * in this grammar.
 *
 * Any actual limit belongs to:
 *
 *     semantic validation
 *     QuantumIrLimits
 *     resource management
 *     scheduling
 *     hardware capability negotiation
 *     runtime resources
 *
 * Therefore:
 *
 *     source semantics != hardware capacity
 *
 * ============================================================================
 * IMPORTANT MODULAR-GRAMMAR RULE
 * ============================================================================
 *
 * This file is a parser-fragment source file.
 *
 * It intentionally does not define:
 *
 *     lexer grammar
 *     tokens
 *     lexer keywords
 *     QubitId
 *     expression
 *     identifier
 *     qualifiedName
 *     typeExpression
 *
 * Those belong to the canonical grammar foundations.
 *
 * The canonical parser integration layer supplies those rules.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * Observable syntax should use the canonical Zamani vocabulary.
 *
 * Preferred keyword spelling:
 *
 *     observe
 *     observable
 *     basis
 *     pauli
 *     generalized
 *     product
 *     tensor
 *     on
 *     as
 *     where
 *
 * IMPORTANT:
 *
 * The observable grammar does NOT create lexer tokens for every possible
 * observable name.
 *
 * Observable names remain identifiers or qualified names.
 *
 * This allows:
 *
 *     X
 *     Y
 *     Z
 *     PauliX
 *     PauliY
 *     PauliZ
 *     Magnetization
 *     Hamiltonian
 *     Energy
 *     MyObservable
 *
 * without changing the lexer whenever a new semantic observable is added.
 *
 * ============================================================================
 * STANDARD OBSERVABLES
 * ============================================================================
 *
 * The grammar recognizes the following semantic forms:
 *
 *     X
 *     Y
 *     Z
 *
 *     pauli(X)
 *     pauli(Y)
 *     pauli(Z)
 *
 *     pauli(X, Y, Z)
 *
 *     product(X(q0), Z(q1))
 *
 *     observable MyObservable
 *
 *     observe Hamiltonian on q;
 *
 * The canonical semantic representation is selected later.
 *
 * ============================================================================
 * PAULI PRODUCT
 * ============================================================================
 *
 * A Pauli product expresses a joint observable such as:
 *
 *     X(q0) * Z(q1)
 *
 * or:
 *
 *     product(X(q0), Z(q1))
 *
 * The grammar does not impose a maximum number of factors.
 *
 * Duplicate-qubit validation is semantic.
 *
 * The grammar therefore accepts:
 *
 *     X(q0) * X(q0)
 *
 * syntactically.
 *
 * Semantic analysis must decide whether such a product is legal.
 *
 * ============================================================================
 * TARGET SEPARATION
 * ============================================================================
 *
 * An observable describes WHAT is measured.
 *
 * A target describes WHERE the semantic observable is applied.
 *
 * Therefore these concepts remain separate:
 *
 *     observable
 *     target
 *
 * For example:
 *
 *     observe X on q;
 *
 * means:
 *
 *     observable = X
 *     target     = q
 *
 * The grammar does not determine whether q is:
 *
 *     logical;
 *     physical;
 *     virtual;
 *     register-derived;
 *     dynamically selected.
 *
 * Semantic analysis determines that.
 *
 * ============================================================================
 * GENERALIZED OBSERVABLES
 * ============================================================================
 *
 * Future observable systems must not require grammar rewrites.
 *
 * Therefore:
 *
 *     generalized(MyObservable)
 *
 * is syntactically valid.
 *
 * Its actual semantic validity belongs to the observable registry/type system.
 *
 * This supports:
 *
 *     POVM observables
 *     custom Hermitian observables
 *     domain-specific observables
 *     application-defined observables
 *     future quantum measurement models
 *
 * ============================================================================
 * OBSERVABLE DECLARATIONS
 * ============================================================================
 *
 * A source program may define a semantic observable:
 *
 *     observable Energy = ...
 *
 *     observable Hamiltonian = ...
 *
 * The expression after '=' is intentionally delegated to the general
 * expression grammar.
 *
 * The observable grammar does not attempt to define matrix algebra itself.
 *
 * ============================================================================
 * OBSERVABLE ALIASES
 * ============================================================================
 *
 * Observable aliases may be introduced without creating new primitive syntax:
 *
 *     observable ZEnergy = Z;
 *
 *     observable Magnetization = product(
 *         Z(q0),
 *         Z(q1)
 *     );
 *
 * Semantic validation determines whether the expression denotes a valid
 * observable.
 *
 * ============================================================================
 * COMPOSITION
 * ============================================================================
 *
 * Observable composition is deliberately generic.
 *
 * The grammar supports:
 *
 *     product(...)
 *     tensor(...)
 *
 * while leaving algebraic meaning to semantic analysis.
 *
 * This prevents the grammar from assuming that all future quantum platforms
 * use the same matrix/tensor realization.
 *
 * ============================================================================
 * MEASUREMENT INTEGRATION
 * ============================================================================
 *
 * `measurement.g4` owns measurement syntax.
 *
 * This file owns observable syntax.
 *
 * Therefore:
 *
 *     measurement.g4
 *             |
 *             +--> quantumObservableExpression
 *
 * rather than measurement.g4 redefining observable syntax.
 *
 * ============================================================================
 * OBSERVATION INTEGRATION
 * ============================================================================
 *
 * `quantum.g4` currently contains an observation statement.
 *
 * The final modular architecture should change that relationship to:
 *
 *     quantum.g4
 *          |
 *          +--> quantumObservationStatement
 *                    |
 *                    +--> quantumObservableExpression
 *                    +--> quantumObservableTargetClause
 *
 * `quantum.g4` MUST NOT create another observable grammar.
 *
 * ============================================================================
 * QEC / ZQN / RESILIENCE
 * ============================================================================
 *
 * This file has no ownership of:
 *
 *     QEC algorithms
 *     syndrome decoding
 *     noise channels
 *     readout errors
 *     mitigation
 *     recovery
 *     retries
 *     backend switching
 *
 * Those systems may consume the semantic observable after lowering.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     filesystem access
 *     network access
 *     process execution
 *     dynamic code execution
 *     hardware access
 *     backend discovery
 *
 * Observable source is treated strictly as source syntax.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Observable parsing must be deterministic.
 *
 * Equivalent syntax must have an unambiguous parse tree.
 *
 * No semantic evaluation occurs in the parser.
 *
 * ============================================================================
 */


/* ============================================================================
 * 1. OBSERVABLE DECLARATION
 * ========================================================================== */

/*
 * Defines a named semantic observable.
 *
 * Examples:
 *
 *     observable Energy = expression;
 *
 *     observable Hamiltonian = product(...);
 *
 * The expression grammar owns the right-hand side.
 */
quantumObservableDeclaration
    : K_OBSERVABLE identifier
      ASSIGN expression
      SEMICOLON
    ;


/* ============================================================================
 * 2. OBSERVABLE ALIAS
 * ========================================================================== */

/*
 * An alias is represented as a declaration whose expression is another
 * observable expression.
 *
 * No separate alias keyword is necessary.
 */
quantumObservableAlias
    : K_OBSERVABLE identifier
      ASSIGN quantumObservableExpression
      SEMICOLON
    ;


/* ============================================================================
 * 3. OBSERVABLE EXPRESSION
 * ========================================================================== */

/*
 * Central observable syntax boundary.
 *
 * Every quantum measurement/observation construct that needs an observable
 * should consume this rule instead of defining its own observable syntax.
 */
quantumObservableExpression
    : quantumObservableReference
    | quantumPauliObservable
    | quantumPauliProductObservable
    | quantumGeneralizedObservable
    | quantumObservableProduct
    | quantumObservableTensor
    | LPAREN quantumObservableExpression RPAREN
    ;


/* ============================================================================
 * 4. NAMED OBSERVABLE REFERENCE
 * ========================================================================== */

/*
 * Named observables may be:
 *
 *     X
 *     Y
 *     Z
 *     Energy
 *     Hamiltonian
 *     MyObservable
 *     namespace::Observable
 *
 * Whether the identifier resolves to an observable is semantic analysis.
 */
quantumObservableReference
    : qualifiedName
    ;


/* ============================================================================
 * 5. STANDARD PAULI OBSERVABLE
 * ========================================================================== */

/*
 * Explicit Pauli syntax:
 *
 *     pauli(X)
 *     pauli(Y)
 *     pauli(Z)
 *
 * This syntax is preferable for unambiguous machine-independent source.
 */
quantumPauliObservable
    : K_PAULI
      LPAREN
      quantumPauliAxis
      RPAREN
    ;


/* ============================================================================
 * 6. PAULI AXIS
 * ========================================================================== */

/*
 * The actual X/Y/Z identifiers remain semantic names.
 *
 * If the canonical lexer does not reserve them, they are represented through
 * identifier/qualified-name syntax.
 *
 * Do not create a separate lexer keyword for every Pauli axis.
 */
quantumPauliAxis
    : identifier
    ;


/* ============================================================================
 * 7. PAULI PRODUCT OBSERVABLE
 * ========================================================================== */

/*
 * Explicit product form:
 *
 *     product(
 *         X(q0),
 *         Z(q1)
 *     )
 *
 * Factor count is unbounded by grammar.
 */
quantumPauliProductObservable
    : K_PRODUCT
      LPAREN
      quantumPauliFactorList
      RPAREN
    ;


/* ============================================================================
 * 8. PAULI FACTOR LIST
 * ========================================================================== */

quantumPauliFactorList
    : quantumPauliFactor
      (COMMA quantumPauliFactor)*
      COMMA?
    ;


/* ============================================================================
 * 9. PAULI FACTOR
 * ========================================================================== */

/*
 * Examples:
 *
 *     X(q0)
 *     Y(q1)
 *     Z(q2)
 *
 * The target is an ordinary expression.
 *
 * No QubitId is defined here.
 */
quantumPauliFactor
    : quantumPauliAxis
      LPAREN
      quantumObservableTargetExpression
      RPAREN
    ;


/* ============================================================================
 * 10. INFIX PAULI PRODUCT
 * ========================================================================== */

/*
 * Also permit:
 *
 *     X(q0) * Z(q1)
 *
 * The multiplication operator is source syntax only.
 *
 * Semantic analysis determines whether the expression is a Pauli product,
 * ordinary algebra, or invalid.
 *
 * This rule is deliberately kept separate from the general expression grammar
 * so that the frontend can lower it as an observable when it occurs in an
 * observable context.
 */
quantumPauliProductInfix
    : quantumPauliFactor
      (STAR quantumPauliFactor)+
    ;


/* ============================================================================
 * 11. GENERALIZED OBSERVABLE
 * ========================================================================== */

/*
 * Examples:
 *
 *     generalized(Energy)
 *     generalized(Hamiltonian)
 *     generalized(MyPOVM)
 *
 * The referenced semantic object is resolved later.
 */
quantumGeneralizedObservable
    : K_GENERALIZED
      LPAREN
      quantumObservableReference
      RPAREN
    ;


/* ============================================================================
 * 12. GENERIC OBSERVABLE PRODUCT
 * ========================================================================== */

/*
 * Observable algebra may require a product that is not specifically Pauli.
 *
 * Example:
 *
 *     product(A, B)
 *
 * Semantic analysis determines the algebraic meaning.
 */
quantumObservableProduct
    : K_PRODUCT
      LPAREN
      quantumObservableExpressionList
      RPAREN
    ;


/* ============================================================================
 * 13. OBSERVABLE TENSOR PRODUCT
 * ========================================================================== */

/*
 * Example:
 *
 *     tensor(A, B)
 *
 * The grammar does not prescribe matrix dimensions or machine sizes.
 */
quantumObservableTensor
    : K_TENSOR
      LPAREN
      quantumObservableExpressionList
      RPAREN
    ;


/* ============================================================================
 * 14. OBSERVABLE EXPRESSION LIST
 * ========================================================================== */

quantumObservableExpressionList
    : quantumObservableExpression
      (COMMA quantumObservableExpression)*
      COMMA?
    ;


/* ============================================================================
 * 15. OBSERVABLE TARGET EXPRESSION
 * ========================================================================== */

/*
 * Observable targets are normal Zamani expressions.
 *
 * Examples:
 *
 *     q
 *     q[i]
 *     register
 *     register[i]
 *     logical_qubit
 *     selected_qubits
 *
 * The grammar does not impose target cardinality.
 */
quantumObservableTargetExpression
    : expression
    ;


/* ============================================================================
 * 16. OBSERVABLE TARGET LIST
 * ========================================================================== */

quantumObservableTargetList
    : quantumObservableTargetExpression
      (COMMA quantumObservableTargetExpression)*
      COMMA?
    ;


/* ============================================================================
 * 17. OBSERVABLE ON-CLAUSE
 * ========================================================================== */

/*
 * Canonical observation form:
 *
 *     observe X on q;
 *
 *     observe Hamiltonian on register;
 */
quantumObservableOnClause
    : K_ON
      quantumObservableTargetList
    ;


/* ============================================================================
 * 18. OBSERVATION STATEMENT
 * ========================================================================== */

/*
 * This is the canonical owner of source-level observation syntax.
 *
 * `quantum.g4` should delegate to this rule rather than maintain a duplicate
 * observation rule.
 */
quantumObservationStatement
    : K_OBSERVE
      quantumObservableExpression
      quantumObservableOnClause?
      SEMICOLON
    ;


/* ============================================================================
 * 19. OBSERVATION WITH RESULT DESTINATION
 * ========================================================================== */

/*
 * Optional result destination:
 *
 *     observe X on q -> result;
 *
 * If the general measurement model owns result destinations instead, this
 * rule should be folded into measurement.g4 rather than duplicated.
 *
 * The canonical integration contract is:
 *
 *     observable syntax
 *             |
 *             v
 *     measurement syntax
 *             |
 *             v
 *     semantic measurement AST
 */
quantumObservationWithDestination
    : K_OBSERVE
      quantumObservableExpression
      quantumObservableOnClause?
      THIN_ARROW
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 20. OBSERVABLE ANNOTATIONS
 * ========================================================================== */

/*
 * Observable metadata remains generic.
 *
 * Examples:
 *
 *     observable Energy
 *         @hermitian
 *         = ...;
 *
 *     observable H
 *         @domain("physics")
 *         = ...;
 *
 * Annotation semantics belong to the annotation/metadata system.
 */
quantumObservableAnnotation
    : annotation
    ;


/* ============================================================================
 * 21. OBSERVABLE REFERENCE WITH QUALIFICATION
 * ========================================================================== */

/*
 * Kept as an explicit integration boundary so semantic analysis can distinguish
 * a source observable reference from an arbitrary expression when needed.
 */
quantumObservableQualifiedReference
    : qualifiedName
    ;


/* ============================================================================
 * 22. OBSERVABLE MODIFIERS
 * ========================================================================== */

/*
 * Observable modifiers are intentionally open-ended.
 *
 * They are represented as generic attributes/annotations rather than a growing
 * keyword list.
 */
quantumObservableModifiers
    : attributes*
    ;


/* ============================================================================
 * 23. OBSERVABLE SPECIFICATION
 * ========================================================================== */

/*
 * General reusable observable specification.
 *
 * This is useful to measurement.g4 and future quantum dialects.
 */
quantumObservableSpecification
    : quantumObservableModifiers
      quantumObservableExpression
    ;


/* ============================================================================
 * 24. SEMANTIC EXTENSION POINT
 * ========================================================================== */

/*
 * Future quantum dialects may introduce additional observable syntax through
 * the dialect system.
 *
 * They MUST NOT modify the canonical meaning of existing observable syntax.
 *
 * Dialect resolution belongs outside this file.
 */
quantumObservableExtension
    : qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


/* ============================================================================
 * 25. OBSERVABLE CONTEXT
 * ========================================================================== */

/*
 * This rule is the recommended entry point for semantic consumers.
 *
 * It permits the canonical forms without exposing implementation-specific
 * observable structures.
 */
quantumObservable
    : quantumObservableExpression
    | quantumPauliProductInfix
    ;


/* ============================================================================
 * 26. RESERVED SEMANTIC NAMES
 * ========================================================================== */

/*
 * IMPORTANT:
 *
 * Do NOT add lexer rules such as:
 *
 *     X : 'X';
 *     Y : 'Y';
 *     Z : 'Z';
 *
 * here.
 *
 * Whether X/Y/Z are reserved identifiers is a language-wide lexical decision.
 *
 * This grammar must remain compatible with both:
 *
 *     X
 *
 * and:
 *
 *     namespace::X
 *
 * without creating duplicate lexer ownership.
 */


/* ============================================================================
 * 27. SCALABILITY CONTRACT
 * ========================================================================== */

/*
 * There is deliberately no recursion bounded by a machine-specific constant.
 *
 * Examples such as:
 *
 *     tensor(A, B, C, ...)
 *
 *     product(X(q0), Z(q1), Y(q2), ...)
 *
 * are bounded only by parser/runtime/resource availability.
 *
 * The grammar itself imposes no hardware-derived maximum.
 */


/* ============================================================================
 * 28. SEMANTIC VALIDATION CONTRACT
 * ========================================================================== */

/*
 * Semantic analysis MUST perform at least:
 *
 *   - observable name resolution;
 *   - namespace validation;
 *   - type validation;
 *   - quantum-target validation;
 *   - Pauli-axis validation;
 *   - Pauli-product duplicate-target validation;
 *   - generalized-observable validation;
 *   - Hermiticity requirements where required;
 *   - observable/target arity validation;
 *   - resource-limit validation;
 *   - dialect validation;
 *   - capability validation;
 *   - compatibility validation.
 *
 * None of those checks belong in this grammar.
 */


/* ============================================================================
 * 29. CANONICAL LOWERING CONTRACT
 * ========================================================================== */

/*
 * Recommended lowering:
 *
 *     quantumObservableReference
 *             |
 *             v
 *     resolved semantic observable
 *
 *     quantumPauliObservable
 *             |
 *             v
 *     MeasurementBasis / PauliAxis
 *
 *     quantumPauliProductObservable
 *             |
 *             v
 *     PauliProduct
 *
 *     quantumGeneralizedObservable
 *             |
 *             v
 *     MeasurementObservable::Generalized
 *
 *     quantumObservationStatement
 *             |
 *             v
 *     Measurement / Observation semantic operation
 *
 * No grammar rule constructs these Rust values.
 */


/* ============================================================================
 * 30. ERROR-RECOVERY CONTRACT
 * ========================================================================== */

/*
 * The parser should preserve enough structure for diagnostics such as:
 *
 *     missing observable
 *     missing target
 *     missing closing parenthesis
 *     invalid factor separator
 *     invalid observable declaration
 *
 * Semantic errors must not be converted into parser errors.
 */


/* ============================================================================
 * 31. COMPATIBILITY CONTRACT
 * ========================================================================== */

/*
 * Existing source forms must remain parseable where they are already part of
 * the Zamani language contract.
 *
 * In particular:
 *
 *     observe <expression> [on <targets>];
 *
 * must remain available through the canonical observation entry point.
 *
 * Migration from the old inline quantum.g4 observation rule must preserve the
 * existing parse-tree/AST meaning where possible.
 */


/* ============================================================================
 * 32. HARD-CODING AUDIT
 * ========================================================================== */

/*
 * This file contains no:
 *
 *     fixed qubit count;
 *     fixed observable count;
 *     fixed Pauli-product width;
 *     fixed target count;
 *     fixed result count;
 *     fixed backend;
 *     fixed device;
 *     fixed topology;
 *     fixed readout channel;
 *     fixed hardware timing;
 *     fixed matrix dimension;
 *     fixed tensor dimension.
 *
 * Any such requirement belongs downstream.
 */


/* ============================================================================
 * 33. SECURITY AUDIT
 * ========================================================================== */

/*
 * This file:
 *
 *   - performs no I/O;
 *   - performs no network access;
 *   - performs no process execution;
 *   - performs no hardware discovery;
 *   - performs no backend selection;
 *   - performs no dynamic code execution.
 *
 * Safe Rust remains mandatory for all generated/frontend implementation code.
 */


/* ============================================================================
 * 34. TEST CONTRACT
 * ========================================================================== */

/*
 * Positive examples:
 *
 *     observe X;
 *     observe Y;
 *     observe Z;
 *
 *     observe pauli(X);
 *     observe pauli(Y);
 *     observe pauli(Z);
 *
 *     observe product(X(q0), Z(q1));
 *
 *     observe generalized(Hamiltonian);
 *
 *     observe Energy on q;
 *
 *     observable Energy = Z;
 *
 *     observable Hamiltonian = product(X(q0), Z(q1));
 *
 *
 * Negative examples:
 *
 *     observe;
 *
 *     observe product();
 *
 *     observe pauli();
 *
 *     observable;
 *
 *     observable Energy;
 *
 *
 * Boundary examples:
 *
 *     product(
 *         X(q0),
 *         X(q1),
 *         X(q2),
 *         ...
 *     )
 *
 *     tensor(A, B, C, ...)
 *
 * No artificial grammar maximum may be introduced.
 */


/* ============================================================================
 * 35. CROSS-DOMAIN TEST CONTRACT
 * ========================================================================== */

/*
 * Required integration scenarios:
 *
 *     classical -> quantum observable
 *
 *     quantum -> observable -> classical result
 *
 *     quantum -> observable -> dynamic control
 *
 *     quantum -> observable -> QEC
 *
 *     quantum -> observable -> ZQN
 *
 *     quantum -> observable -> resilience
 *
 *     quantum -> observable -> scheduling
 *
 *     quantum -> observable -> hardware realization
 *
 *     quantum -> observable -> simulator
 *
 *     quantum + distributed observable evaluation
 *
 *     quantum + accelerator-backed observable evaluation
 *
 * The grammar must remain unchanged across these target scenarios.
 */


/* ============================================================================
 * 36. DETERMINISM TEST CONTRACT
 * ========================================================================== */

/*
 * The same source text MUST produce the same parse structure.
 *
 * Observable declaration order, factor order, and target order must remain
 * source-preserving until semantic canonicalization.
 *
 * Canonicalization belongs to semantic/IR layers.
 */


/* ============================================================================
 * 37. COMPLETION CRITERIA
 * ========================================================================== */

/*
 * This file is complete when:
 *
 * [ ] observable syntax has one authoritative grammar owner;
 *
 * [ ] observation syntax has one authoritative grammar owner;
 *
 * [ ] measurement.g4 consumes observable syntax rather than redefining it;
 *
 * [ ] quantum.g4 consumes the canonical observation rule rather than
 *     maintaining a duplicate;
 *
 * [ ] X/Y/Z do not require dedicated lexer ownership here;
 *
 * [ ] named observables are supported;
 *
 * [ ] generalized observables are supported;
 *
 * [ ] Pauli products are supported;
 *
 * [ ] observable composition is supported;
 *
 * [ ] target cardinality is not hard-coded;
 *
 * [ ] observable cardinality is not hard-coded;
 *
 * [ ] physical hardware is not encoded;
 *
 * [ ] semantic IR types are not duplicated;
 *
 * [ ] quantum::ir remains the canonical semantic boundary;
 *
 * [ ] QEC remains downstream;
 *
 * [ ] ZQN remains downstream;
 *
 * [ ] scheduling remains downstream;
 *
 * [ ] routing remains downstream;
 *
 * [ ] hardware HAL remains downstream;
 *
 * [ ] runtime remains downstream;
 *
 * [ ] dialects remain extensibility mechanisms;
 *
 * [ ] positive tests exist;
 *
 * [ ] negative tests exist;
 *
 * [ ] boundary/scalability tests exist;
 *
 * [ ] deterministic parsing tests exist;
 *
 * [ ] cross-domain tests exist;
 *
 * [ ] no unsafe Rust is introduced;
 *
 * [ ] Rust 1.97 / 1.97.1 compatibility is preserved;
 *
 * [ ] no machine-size assumption exists in the grammar.
 */


/* ============================================================================
 * END OF FILE
 * ========================================================================== */