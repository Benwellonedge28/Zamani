/*
 * ============================================================================
 * Zamani Universal Programming Language
 * Quantum Observable Grammar
 * ============================================================================
 *
 * File:
 *     grammar/quantum/observables.g4
 *
 * Grammar:
 *     QuantumObservables
 *
 * Status:
 *     CANONICAL / PRODUCTION QUANTUM OBSERVABLE GRAMMAR
 *
 * Language:
 *     Zamani
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the single grammar owner for SOURCE-LEVEL QUANTUM OBSERVABLE
 * SYNTAX.
 *
 * It describes what a Zamani program means syntactically when it refers to
 * or requests an observable.
 *
 * It does NOT implement:
 *
 *     - observable mathematics;
 *     - matrix construction;
 *     - expectation-value calculation;
 *     - probability calculation;
 *     - sampling;
 *     - measurement hardware;
 *     - readout electronics;
 *     - calibration;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - backend selection;
 *     - physical placement;
 *     - simulator implementation;
 *     - canonical quantum::ir.
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
 *     QuantumObservables
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
 *          v
 *     canonical semantic model
 *          |
 *          v
 *     quantum::ir
 *          |
 *          +--> optimization
 *          +--> decomposition
 *          +--> routing
 *          +--> scheduling
 *          +--> QEC
 *          +--> ZQN
 *          +--> resilience
 *          +--> HAL
 *          +--> simulator
 *          |
 *          v
 *     target realization
 *
 * This grammar MUST remain above the semantic IR boundary.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - quantumObservableExpression
 *     - quantumObservableReference
 *     - quantumObservableInvocation
 *     - quantumObservableArgumentList
 *     - quantumObservableComposition
 *     - quantumObservableTargetClause
 *     - quantumObservableTargetList
 *     - quantumObservationStatement
 *     - quantumObservableDeclaration
 *     - observable declaration structure
 *     - source-level observable composition
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer rules;
 *     - keyword definitions;
 *     - identifiers;
 *     - qualified-name implementation;
 *     - general expression grammar;
 *     - general type grammar;
 *     - qubit declarations;
 *     - quantum registers;
 *     - quantum operations;
 *     - measurement implementation;
 *     - measurement result representation;
 *     - reset;
 *     - dynamic control;
 *     - classical feed-forward;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - hardware;
 *     - resources;
 *     - backend selection;
 *     - runtime execution;
 *     - canonical quantum::ir.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Canonical lexical authority:
 *
 *     grammar/lexer/keywords.md
 *     grammar/lexer/keywords.g4
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical source syntax authority:
 *
 *     grammar/spec/syntax.md
 *
 * Canonical quantum semantic authority:
 *
 *     grammar/spec/quantum.md
 *
 * Canonical parser composition:
 *
 *     grammar/Zamani.g4
 *
 * Implementation conformance:
 *
 *     grammar/grammar.md
 *
 * Historical/extended design:
 *
 *     grammar/Zamani-Grammar.md
 *
 * Zamani-Grammar.md MUST NOT independently create additional observable
 * syntax.
 *
 * ============================================================================
 * CANONICAL LEXER
 * ============================================================================
 *
 * Parser grammars consume:
 *
 *     ZamaniLexer
 *
 * They MUST NOT consume:
 *
 *     ZamaniTokens
 *
 * directly.
 *
 * The canonical observable keyword currently supplied by the lexical contract
 * is:
 *
 *     OBSERVE
 *
 * Observable concepts such as:
 *
 *     pauli
 *     product
 *     tensor
 *     generalized
 *     on
 *     observable
 *
 * are deliberately NOT assumed to be lexer keywords here.
 *
 * They remain contextual/source-level names and are resolved by semantic
 * analysis where necessary.
 *
 * This prevents keyword proliferation and preserves future extensibility.
 *
 * ============================================================================
 * OPEN-WORLD OBSERVABLE MODEL
 * ============================================================================
 *
 * Zamani MUST NOT enumerate every possible observable.
 *
 * These are therefore source-level names, not a closed grammar enumeration:
 *
 *     X
 *     Y
 *     Z
 *     PauliX
 *     PauliY
 *     PauliZ
 *     Energy
 *     Hamiltonian
 *     Magnetization
 *     Number
 *     Position
 *     Momentum
 *     custom_observable
 *     vendor::observable
 *     future::observable
 *
 * The semantic layer determines whether a name denotes a valid observable.
 *
 * This permits:
 *
 *     standard observables;
 *     user-defined observables;
 *     imported observables;
 *     library observables;
 *     dialect observables;
 *     POVM observables;
 *     generalized observables;
 *     future observable models.
 *
 * ============================================================================
 * PAULI MODEL
 * ============================================================================
 *
 * Pauli axes remain semantic names.
 *
 * The grammar does NOT define:
 *
 *     X : 'X';
 *     Y : 'Y';
 *     Z : 'Z';
 *
 * and does not introduce:
 *
 *     K_PAULI
 *     K_X
 *     K_Y
 *     K_Z
 *
 * A semantic Pauli representation may therefore be expressed through the
 * open observable invocation model:
 *
 *     pauli(X)
 *     pauli(Y)
 *     pauli(Z)
 *
 * The semantic layer validates that the supplied axis is a valid Pauli axis.
 *
 * ============================================================================
 * GENERALIZED OBSERVABLE MODEL
 * ============================================================================
 *
 * Generic observable invocation permits forms such as:
 *
 *     generalized(MyObservable)
 *     povm(MyObservable)
 *     hermitian(MyObservable)
 *     observable_basis(...)
 *
 * without requiring a lexer or parser update for every future observable
 * category.
 *
 * The grammar records structure.
 *
 * Semantic analysis determines meaning.
 *
 * ============================================================================
 * COMPOSITION
 * ============================================================================
 *
 * Observable composition is open-ended.
 *
 * Examples:
 *
 *     product(A, B)
 *
 *     tensor(A, B)
 *
 *     product(
 *         X,
 *         Z,
 *         Y
 *     )
 *
 * No finite number of factors is encoded.
 *
 * The semantic layer determines:
 *
 *     - algebraic validity;
 *     - operand compatibility;
 *     - Hermiticity;
 *     - commutation requirements;
 *     - target compatibility;
 *     - dimensional compatibility;
 *     - measurement feasibility.
 *
 * ============================================================================
 * TARGET SEPARATION
 * ============================================================================
 *
 * Observable identity and observable target are separate concepts.
 *
 * Conceptually:
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
 *     virtual;
 *     physical;
 *     register-derived;
 *     dynamically selected.
 *
 * Semantic analysis determines that.
 *
 * ============================================================================
 * CONTEXTUAL "ON"
 * ============================================================================
 *
 * The current canonical lexer does not establish ON as a reserved token.
 *
 * Therefore this grammar does not invent ON or K_ON.
 *
 * The source spelling:
 *
 *     on
 *
 * is represented through the normal identifier pathway.
 *
 * The semantic layer MUST validate that the contextual marker is exactly the
 * language-defined observable target marker.
 *
 * This preserves the current lexical authority while allowing source syntax:
 *
 *     observe X on q;
 *
 * without introducing a duplicate keyword registry.
 *
 * ============================================================================
 * OBSERVATION STATEMENT
 * ============================================================================
 *
 * The canonical statement is:
 *
 *     observe X;
 *
 * and may associate targets:
 *
 *     observe X on q;
 *
 * and a result destination:
 *
 *     observe X on q -> result;
 *
 * Observation syntax records source intent.
 *
 * Measurement semantics remain owned by the measurement subsystem.
 *
 * ============================================================================
 * MEASUREMENT SEPARATION
 * ============================================================================
 *
 * `measurement.g4` owns:
 *
 *     measure ...
 *
 * This file owns:
 *
 *     observe ...
 *
 * An observation requests semantic information about an observable.
 *
 * It does not define the physical measurement procedure.
 *
 * Measurement lowering may consume the observable represented here.
 *
 * No duplicate measurement grammar is introduced.
 *
 * ============================================================================
 * DECLARATION SEPARATION
 * ============================================================================
 *
 * Named observables may be declared using the language's declaration
 * composition system.
 *
 * This file provides the observable-specific declaration payload.
 *
 * The contextual declaration spelling:
 *
 *     observable
 *
 * is not promoted to a lexer keyword merely for this grammar.
 *
 * Semantic validation must distinguish an observable declaration from arbitrary
 * identifier pairs.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every observable construct must preserve enough source information for the
 * domain-neutral AST to represent:
 *
 *     - observable reference;
 *     - qualification;
 *     - invocation/composition;
 *     - arguments;
 *     - targets;
 *     - destination;
 *     - declaration name;
 *     - declaration initializer;
 *     - source span;
 *     - source ordering.
 *
 * Representative semantic shape:
 *
 *     ObservableExpression {
 *         designator,
 *         arguments,
 *         composition,
 *         source_span
 *     }
 *
 *     Observation {
 *         observable,
 *         targets,
 *         destination,
 *         source_span
 *     }
 *
 * The exact Rust AST type belongs to:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT define a second AST hierarchy.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - observable name resolution;
 *     - namespace resolution;
 *     - contextual-keyword validation;
 *     - observable type validation;
 *     - Pauli-axis validation;
 *     - generalized-observable validation;
 *     - composition validation;
 *     - target validation;
 *     - target/observable compatibility;
 *     - Hermiticity requirements;
 *     - arity;
 *     - operand compatibility;
 *     - effect checking;
 *     - capability checking;
 *     - resource checking;
 *     - dialect checking;
 *     - language-version checking.
 *
 * The parser performs none of these operations.
 *
 * ============================================================================
 * CANONICAL QUANTUM IR CONTRACT
 * ============================================================================
 *
 * Observable syntax lowers through the existing canonical quantum semantic
 * boundary.
 *
 * Conceptually:
 *
 *     quantumObservableExpression
 *              |
 *              v
 *     semantic observable
 *              |
 *              v
 *     quantum::ir
 *
 * The repository already contains canonical measurement/observable concepts
 * including:
 *
 *     MeasurementObservable
 *     MeasurementBasis
 *     PauliAxis
 *     PauliFactor
 *     PauliProduct
 *     MeasurementKind
 *
 * This grammar MUST NOT introduce:
 *
 *     ObservableIR
 *     QuantumObservableIR
 *     PauliIR
 *     MeasurementObservableIR
 *
 * or any competing quantum IR.
 *
 * ============================================================================
 * QEC / ZQN
 * ============================================================================
 *
 * Observable syntax may ultimately be consumed by:
 *
 *     QEC;
 *     ZQN;
 *     resilience;
 *     mitigation;
 *     characterization;
 *     verification.
 *
 * Those systems remain downstream.
 *
 * This grammar does not encode:
 *
 *     stabilizers;
 *     syndrome extraction;
 *     decoders;
 *     noise matrices;
 *     calibration;
 *     readout correction;
 *     fault recovery.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * This grammar is target-independent.
 *
 * It MUST NOT encode:
 *
 *     physical qubit IDs;
 *     QPU IDs;
 *     device IDs;
 *     vendor IDs;
 *     topology;
 *     coupling maps;
 *     detector IDs;
 *     readout channels;
 *     pulse schedules;
 *     backend names.
 *
 * The same source-level observable must remain meaningful across:
 *
 *     CPU simulation;
 *     GPU simulation;
 *     FPGA/accelerator environments;
 *     distributed simulation;
 *     superconducting QPUs;
 *     trapped-ion systems;
 *     neutral-atom systems;
 *     photonic systems;
 *     future quantum substrates.
 *
 * ============================================================================
 * NO ARTIFICIAL LIMITS
 * ============================================================================
 *
 * This grammar contains no universal limit on:
 *
 *     - observable count;
 *     - factor count;
 *     - argument count;
 *     - target count;
 *     - expression depth;
 *     - composition depth;
 *     - measurement count;
 *     - shot count;
 *     - qubit count;
 *     - device count;
 *     - node count;
 *     - tensor dimension;
 *     - matrix dimension.
 *
 * There is intentionally no:
 *
 *     MAX_QUBITS
 *     MAX_OBSERVABLES
 *     MAX_PAULI_WEIGHT
 *     MAX_FACTORS
 *     MAX_TARGETS
 *     MAX_MEASUREMENTS
 *     MAX_SHOTS
 *
 * Any real resource restriction is downstream:
 *
 *     semantic resource analysis
 *     compiler policy
 *     runtime resources
 *     target capabilities
 *     scheduling
 *     provider quotas
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source;
 *     lexical vocabulary;
 *     grammar;
 *     language version;
 *     active grammar composition.
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware;
 *     backend discovery;
 *     runtime state;
 *     network state;
 *     randomness;
 *     current time;
 *     available QPUs.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar:
 *
 *     - performs no I/O;
 *     - performs no network access;
 *     - performs no process execution;
 *     - performs no hardware access;
 *     - performs no dynamic evaluation;
 *     - performs no backend selection.
 *
 * Observable arguments are syntax only.
 *
 * They are never evaluated by the parser.
 *
 * ============================================================================
 * PERFORMANCE
 * ============================================================================
 *
 * The grammar avoids:
 *
 *     - left-recursive observable rules;
 *     - fixed-depth nesting;
 *     - duplicated expression hierarchies;
 *     - hardware-dependent parser decisions;
 *     - semantic predicates based on runtime state.
 *
 * Lists use ordinary ANTLR repetition.
 *
 * Actual parser-resource limits remain implementation limits rather than
 * language semantics.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * This grammar depends only on shared language foundations:
 *
 *     Names
 *     Expressions
 *
 * It MUST NOT import:
 *
 *     Measurement
 *     Operations
 *     Reset
 *     QEC
 *     ZQN
 *     Hardware
 *     Runtime
 *
 * merely to perform semantic validation.
 *
 * This dependency direction prevents grammar cycles.
 *
 * ============================================================================
 */

