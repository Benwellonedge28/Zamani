/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/quantum-states.g4
 *
 * Purpose:
 *     Canonical reusable parser grammar for SOURCE-LEVEL QUANTUM STATE
 *     EXPRESSIONS.
 *
 * Language:
 *     Zamani
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniTokens
 *          |
 *          v
 *     canonical parser
 *          |
 *          +---- quantum-states.g4
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical quantum semantic representation
 *          |
 *          v
 *     quantum::ir
 *          |
 *          +---- optimization
 *          +---- routing
 *          +---- scheduling
 *          +---- ZQN
 *          +---- QEC
 *          +---- resilience
 *          +---- hardware HAL
 *          +---- simulation
 *          |
 *          v
 *     target realization
 *
 * This grammar is ONLY a syntax layer.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - quantum state expression syntax;
 *   - basis-state syntax;
 *   - symbolic state references;
 *   - state constructors;
 *   - state superposition syntax;
 *   - state mixture syntax;
 *   - tensor/product state composition;
 *   - state transformation syntax;
 *   - recursive state-expression composition;
 *   - state-expression argument lists;
 *   - state-level syntactic extension boundaries.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - qubit declarations;
 *   - quantum register declarations;
 *   - quantum type declarations;
 *   - quantum gates;
 *   - quantum operations;
 *   - measurements;
 *   - observables;
 *   - circuits;
 *   - logical qubits;
 *   - physical qubits;
 *   - QEC algorithms;
 *   - QEC codes;
 *   - QEC decoders;
 *   - noise models;
 *   - ZQN;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - hardware discovery;
 *   - hardware topology;
 *   - calibration;
 *   - backend selection;
 *   - simulator selection;
 *   - runtime allocation;
 *   - state-vector allocation;
 *   - density-matrix allocation;
 *   - state normalization;
 *   - numerical evaluation;
 *   - quantum::ir.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * The grammar recognizes STRUCTURE.
 *
 * It does not determine whether a state expression is:
 *
 *   - normalized;
 *   - physically realizable;
 *   - dimensionally valid;
 *   - pure;
 *   - mixed;
 *   - stabilizer-compatible;
 *   - efficiently simulatable;
 *   - representable by a target;
 *   - supported by a backend;
 *   - supported by available resources.
 *
 * Those decisions belong to semantic analysis, resource analysis, quantum
 * compilation, simulation, and runtime systems.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A state expression describes computational intent.
 *
 * It MUST NOT encode:
 *
 *   - physical qubit identifiers;
 *   - physical device identifiers;
 *   - QPU topology;
 *   - simulator dimensions;
 *   - fixed state-vector sizes;
 *   - fixed density-matrix sizes;
 *   - maximum qubit counts;
 *   - maximum amplitude counts;
 *   - fixed hardware precision;
 *   - fixed memory capacity.
 *
 * Example:
 *
 *     |0⟩
 *
 * means an abstract computational basis state.
 *
 * It does NOT mean:
 *
 *     physical qubit 0
 *
 * Likewise:
 *
 *     superposition(...)
 *
 * does not prescribe a particular simulator or physical realization.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar intentionally uses recursive and repeated productions.
 *
 * There is no grammar-level limit on:
 *
 *   - number of state terms;
 *   - number of tensor factors;
 *   - nesting depth;
 *   - symbolic expression size;
 *   - state-constructor argument count;
 *   - number of quantum resources;
 *   - number of qubits.
 *
 * Practical implementation limits may exist in ANTLR, memory, compiler,
 * operating-system, target, or runtime environments. Those are NOT language
 * semantics and MUST NOT be encoded as grammar constants.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The authoritative lexical vocabulary is:
 *
 *     grammar/lexer/tokens.g4
 *
 * with:
 *
 *     lexer grammar ZamaniTokens;
 *
 * Relevant tokens include:
 *
 *     QUANTUM_LITERAL
 *     IDENTIFIER
 *     INTEGER_LITERAL
 *     FLOAT_LITERAL
 *     LPAREN
 *     RPAREN
 *     LBRACKET
 *     RBRACKET
 *     LBRACE
 *     RBRACE
 *     COMMA
 *     COLON
 *     ASSIGN
 *     PLUS
 *     MINUS
 *     STAR
 *     SLASH
 *     PERCENT
 *     PIPE
 *     LESS_THAN
 *     GREATER_THAN
 *     DOUBLE_COLON
 *     THIN_ARROW
 *
 * This file MUST NOT declare lexer rules.
 *
 * ============================================================================
 * IMPORTANT LEXER COMPATIBILITY RULE
 * ============================================================================
 *
 * The current lexer already exposes:
 *
 *     QUANTUM_LITERAL
 *
 * for the compact basis-state forms:
 *
 *     |0⟩
 *     |1⟩
 *     |+⟩
 *     |-⟩
 *
 * Therefore this grammar consumes QUANTUM_LITERAL directly.
 *
 * The grammar MUST NOT recreate that lexer rule.
 *
 * The generic PIPE token remains useful as an extension boundary for future
 * parser-level state syntax, but this file does not require a second competing
 * representation of the existing basis literal.
 *
 * ============================================================================
 * NO DUPLICATE QUANTUM STATE RULE
 * ============================================================================
 *
 * This file becomes the canonical owner of:
 *
 *     quantumStateExpression
 *
 * Existing quantum grammar files that currently define that rule must delegate
 * to this grammar rather than maintain an independent implementation.
 *
 * In particular, the existing:
 *
 *     grammar/quantum/quantum.g4
 *
 * contains a quantumStateExpression rule.
 *
 * That rule must be removed from its ownership role and replaced by an import /
 * delegation to this grammar.
 *
 * The older:
 *
 *     grammar/antlr/Quantum.g4
 *
 * also contains quantum literal/state-expression rules.
 *
 * Those rules are legacy compatibility material and must not become a second
 * canonical state grammar.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser produces syntax.
 *
 * AST construction belongs to the frontend.
 *
 * The AST should preserve enough structure to distinguish:
 *
 *     basis state
 *     named state
 *     parameterized state
 *     superposition
 *     mixture
 *     tensor/product composition
 *     state transformation
 *     parenthesized state
 *
 * The AST MUST NOT contain target-specific allocation information merely
 * because a state expression was parsed.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT construct:
 *
 *     QuantumStateIR
 *     QuantumStateNode
 *     StateVectorIR
 *     DensityMatrixIR
 *     PhysicalStateIR
 *
 * unless such concepts already exist as canonical semantic representations
 * downstream.
 *
 * Syntax is lowered through the frontend semantic layer into the existing
 * canonical quantum representation and ultimately quantum::ir.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust implementation code.
 *
 * Generated parser/runtime integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * The repository must prohibit unsafe Rust.
 *
 * The grammar itself requires no unsafe code.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * PARSER GRAMMAR
 * ============================================================================
 */