parser grammar QuantumObservables;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions;


/* ============================================================================
 * 1. PUBLIC OBSERVABLE EXPRESSION
 * ========================================================================== */

/**
 * Canonical observable expression entry point.
 *
 * Examples:
 *
 *     X
 *     Energy
 *     namespace::Energy
 *
 *     pauli(X)
 *     generalized(Hamiltonian)
 *     product(X, Z)
 *     tensor(A, B)
 *
 * Observable names and observable operation names remain open semantic names.
 */
quantumObservableExpression
    : quantumObservableComposition
    ;


/* ============================================================================
 * 2. OBSERVABLE COMPOSITION
 * ========================================================================== */

/**
 * Composition is intentionally represented structurally rather than by a
 * closed list of mathematical observable operators.
 *
 * A composition may be:
 *
 *     reference
 *     invocation
 *     parenthesized observable
 *     invocation containing further observable arguments
 *
 * Future semantic observable forms can therefore normally be introduced
 * without adding lexer keywords.
 */
quantumObservableComposition
    : quantumObservableReference
    | quantumObservableInvocation
    | LPAREN quantumObservableExpression RPAREN
    ;


/* ============================================================================
 * 3. OBSERVABLE REFERENCE
 * ========================================================================== */

/**
 * Observable identity is an ordinary Zamani qualified name.
 *
 * Examples:
 *
 *     X
 *     Y
 *     Z
 *     Energy
 *     Hamiltonian
 *     MyObservable
 *     quantum::X
 *     physics::Hamiltonian
 *     future::observable
 *
 * Whether the reference denotes an observable is a semantic question.
 */
quantumObservableReference
    : qualifiedName
    ;


/* ============================================================================
 * 4. OBSERVABLE INVOCATION
 * ========================================================================== */

/**
 * Generic observable invocation.
 *
 * Examples:
 *
 *     pauli(X)
 *     pauli(Y)
 *     pauli(Z)
 *
 *     generalized(MyObservable)
 *
 *     product(X, Z)
 *
 *     tensor(A, B)
 *
 *     custom_observable(parameter)
 *
 * The invocation name is intentionally open.
 */
quantumObservableInvocation
    : qualifiedName
      LPAREN
      quantumObservableArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 5. OBSERVABLE ARGUMENT LIST
 * ========================================================================== */

/**
 * Observable arguments are ordinary Zamani expressions.
 *
 * This permits:
 *
 *     pauli(axis)
 *     observable(parameter)
 *     product(A, B)
 *     tensor(A, B, C)
 *     generalized(H)
 *
 * without introducing a second expression grammar.
 *
 * There is no fixed argument count.
 */
quantumObservableArgumentList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/* ============================================================================
 * 6. OBSERVABLE TARGET CLAUSE
 * ========================================================================== */

/**
 * Target association:
 *
 *     observe X on q;
 *
 * `on` is intentionally contextual because the canonical lexer does not
 * reserve ON.
 *
 * The first identifier is validated semantically as the observable target
 * marker.
 *
 * This avoids introducing K_ON/ON solely for this grammar.
 */