parser grammar QuantumStates;

options {
    tokenVocab = ZamaniTokens;
}


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Canonical entry point for source-level quantum state expressions.
 *
 * Examples:
 *
 *     |0⟩
 *     |1⟩
 *     |+⟩
 *     |-⟩
 *
 *     psi
 *
 *     state(|0⟩)
 *
 *     superposition(...)
 *
 *     mixture(...)
 *
 *     tensor(...)
 *
 *     product(...)
 *
 * The grammar remains extensible without requiring one keyword for every
 * future state representation.
 */
quantumStateExpression
    : quantumStateAtom
      quantumStatePostfix*
    ;


/*
 * ============================================================================
 * 2. STATE ATOMS
 * ============================================================================
 *
 * A state atom is the smallest syntactic state unit.
 *
 * It can be:
 *
 *     - an existing lexer-provided basis literal;
 *     - a symbolic state reference;
 *     - a state constructor;
 *     - a superposition;
 *     - a mixture;
 *     - a tensor/product expression;
 *     - a transformed state;
 *     - a parenthesized state.
 */
quantumStateAtom
    : quantumBasisStateLiteral
    | quantumStateReference
    | quantumStateConstructor
    | quantumSuperposition
    | quantumMixture
    | quantumTensorProduct
    | quantumStateTransform
    | quantumParenthesizedState
    ;


/*
 * ============================================================================
 * 3. BASIS STATE
 * ============================================================================
 *
 * The lexer already recognizes compact canonical basis-state literals.
 *
 * Examples:
 *
 *     |0⟩
 *     |1⟩
 *     |+⟩
 *     |-⟩
 *
 * This rule deliberately delegates lexical recognition to ZamaniTokens.
 */
quantumBasisStateLiteral
    : QUANTUM_LITERAL
    ;


/*
 * ============================================================================
 * 4. SYMBOLIC STATE REFERENCE
 * ============================================================================
 *
 * Examples:
 *
 *     psi
 *     initial_state
 *     algorithm::input_state
 *     quantum::state::ground
 *
 * The parser does not determine whether the name refers to:
 *
 *     - a declared state;
 *     - a parameter;
 *     - a constant;
 *     - a library state;
 *     - a compiler intrinsic;
 *     - a future quantum abstraction.
 *
 * Name resolution belongs downstream.
 */