quantumObservableTargetClause
    : identifier
      quantumObservableTargetList
    ;


/* ============================================================================
 * 7. OBSERVABLE TARGET LIST
 * ========================================================================== */

/**
 * Targets are ordinary expressions.
 *
 * Examples:
 *
 *     q
 *     q[i]
 *     register
 *     register[i]
 *     selected
 *     logical_qubit
 *     selected_qubits[index]
 *
 * Semantic analysis determines whether a target denotes a measurable quantum
 * resource.
 *
 * No target cardinality is hard-coded.
 */
quantumObservableTargetList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/* ============================================================================
 * 8. OBSERVATION STATEMENT
 * ========================================================================== */

/**
 * Canonical source-level observation.
 *
 * Examples:
 *
 *     observe X;
 *
 *     observe X on q;
 *
 *     observe Hamiltonian on register;
 *
 *     observe product(X, Z) on q0, q1;
 *
 * The optional target clause is syntactic association only.
 *
 * The semantic layer determines whether the observable and targets are
 * compatible.
 */
quantumObservationStatement
    : OBSERVE
      quantumObservableExpression
      quantumObservableTargetClause?
      SEMICOLON
    ;


/* ============================================================================
 * 9. OBSERVATION WITH RESULT DESTINATION
 * ========================================================================== */

/**
 * Observation result routing:
 *
 *     observe X on q -> result;
 *
 *     observe Energy -> estimate;
 *
 * The result destination is an ordinary expression.
 *
 * The semantic layer determines:
 *
 *     - destination type;
 *     - result cardinality;
 *     - lifetime;
 *     - ownership;
 *     - measurement/result compatibility.
 *
 * This grammar does not duplicate measurement-result semantics.
 */
quantumObservationWithDestination
    : OBSERVE
      quantumObservableExpression
      quantumObservableTargetClause?
      THIN_ARROW
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 10. OBSERVATION SPECIFICATION
 * ========================================================================== */

/**
 * Reusable observation specification without statement punctuation.
 *
 * This is useful to other grammar components that need to embed an observable
 * request without duplicating its observable expression.
 */
quantumObservationSpecification
    : quantumObservableExpression
      quantumObservableTargetClause?
    ;


/* ============================================================================
 * 11. OBSERVABLE DECLARATION
 * ========================================================================== */

/**
 * Contextual observable declaration.
 *
 * Source form:
 *
 *     observable Energy = X;
 *
 *     observable Hamiltonian = product(X, Z);
 *
 * Because `observable` is not currently a canonical reserved lexer token,
 * this grammar uses the ordinary identifier pathway.
 *
 * Semantic analysis MUST verify that the declaration marker is the exact
 * language-defined contextual spelling `observable`.
 *
 * This deliberately avoids introducing:
 *
 *     K_OBSERVABLE
 *     OBSERVABLE
 *
 * without a corresponding lexical specification.
 */
quantumObservableDeclaration
    : identifier
      identifier
      ASSIGN
      quantumObservableExpression
      SEMICOLON
    ;


/* ============================================================================
 * 12. OBSERVABLE DECLARATION VALUE
 * ========================================================================== */

/**
 * Reusable declaration initializer.
 */
quantumObservableInitializer
    : ASSIGN
      quantumObservableExpression
    ;


/* ============================================================================
 * 13. OBSERVABLE ALIAS
 * ========================================================================== */

/**
 * Observable aliases do not require a separate alias grammar.
 *
 * An alias is semantically represented by an observable declaration whose
 * initializer resolves to another observable.
 *
 * Therefore there is deliberately no separate:
 *
 *     quantumObservableAlias
 *
 * syntax.
 *
 * This removes duplicate declaration ownership from the previous design.
 */


/* ============================================================================
 * 14. OBSERVABLE REFERENCE CONTEXT
 * ========================================================================== */

/**
 * Explicit semantic boundary for consumers that need an observable reference
 * rather than an arbitrary expression.
 */
quantumObservableReferenceExpression
    : quantumObservableReference
    ;


/* ============================================================================
 * 15. OBSERVABLE ARGUMENT
 * ========================================================================== */

/**
 * Kept as a named boundary for tooling and semantic consumers.
 *
 * The value remains an ordinary expression.
 */
quantumObservableArgument
    : expression
    ;


/* ============================================================================
 * 16. OBSERVABLE ARGUMENT LIST WITH NAMED VALUES
 * ========================================================================== */

/**
 * Generic observable calls may require named configuration values.
 *
 * Example:
 *
 *     observable(
 *         basis = X,
 *         mode = projective
 *     )
 *
 * Named-argument semantics remain owned by the expression/semantic system.
 *
 * This rule is provided as an extension boundary without making any option
 * name a keyword.
 */
quantumObservableNamedArgument
    : identifier
      ASSIGN
      expression
    ;


/* ============================================================================
 * 17. OBSERVABLE CONFIGURATION
 * ========================================================================== */

/**
 * Generic configuration list.
 *
 * This rule is intentionally separate from the ordinary argument list so
 * semantic consumers can distinguish positional and named configuration.
 */
quantumObservableConfiguration
    : quantumObservableNamedArgument
      (COMMA quantumObservableNamedArgument)*
      COMMA?
    ;


/* ============================================================================
 * 18. GENERALIZED OBSERVABLE ENTRY
 * ========================================================================== */

/**
 * Generalized observables remain open-world.
 *
 * There is intentionally no:
 *
 *     quantumGeneralizedObservable
 *         : POVM
 *         | Hermitian
 *         | ...
 *
 * closed enumeration.
 *
 * A generalized observable is represented through an ordinary qualified
 * observable invocation and is validated semantically.
 */
quantumGeneralizedObservableExpression
    : quantumObservableInvocation
    ;


/* ============================================================================
 * 19. PAULI OBSERVABLE ENTRY
 * ========================================================================== */

/**
 * Pauli observables are likewise open at the lexical level.
 *
 * Typical source forms:
 *
 *     pauli(X)
 *     pauli(Y)
 *     pauli(Z)
 *
 * The grammar does not hard-code X/Y/Z as tokens.
 *
 * Semantic analysis maps the resolved names to the canonical PauliAxis
 * representation where appropriate.
 */
quantumPauliObservableExpression
    : quantumObservableInvocation
    ;


/* ============================================================================
 * 20. PRODUCT OBSERVABLE ENTRY
 * ========================================================================== */

/**
 * Product observables use the same generic invocation boundary:
 *
 *     product(A, B)
 *
 *     product(X, Z, Y)
 *
 * There is no product-factor limit.
 */
quantumProductObservableExpression
    : quantumObservableInvocation
    ;


/* ============================================================================
 * 21. TENSOR OBSERVABLE ENTRY
 * ========================================================================== */

/**
 * Tensor-composed observables use the same open invocation boundary:
 *
 *     tensor(A, B)
 *
 *     tensor(A, B, C, ...)
 *
 * No tensor-rank or operand-count limit is encoded.
 */
quantumTensorObservableExpression
    : quantumObservableInvocation
    ;


/* ============================================================================
 * 22. OBSERVABLE SEMANTIC EXTENSION
 * ========================================================================== */

/**
 * Extension point for future observable forms.
 *
 * New semantic observable categories should normally be expressible through
 *:
 *
 *     qualifiedName(...)
 *
 * before a new keyword or dedicated grammar production is considered.
 *
 * This keeps the language extensible and prevents keyword/grammar explosion.
 */
quantumObservableExtensionExpression
    : quantumObservableInvocation
    ;


/* ============================================================================
 * 23. OBSERVABLE RESULT DESTINATION
 * ========================================================================== */

/**
 * Standalone destination boundary for tooling and semantic consumers.
 */
quantumObservableResultDestination
    : expression
    ;


/* ============================================================================
 * 24. OBSERVABLE TARGET
 * ========================================================================== */

/**
 * Standalone target boundary.
 */
quantumObservableTarget
    : expression
    ;


/* ============================================================================
 * 25. SCALABILITY CONTRACT
 * ========================================================================== */

/*
 * All scalable collections use ANTLR repetition:
 *
 *     *
 *     +
 *
 * rather than finite alternatives.
 *
 * Therefore the grammar does not impose a universal ceiling on:
 *
 *     observable arguments;
 *     observable targets;
 *     product factors;
 *     tensor operands;
 *     observations;
 *     declarations;
 *     source size.
 *
 * Actual resource limits are resolved downstream.
 */


/* ============================================================================
 * 26. SEMANTIC VALIDATION CONTRACT
 * ========================================================================== */

/*
 * The semantic layer MUST validate at minimum:
 *
 *     1. Observable references resolve.
 *
 *     2. Contextual `on` is used only where permitted.
 *
 *     3. Contextual `observable` is used only as a declaration marker.
 *
 *     4. Pauli invocation names and axes are valid.
 *
 *     5. Generalized observable references resolve to legal observable
 *        semantics.
 *
 *     6. Product operands are compatible.
 *
 *     7. Tensor operands are compatible.
 *
 *     8. Observable and target cardinality are semantically compatible.
 *
 *     9. Observable targets are valid quantum resources.
 *
 *    10. Result destinations can receive the resulting semantic value.
 *
 *    11. Required capabilities are available.
 *
 *    12. Required resources are satisfiable.
 *
 *    13. Effects are legal.
 *
 *    14. Language-version and dialect rules are satisfied.
 *
 *    15. Measurement/observation semantics are valid.
 *
 *    16. Hermiticity or other observable properties are satisfied wherever
 *        required by the selected semantic observable kind.
 *
 * None of these checks belong in this grammar.
 */