quantumStateReference
    : quantumStateName
    ;


quantumStateName
    : IDENTIFIER
      (
          DOUBLE_COLON IDENTIFIER
      )*
    ;


/*
 * ============================================================================
 * 5. GENERIC STATE CONSTRUCTOR
 * ============================================================================
 *
 * Generic constructor form:
 *
 *     state(...)
 *     state(argument)
 *     state(argument, argument)
 *
 * or a qualified constructor:
 *
 *     quantum::state(...)
 *     domain::quantum::state(...)
 *
 * The constructor name is deliberately not a closed keyword list.
 *
 * This allows future state representations without continually expanding the
 * core lexer.
 */
quantumStateConstructor
    : quantumStateConstructorName
      LPAREN
      quantumStateArgumentList?
      RPAREN
    ;


quantumStateConstructorName
    : quantumStateName
    ;


/*
 * ============================================================================
 * 6. STATE ARGUMENT LIST
 * ============================================================================
 *
 * No fixed number of state arguments is encoded.
 *
 * Arguments may be:
 *
 *     state expressions;
 *     ordinary expressions;
 *     type expressions;
 *     symbolic parameters;
 *     nested constructors.
 *
 * Semantic analysis determines whether the arguments are valid for the
 * selected constructor.
 */
quantumStateArgumentList
    : quantumStateArgument
      (
          COMMA quantumStateArgument
      )*
      COMMA?
    ;


quantumStateArgument
    : quantumStateArgumentState
    | quantumStateArgumentExpression
    ;


quantumStateArgumentState
    : quantumStateExpression
    ;


quantumStateArgumentExpression
    : expression
    ;


/*
 * ============================================================================
 * 7. SUPERPOSITION
 * ============================================================================
 *
 * Explicit superposition syntax.
 *
 * Canonical structural form:
 *
 *     superposition(
 *         term,
 *         term,
 *         ...
 *     )
 *
 * Terms are intentionally generic.
 *
 * The grammar does not decide:
 *
 *     - amplitude representation;
 *     - numeric precision;
 *     - normalization;
 *     - basis dimension;
 *     - state-vector storage;
 *     - simulator implementation.
 */
quantumSuperposition
    : quantumSuperpositionName
      LPAREN
      quantumSuperpositionTermList?
      RPAREN
    ;


quantumSuperpositionName
    : quantumStateName
    ;


quantumSuperpositionTermList
    : quantumSuperpositionTerm
      (
          COMMA quantumSuperpositionTerm
      )*
      COMMA?
    ;


quantumSuperpositionTerm
    : quantumAmplitude
      quantumStateExpression
    | quantumStateExpression
    ;


/*
 * ============================================================================
 * 8. AMPLITUDES
 * ============================================================================
 *
 * An amplitude is syntax-level data.
 *
 * Examples:
 *
 *     a
 *     alpha
 *     1
 *     0.5
 *     theta
 *     expression
 *
 * Complex-number semantics belong to the type/semantic layer.
 *
 * The grammar deliberately does not define a machine floating-point format.
 */
quantumAmplitude
    : expression
    ;


/*
 * ============================================================================
 * 9. MIXTURE
 * ============================================================================
 *
 * Mixture syntax is represented independently from superposition syntax.
 *
 * This distinction is important because a mixed state and a coherent
 * superposition are semantically different even though both may contain
 * multiple component states.
 *
 * Canonical structural form:
 *
 *     mixture(term, term, ...)
 *
 * The semantic layer determines:
 *
 *     - probabilities;
 *     - normalization;
 *     - density representation;
 *     - physical validity.
 */
quantumMixture
    : quantumMixtureName
      LPAREN
      quantumMixtureTermList?
      RPAREN
    ;


quantumMixtureName
    : quantumStateName
    ;


quantumMixtureTermList
    : quantumMixtureTerm
      (
          COMMA quantumMixtureTerm
      )*
      COMMA?
    ;


quantumMixtureTerm
    : quantumProbability
      quantumStateExpression
    | quantumStateExpression
    ;


quantumProbability
    : expression
    ;


/*
 * ============================================================================
 * 10. TENSOR / PRODUCT COMPOSITION
 * ============================================================================
 *
 * Quantum systems can be composed.
 *
 * Canonical structural forms:
 *
 *     tensor(state_a, state_b)
 *
 *     product(state_a, state_b, state_c)
 *
 * No fixed number of tensor factors is encoded.
 */