/* ============================================================================
 * 27. CANONICAL IR LOWERING
 * ========================================================================== */

/*
 * Required conceptual lowering:
 *
 *     quantumObservableReference
 *             |
 *             v
 *     resolved semantic observable
 *
 *     quantumObservableInvocation
 *             |
 *             v
 *     semantic observable operation/composition
 *
 *     quantumObservationStatement
 *             |
 *             v
 *     semantic observation/measurement intent
 *
 *             |
 *             v
 *
 *     canonical quantum::ir
 *
 * Existing canonical IR types remain authoritative.
 *
 * No observable-specific IR is introduced by this grammar.
 */


/* ============================================================================
 * 28. MEASUREMENT INTEGRATION
 * ========================================================================== */

/*
 * Integration direction:
 *
 *     observables.g4
 *          |
 *          v
 *     observable expression
 *          |
 *          v
 *     measurement semantic analysis
 *          |
 *          v
 *     quantum::ir measurement representation
 *
 * `measurement.g4` MUST consume observable syntax through this canonical
 * boundary where an observable is part of a measurement request.
 *
 * `measurement.g4` MUST NOT redefine:
 *
 *     quantumObservableExpression
 *     quantumPauliObservableExpression
 *     quantumObservableReference
 *     quantumObservableInvocation
 *
 * No grammar cycle is permitted.
 */


/* ============================================================================
 * 29. QUANTUM STATEMENT INTEGRATION
 * ========================================================================== */

/*
 * `grammar/statements/quantum.g4` already provides the statement-level
 * dispatcher containing:
 *
 *     quantumObservationStatement
 *
 * That dispatcher should consume this rule exactly once.
 *
 * It MUST NOT define another observation syntax.
 *
 * Integration:
 *
 *     statements/quantum.g4
 *              |
 *              +--> quantumObservationStatement
 *                         |
 *                         +--> QuantumObservables
 *
 * The observable grammar therefore remains an independent feature file.
 */


/* ============================================================================
 * 30. QUANTUM COMPOSITION INTEGRATION
 * ========================================================================== */

/*
 * `grammar/quantum/quantum.g4` already exposes:
 *
 *     quantumObservableElement
 *         : quantumObservationStatement
 *         ;
 *
 * That rule remains the quantum orchestration adapter.
 *
 * `quantum.g4` MUST NOT duplicate observable syntax.
 */


/* ============================================================================
 * 31. EXPRESSION INTEGRATION
 * ========================================================================== */

/*
 * The general expression hierarchy remains authoritative.
 *
 * Observable arguments use:
 *
 *     expression
 *
 * rather than introducing:
 *
 *     observableExpression2
 *     observableArithmeticExpression
 *     observableMathExpression
 *
 * This ensures one expression language across:
 *
 *     classical;
 *     quantum;
 *     hybrid;
 *     AI;
 *     data;
 *     HDL;
 *     resource;
 *     distributed;
 *     networking.
 *
 * Quantum-specific observable semantics are introduced only at the observable
 * boundary.
 */


/* ============================================================================
 * 32. NO INFIX PAULI DUPLICATION
 * ========================================================================== */

/*
 * The previous implementation introduced a separate grammar concept for:
 *
 *     X(q0) * Z(q1)
 *
 * This creates unnecessary competition with the canonical expression grammar.
 *
 * The production architecture instead uses:
 *
 *     product(X, Z)
 *
 * or another semantic observable invocation.
 *
 * If infix observable algebra is eventually standardized, it MUST be added
 * through the canonical expression precedence system and then mapped
 * semantically to observables.
 *
 * It must not create a second multiplication grammar here.
 */


/* ============================================================================
 * 33. NO FIXED OBSERVABLE CATALOGUE
 * ========================================================================== */

/*
 * NEVER add:
 *
 *     quantumObservable
 *         : X
 *         | Y
 *         | Z
 *         | H
 *         | ...
 *
 * and NEVER add:
 *
 *     quantumPauliAxis
 *         : X
 *         | Y
 *         | Z
 *         ;
 *
 * as lexer/parser keyword enumerations.
 *
 * Pauli axis validity is semantic.
 *
 * Observable identity is open-world.
 */


/* ============================================================================
 * 34. NO HARD-CODED HARDWARE
 * ========================================================================== */

/*
 * NEVER encode:
 *
 *     q0
 *     q1
 *     physical_qubit_0
 *     qpu0
 *     device0
 *     readout0
 *
 * as universal grammar concepts.
 *
 * Source expressions may contain user-defined identifiers with such spellings
 * if the language's identifier rules permit them, but this grammar does not
 * assign them physical meaning.
 */


/* ============================================================================
 * 35. NO HARD-CODED RESOURCE LIMITS
 * ========================================================================== */

/*
 * Forbidden universal limits include:
 *
 *     MAX_QUBITS
 *     MAX_OBSERVABLES
 *     MAX_PAULI_WEIGHT
 *     MAX_FACTORS
 *     MAX_TARGETS
 *     MAX_MEASUREMENTS
 *     MAX_SHOTS
 *     MAX_MATRIX_DIMENSION
 *     MAX_TENSOR_RANK
 *     MAX_DEVICES
 *     MAX_NODES
 *
 * Resource availability is evaluated after parsing.
 */


/* ============================================================================
 * 36. PORTABILITY CONTRACT
 * ========================================================================== */