quantumTensorProduct
    : quantumTensorProductName
      LPAREN
      quantumTensorFactorList?
      RPAREN
    ;


quantumTensorProductName
    : quantumStateName
    ;


quantumTensorFactorList
    : quantumStateExpression
      (
          COMMA quantumStateExpression
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 11. STATE TRANSFORMATION
 * ============================================================================
 *
 * State transformations are represented structurally.
 *
 * Examples:
 *
 *     transform(state, operation)
 *
 *     evolve(state, parameter)
 *
 *     prepare(state)
 *
 * The grammar does not define the physical realization.
 */
quantumStateTransform
    : quantumStateTransformName
      LPAREN
      quantumStateTransformArguments?
      RPAREN
    ;


quantumStateTransformName
    : quantumStateName
    ;


quantumStateTransformArguments
    : quantumStateTransformArgument
      (
          COMMA quantumStateTransformArgument
      )*
      COMMA?
    ;


quantumStateTransformArgument
    : quantumStateExpression
    | expression
    ;


/*
 * ============================================================================
 * 12. PARENTHESIZED STATE
 * ============================================================================
 *
 * Parentheses preserve source grouping.
 *
 * They do not imply allocation or materialization.
 */
quantumParenthesizedState
    : LPAREN
      quantumStateExpression
      RPAREN
    ;


/*
 * ============================================================================
 * 13. STATE POSTFIXES
 * ============================================================================
 *
 * Postfix syntax provides an extension point for future state-level language
 * constructs while keeping the core grammar small.
 *
 * At present the postfix is intentionally conservative.
 *
 * A future semantic extension can add additional postfix forms without
 * changing the representation of existing basis/state constructors.
 */
quantumStatePostfix
    : quantumStateAnnotation
    ;


quantumStateAnnotation
    : AT
      quantumStateAnnotationName
      (
          LPAREN
          quantumStateArgumentList?
          RPAREN
      )?
    ;


quantumStateAnnotationName
    : quantumStateName
    ;


/*
 * ============================================================================
 * 14. STATE LITERALS AS GENERIC EXPRESSIONS
 * ============================================================================
 *
 * The canonical basis-state token is intentionally separate from ordinary
 * identifiers and numeric literals.
 *
 * This gives the AST a deterministic syntactic boundary:
 *
 *     QUANTUM_LITERAL
 *
 * is unambiguously a quantum-state literal.
 *
 * It must not be interpreted as:
 *
 *     a physical qubit identifier;
 *     a device identifier;
 *     a register index.
 */


/*
 * ============================================================================
 * 15. STATE COMPOSITION
 * ============================================================================
 *
 * This rule exists as a named semantic boundary for consumers that need to
 * recognize recursively compositional state syntax.
 *
 * It does not introduce a second representation.
 */
quantumStateComposition
    : quantumStateExpression
    ;


/*
 * ============================================================================
 * 16. STATE PARAMETER
 * ============================================================================
 *
 * State parameters may be classical or quantum depending on semantic context.
 *
 * The grammar deliberately accepts ordinary expressions.
 */
quantumStateParameter
    : expression
    ;


/*
 * ============================================================================
 * 17. STATE DIMENSION / CARDINALITY INTENT
 * ============================================================================
 *
 * If a state constructor requires a symbolic dimension, it is expressed as
 * an ordinary expression.
 *
 * Examples:
 *
 *     state_dimension(n)
 *
 *     state_space(width)
 *
 * The grammar never evaluates the expression and never converts it to a
 * machine-sized integer.
 */
quantumStateDimensionExpression
    : expression
    ;


/*
 * ============================================================================
 * 18. STATE EXTENSION BOUNDARY
 * ============================================================================
 *
 * Future quantum state forms should preferentially use:
 *
 *     named constructors;
 *     qualified names;
 *     nested state expressions;
 *     ordinary expressions;
 *     annotations;
 *
 * rather than adding one new lexer keyword for every state representation.
 *
 * This preserves long-term language extensibility.
 */
quantumStateExtension
    : quantumStateConstructor
    | quantumStateReference
    | quantumStateAnnotation
    ;


/*
 * ============================================================================
 * 19. SEMANTIC NOTES
 * ============================================================================
 *
 * The following are intentionally NOT grammar errors:
 *
 *     state()
 *     state(a, b, c, ...)
 *
 * unless the selected semantic constructor requires a different arity.
 *
 * Likewise, the parser does not determine whether:
 *
 *     superposition(a, |0⟩, b, |1⟩)
 *
 * is normalized.
 *
 * Nor does it determine whether:
 *
 *     mixture(p, psi)
 *
 * has a valid probability.
 *
 * Nor does it determine whether:
 *
 *     tensor(psi, phi)
 *
 * can be materialized by the target.
 *
 * Those are semantic/resource/target questions.
 *
 * ============================================================================
 * 20. MACHINE-INDEPENDENCE GUARANTEE
 * ============================================================================
 *
 * No production in this file contains:
 *
 *     MAX_QUBITS
 *     MAX_STATES
 *     MAX_AMPLITUDES
 *     MAX_STATE_VECTOR_SIZE
 *     MAX_DENSITY_MATRIX_SIZE
 *     MAX_TENSOR_RANK
 *     MAX_STATE_TERMS
 *     MAX_ARGUMENTS
 *     MAX_NESTING
 *     DEVICE_ID
 *     QPU_ID
 *     PHYSICAL_QUBIT
 *     TOPOLOGY
 *     VENDOR
 *
 * The absence of these constructs is intentional.
 *
 * ============================================================================
 * 21. ERROR OWNERSHIP
 * ============================================================================
 *
 * Syntax errors belong to the parser/frontend diagnostic layer.
 *
 * Examples:
 *
 *     missing ')'
 *     malformed constructor
 *     missing state argument
 *     invalid comma placement
 *
 * Semantic errors belong downstream.
 *
 * Examples:
 *
 *     invalid normalization
 *     incompatible state dimension
 *     invalid probability
 *     unsupported state representation
 *     unavailable target resources
 *     unsupported backend capability
 *
 * Runtime failures belong runtime/resilience.
 *
 * This file MUST NOT encode downstream errors as parser hacks.
 *
 * ============================================================================
 * 22. DETERMINISM
 * ============================================================================
 *
 * The grammar must have deterministic parse behavior under the canonical
 * Zamani parser configuration.
 *
 * Ambiguous interpretation between a named state and a named constructor is
 * resolved structurally:
 *
 *     identifier
 *
 * is a state reference.
 *
 *     identifier(...)
 *
 * is a constructor/call-like state expression.
 *
 * Semantic analysis determines the meaning of the resolved symbol.
 *
 * ============================================================================
 * 23. COMPATIBILITY
 * ============================================================================
 *
 * Existing source syntax:
 *
 *     |0⟩
 *     |1⟩
 *     |+⟩
 *     |-⟩
 *
 * remains supported through QUANTUM_LITERAL.
 *
 * The grammar does not require changes to the lexical spelling of those
 * literals.
 *
 * Future state syntax should be introduced through versioned language
 * compatibility policy rather than silently changing the meaning of an
 * existing state expression.
 *
 * ============================================================================
 * 24. INTEGRATION CHECKLIST
 * ============================================================================
 *
 * Before this file is considered COMPLETE:
 *
 *   [ ] Imported by the canonical Zamani parser.
 *
 *   [ ] `quantumStateExpression` has exactly one canonical owner.
 *
 *   [ ] Existing duplicate quantumStateExpression definitions are removed
 *       from competing quantum parser fragments or converted into delegates.
 *
 *   [ ] `QUANTUM_LITERAL` comes exclusively from ZamaniTokens.
 *
 *   [ ] No lexer rules are duplicated here.
 *
 *   [ ] No physical-machine assumptions exist.
 *
 *   [ ] No fixed state-vector limits exist.
 *
 *   [ ] No fixed qubit limits exist.
 *
 *   [ ] AST lowering has a documented mapping for every state production.
 *
 *   [ ] Semantic validation is downstream from parsing.
 *
 *   [ ] quantum::ir remains the canonical quantum semantic boundary.
 *
 *   [ ] QEC does not depend directly on parser productions.
 *
 *   [ ] ZQN does not depend directly on parser productions.
 *
 *   [ ] Routing does not depend directly on parser productions.
 *
 *   [ ] Scheduling does not depend directly on parser productions.
 *
 *   [ ] Hardware discovery does not depend directly on parser productions.
 *
 *   [ ] Runtime does not parse source-level state syntax.
 *
 *   [ ] Positive tests exist.
 *
 *   [ ] Negative tests exist.
 *
 *   [ ] Boundary/scalability tests exist.
 *
 *   [ ] Cross-domain tests exist.
 *
 *   [ ] Determinism tests exist.
 *
 * ============================================================================
 */