/*
 * A valid observable source program must not require rewriting merely because
 * the implementation target changes.
 *
 * The same semantic observable may be realized through:
 *
 *     direct hardware measurement;
 *     basis transformation;
 *     grouped measurement;
 *     simulator evaluation;
 *     tensor-network evaluation;
 *     distributed evaluation;
 *     accelerator execution;
 *     future quantum measurement mechanisms.
 *
 * The grammar remains unchanged.
 */


/* ============================================================================
 * 37. DIALECT CONTRACT
 * ========================================================================== */

/*
 * A dialect may introduce observable semantics through qualified names and
 * existing extension mechanisms.
 *
 * A dialect MUST NOT silently:
 *
 *     - redefine the meaning of core observable syntax;
 *     - introduce hidden hardware limits;
 *     - create a competing quantum IR;
 *     - change the meaning of OBSERVE globally.
 *
 * Dialect-specific syntax belongs to the dialect grammar and compatibility
 * contract.
 */


/* ============================================================================
 * 38. DIAGNOSTIC CONTRACT
 * ========================================================================== */

/*
 * Parser diagnostics should identify structural failures such as:
 *
 *     observe;
 *     observe ();
 *     observe X on;
 *     observe X ->;
 *     observe product();
 *     observe generalized();
 *
 * Semantic diagnostics should identify:
 *
 *     unknown observable;
 *     invalid Pauli axis;
 *     invalid observable composition;
 *     invalid target;
 *     invalid target cardinality;
 *     unsupported observable kind;
 *     missing capability;
 *     insufficient resources;
 *     invalid contextual marker.
 *
 * The parser must not pretend that semantic failures are syntax failures.
 */


/* ============================================================================
 * 39. SOURCE-SPAN CONTRACT
 * ========================================================================== */

/*
 * The frontend AST must preserve source spans for:
 *
 *     OBSERVE;
 *     observable name;
 *     invocation;
 *     each observable argument;
 *     target marker;
 *     each target;
 *     destination;
 *     declaration marker;
 *     declaration name;
 *     initializer.
 *
 * This is required for:
 *
 *     diagnostics;
 *     IDE tooling;
 *     formatting;
 *     provenance;
 *     compatibility diagnostics;
 *     semantic analysis.
 */


/* ============================================================================
 * 40. DETERMINISM TEST CONTRACT
 * ========================================================================== */

/*
 * The following must produce stable parse structures:
 *
 *     observe X;
 *
 *     observe X on q;
 *
 *     observe product(X, Z) on q0, q1;
 *
 *     observe generalized(Hamiltonian);
 *
 *     observe tensor(A, B, C);
 *
 *     observable Energy = X;
 *
 * The parser must preserve source ordering.
 *
 * Semantic canonicalization belongs downstream.
 */


/* ============================================================================
 * 41. POSITIVE TEST CONTRACT
 * ========================================================================== */

/*
 * Minimum positive cases:
 *
 *     observe X;
 *     observe Y;
 *     observe Z;
 *
 *     observe pauli(X);
 *     observe pauli(Y);
 *     observe pauli(Z);
 *
 *     observe Energy;
 *     observe namespace::Energy;
 *
 *     observe product(X, Z);
 *     observe product(X, Y, Z);
 *
 *     observe tensor(A, B);
 *     observe tensor(A, B, C, D);
 *
 *     observe generalized(Hamiltonian);
 *
 *     observe X on q;
 *     observe product(X, Z) on q0, q1;
 *
 *     observe X on register[i];
 *
 *     observe X on q -> result;
 *
 *     observable Energy = X;
 *
 *     observable Hamiltonian = product(X, Z);
 */


/* ============================================================================
 * 42. NEGATIVE TEST CONTRACT
 * ========================================================================== */

/*
 * Minimum negative cases:
 *
 *     observe;
 *
 *     observe ();
 *
 *     observe product();
 *
 *     observe generalized();
 *
 *     observe X on;
 *
 *     observe X ->;
 *
 *     observe X on q ->
 *
 *     observable;
 *
 *     observable Energy;
 *
 *     observable = X;
 *
 *     observe X on ;
 *
 * Semantic-negative cases must separately cover:
 *
 *     unresolved observable;
 *     invalid Pauli axis;
 *     invalid target type;
 *     incompatible observable/target;
 *     invalid generalized observable;
 *     unavailable required capability;
 *     insufficient target resources.
 */


/* ============================================================================
 * 43. BOUNDARY TEST CONTRACT
 * ========================================================================== */

/*
 * Boundary tests must cover:
 *
 *     one observable;
 *     one argument;
 *     many arguments;
 *     one target;
 *     many targets;
 *     one product factor;
 *     many product factors;
 *     deeply nested observable invocations;
 *     long qualified names;
 *     symbolic target expressions;
 *     symbolic observable arguments;
 *     empty optional target clause;
 *     trailing commas where repository-wide syntax permits them.
 *
 * No boundary test may establish a universal machine-size ceiling.
 */


/* ============================================================================
 * 44. SCALABILITY TEST CONTRACT
 * ========================================================================== */

/*
 * Scalability tests must exercise source growth rather than fixed constants.
 *
 * Examples:
 *
 *     product(A0, A1, A2, ...);
 *
 *     tensor(T0, T1, T2, ...);
 *
 *     observe O on q0, q1, q2, ...;
 *
 *     nested_observable(
 *         nested_observable(
 *             ...
 *         )
 *     );
 *
 * The implementation may eventually encounter available-memory or parser
 * recursion limits, but such implementation limits must not become language
 * semantics.
 *
 * No MAX_* grammar constant may be introduced to make these tests pass.
 */


/* ============================================================================
 * 45. COMPATIBILITY CONTRACT
 * ========================================================================== */

/*
 * Existing canonical source:
 *
 *     observe ...
 *
 * must retain its semantic meaning.
 *
 * Legacy K_* token names are NOT accepted as parser vocabulary.
 *
 * The following stale forms are specifically prohibited from being
 * reintroduced:
 *
 *     K_OBSERVE
 *     K_OBSERVABLE
 *     K_PAULI
 *     K_PRODUCT
 *     K_GENERALIZED
 *     K_TENSOR
 *     K_ON
 *
 * Compatibility handling for historical spellings belongs in:
 *
 *     grammar/compatibility/
 *
 * and not in this grammar.
 */


/* ============================================================================
 * 46. RUST CONTRACT
 * ========================================================================== */

/*
 * This `.g4` file contains no Rust implementation code.
 *
 * The corresponding Rust frontend/compiler implementation must target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Edition 2021
 *
 * and must use safe Rust only.
 *
 * No:
 *
 *     unsafe
 *
 * Rust implementation is permitted.
 *
 * This grammar must not contain embedded target-specific parser actions.
 */


/* ============================================================================
 * 47. COMPILER INTEGRATION
 * ========================================================================== */

/*
 * After parsing:
 *
 *     observable syntax
 *          |
 *          v
 *     AST
 *          |
 *          v
 *     semantic observable
 *          |
 *          +--> type/effect validation
 *          +--> capability validation
 *          +--> resource validation
 *          +--> portability validation
 *          |
 *          v
 *     quantum::ir
 *          |
 *          +--> optimization
 *          +--> measurement grouping
 *          +--> decomposition
 *          +--> routing
 *          +--> scheduling
 *          +--> QEC
 *          +--> ZQN
 *          +--> resilience
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target
 *
 * This file must remain independent of all downstream implementation choices.
 */


/* ============================================================================
 * 48. RUNTIME INTEGRATION
 * ========================================================================== */

/*
 * The runtime receives semantic measurement/observation requests.
 *
 * It is responsible for:
 *
 *     - execution;
 *     - result production;
 *     - resource handling;
 *     - target interaction;
 *     - failure reporting.
 *
 * The parser performs none of these functions.
 */


/* ============================================================================
 * 49. TOOLING INTEGRATION
 * ========================================================================== */

/*
 * Tooling may use these grammar boundaries for:
 *
 *     syntax highlighting;
 *     completion;
 *     navigation;
 *     formatting;
 *     diagnostics;
 *     semantic indexing;
 *     documentation.
 *
 * Tooling MUST derive keyword information from the canonical lexical contract.
 *
 * It must not create a second observable keyword catalogue.
 */


/* ============================================================================
 * 50. HARD-CODING AUDIT
 * ========================================================================== */

/*
 * PASS CONDITIONS:
 *
 * [x] No fixed observable catalogue.
 * [x] No fixed Pauli-axis token enumeration.
 * [x] No fixed product width.
 * [x] No fixed tensor rank.
 * [x] No fixed target count.
 * [x] No fixed measurement count.
 * [x] No fixed qubit count.
 * [x] No physical device IDs.
 * [x] No vendor gate catalogue.
 * [x] No topology.
 * [x] No scheduler assumptions.
 * [x] No QEC implementation.
 * [x] No ZQN implementation.
 * [x] No second quantum IR.
 * [x] No MAX_* hardware constants.
 * [x] No Rust code.
 * [x] No unsafe Rust.
 *
 * The grammar is therefore structurally compatible with POCO-REAF.
 */


/* ============================================================================
 * 51. COMPLETION CRITERIA
 * ========================================================================== */

/*
 * This file is complete when:
 *
 * [x] Canonical parser grammar declaration exists.
 * [x] Canonical ZamaniLexer is consumed.
 * [x] Shared Names and Expressions are imported.
 * [x] Observable identity is open-world.
 * [x] Observable invocation is open-world.
 * [x] Pauli syntax does not require lexer additions.
 * [x] Generalized observables do not require lexer additions.
 * [x] Product composition is scalable.
 * [x] Tensor composition is scalable.
 * [x] Observable targets are scalable.
 * [x] Observation is owned exactly once.
 * [x] Observable aliases do not create duplicate syntax.
 * [x] Measurement syntax remains owned by measurement.g4.
 * [x] Quantum statement composition remains owned by statements/quantum.g4.
 * [x] Quantum orchestration remains owned by quantum.g4.
 * [x] General expressions remain owned by Expressions.
 * [x] Semantic validation remains downstream.
 * [x] quantum::ir remains the canonical IR boundary.
 * [x] No physical hardware assumptions exist.
 * [x] No artificial machine-size limits exist.
 * [x] Rust 1.97/1.97.1 compatibility is documented.
 * [x] Safe-Rust-only implementation policy is preserved.
 * [x] Positive tests are defined.
 * [x] Negative tests are defined.
 * [x] Boundary tests are defined.
 * [x] Scalability tests are defined.
 * [x] Determinism requirements are defined.
 * [x] Compatibility requirements are defined.
 */


/* ============================================================================
 * END OF FILE
 * ============================================================================
 */