/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/quantum/states.g4
 *
 * GRAMMAR
 * -------
 * QuantumStatesDomain
 *
 * STATUS
 * ------
 * CANONICAL / PRODUCTION QUANTUM STATE DOMAIN COMPOSITION GRAMMAR
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the canonical SOURCE-LEVEL QUANTUM STATE DOMAIN BOUNDARY.
 *
 * It owns state-domain constructs that are broader than an individual state
 * expression while deliberately delegating the actual state-expression
 * language to:
 *
 *     grammar/quantum/quantum-states.g4
 *
 * This separation is intentional.
 *
 * `quantum-states.g4` owns:
 *
 *     quantumStateExpression
 *     quantumStateAtom
 *     basis-state expressions
 *     symbolic state references
 *     state constructors
 *     superpositions
 *     mixtures
 *     tensor/product state expressions
 *     state transformations
 *     parenthesized state expressions
 *
 * This file owns:
 *
 *     state declarations
 *     state definitions
 *     state aliases
 *     state parameters
 *     state attributes
 *     state-domain composition
 *     state initialization boundaries
 *     state-domain extension points
 *
 * There MUST NOT be another authoritative owner of these productions.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                     canonical lexer
 *                              |
 *                              v
 *                     canonical parser
 *                              |
 *              +---------------+----------------+
 *              |                                |
 *              v                                v
 *      universal grammar                quantum domain
 *                                               |
 *                              +----------------+----------------+
 *                              |                                 |
 *                              v                                 v
 *                       quantum types                  quantum state domain
 *                              |                                 |
 *                              |                       +---------+---------+
 *                              |                       |                   |
 *                              |                       v                   v
 *                              |                state declarations   state expressions
 *                              |                       |                   |
 *                              +-----------------------+-------------------+
 *                                                      |
 *                                                      v
 *                                             domain-neutral AST
 *                                                      |
 *                                                      v
 *                                             semantic analysis
 *                                                      |
 *                         +----------------------------+-------------------+
 *                         |                            |                   |
 *                         v                            v                   v
 *                      type system                resources          capabilities
 *                         |                            |                   |
 *                         +----------------------------+-------------------+
 *                                                      |
 *                                                      v
 *                                          canonical semantic representation
 *                                                      |
 *                                                      v
 *                                                 quantum::ir
 *                                                      |
 *                           +--------------------------+----------------------+
 *                           |                          |                      |
 *                           v                          v                      v
 *                      optimization                 routing              scheduling
 *                           |                          |                      |
 *                           +--------------------------+----------------------+
 *                                                      |
 *                                                      v
 *                                                 QEC / ZQN
 *                                                      |
 *                                                      v
 *                                                    HAL
 *                                                      |
 *                                                      v
 *                                             target realization
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * This file MUST NOT redefine:
 *
 *     quantumStateExpression
 *     quantumStateAtom
 *     quantumBasisStateLiteral
 *     quantumStateReference
 *     quantumStateConstructor
 *     quantumSuperposition
 *     quantumMixture
 *     quantumTensorProduct
 *     quantumStateTransform
 *
 * Those productions are owned by:
 *
 *     grammar/quantum/quantum-states.g4
 *
 * This file composes them.
 *
 * Likewise this file MUST NOT redefine:
 *
 *     quantumType
 *
 * which remains owned by the quantum type subsystem.
 *
 * ============================================================================
 * LANGUAGE PRINCIPLE
 * ============================================================================
 *
 * A state declaration describes source-level computational meaning.
 *
 * It does NOT select:
 *
 *     - a state-vector implementation;
 *     - a density-matrix implementation;
 *     - a tensor-network implementation;
 *     - a simulator;
 *     - a QPU;
 *     - a physical qubit;
 *     - a physical topology;
 *     - a vendor;
 *     - a device;
 *     - a numerical precision;
 *     - a memory layout;
 *     - an execution strategy.
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * State syntax MUST remain target independent.
 *
 * The same source state declaration can be lowered to different realizations
 * depending on available resources and target capabilities.
 *
 * For example:
 *
 *     state input = |0⟩;
 *
 * describes semantic state intent.
 *
 * It does NOT mean:
 *
 *     physical qubit 0
 *
 * and it does NOT select a particular simulator or QPU.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This file imposes NO artificial finite limits on:
 *
 *     - number of state declarations;
 *     - number of state parameters;
 *     - number of state terms;
 *     - number of tensor factors;
 *     - state-expression nesting;
 *     - number of qubits;
 *     - state dimension;
 *     - amplitude count;
 *     - matrix dimension;
 *     - tensor rank;
 *     - number of quantum resources;
 *     - number of declarations;
 *     - number of modules;
 *     - namespace depth.
 *
 * Forbidden language-level assumptions include:
 *
 *     MAX_QUBITS
 *     MAX_STATES
 *     MAX_STATE_DIMENSION
 *     MAX_AMPLITUDES
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *     MAX_STATE_PARAMETERS
 *     MAX_REGISTER_SIZE
 *     MAX_QPU_COUNT
 *     MAX_DEVICE_COUNT
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *
 * "Infinity" means that the language does not impose an artificial machine
 * ceiling.
 *
 * It does not claim that a particular compiler, operating system, simulator,
 * device, or physical machine possesses infinite resources.
 *
 * Resource feasibility is a downstream concern.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * These concepts remain distinct:
 *
 *     state declaration
 *         source semantic construct
 *
 *     requires memory >= required_memory
 *         resource requirement
 *
 *     requires capability("quantum.state_preparation")
 *         capability requirement
 *
 *     prefer accelerator("quantum")
 *         preference
 *
 *     physical state representation
 *         target realization
 *
 * This file owns only the source-level state declaration boundary.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * Canonical lexical composition:
 *
 *     grammar/lexer/tokens.g4
 *
 * Production lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Parser grammars consume the production vocabulary according to the
 * repository's canonical parser composition.
 *
 * This file declares NO lexer rules.
 *
 * Relevant lexical categories include:
 *
 *     QUANTUM_LITERAL
 *     IDENTIFIER
 *     INTEGER_LITERAL
 *     FLOAT_LITERAL
 *
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     LBRACKET
 *     RBRACKET
 *     COMMA
 *     COLON
 *     SEMICOLON
 *     ASSIGN
 *     DOUBLE_COLON
 *     LESS_THAN
 *     GREATER_THAN
 *
 * Quantum state literals are consumed through:
 *
 *     QUANTUM_LITERAL
 *
 * rather than recreated here.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The grammar produces parser structure only.
 *
 * The frontend MUST map declarations into the existing domain-neutral AST
 * architecture.
 *
 * Conceptual mapping:
 *
 *     quantumStateDeclaration
 *         ->
 *     ordinary/domain-neutral declaration node
 *         +
 *     state-specific semantic metadata
 *
 *     quantumStateAlias
 *         ->
 *     ordinary/domain-neutral alias/declaration representation
 *
 *     quantumStateParameterList
 *         ->
 *     ordinary parameter/type representation
 *
 *     quantumStateExpression
 *         ->
 *     existing state-expression AST representation
 *
 * This file MUST NOT introduce:
 *
 *     QuantumStateIR
 *     QuantumStateNode
 *     QuantumStateVectorIR
 *     QuantumDensityMatrixIR
 *     PhysicalStateIR
 *     QubitId
 *     PhysicalQubitId
 *
 * merely to support parsing.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis, not parsing, determines:
 *
 *     - whether a state name is unique;
 *     - whether a referenced state exists;
 *     - whether a state expression has the required type;
 *     - whether dimensions agree;
 *     - whether amplitudes are valid;
 *     - whether probabilities are valid;
 *     - whether normalization is required and satisfied;
 *     - whether a state is pure or mixed;
 *     - whether a state is compatible with an operation;
 *     - whether a state can be materialized;
 *     - whether a symbolic dimension can be resolved;
 *     - whether resources are sufficient;
 *     - whether capabilities are available;
 *     - whether a target supports the required semantics.
 *
 * The grammar MUST NOT perform those checks.
 *
 * ============================================================================
 * STATE DECLARATION MODEL
 * ============================================================================
 *
 * Canonical source form:
 *
 *     state Name = expression;
 *
 * Examples:
 *
 *     state zero = |0⟩;
 *
 *     state one = |1⟩;
 *
 *     state plus = |+⟩;
 *
 *     state initial = prepare(...);
 *
 *     state input = existing_state;
 *
 * The expression is delegated to:
 *
 *     quantumStateExpression
 *
 * This prevents state declarations from creating a second state-expression
 * language.
 *
 * ============================================================================
 * PARAMETERIZED STATES
 * ============================================================================
 *
 * State declarations may have parameters.
 *
 * Examples:
 *
 *     state make_state(n) = expression;
 *
 *     state make_state<T>(value: T) = expression;
 *
 *     state make_state(n: Integer) = expression;
 *
 * The exact type/parameter semantics belong to the universal function/type
 * systems.
 *
 * This grammar only establishes a syntactic boundary.
 *
 * No finite parameter count is encoded.
 *
 * ============================================================================
 * STATE TYPE ANNOTATIONS
 * ============================================================================
 *
 * Optional type annotations may be attached to a state declaration:
 *
 *     state input: quantum<State> = expression;
 *
 *     state input: quantum::State = expression;
 *
 * The actual type syntax remains owned by the universal type system and the
 * quantum type grammar.
 *
 * This file MUST NOT create a second `quantumType` grammar.
 *
 * ============================================================================
 * ATTRIBUTES
 * ============================================================================
 *
 * State declarations may carry ordinary language attributes.
 *
 * Example:
 *
 *     @attribute(...)
 *     state input = |0⟩;
 *
 * Attribute syntax belongs to the canonical attribute grammar.
 *
 * This file only provides the state declaration attachment point.
 *
 * ============================================================================
 * VISIBILITY
 * ============================================================================
 *
 * Visibility modifiers, where supported, remain owned by the universal
 * declaration/modifier system.
 *
 * This grammar MUST NOT duplicate public/private/protected/etc. syntax.
 *
 * ============================================================================
 * STATE ALIASING
 * ============================================================================
 *
 * A named state may be defined in terms of another state:
 *
 *     state initial = ground;
 *
 * or:
 *
 *     state initial = quantum::state::ground;
 *
 * Name resolution remains semantic.
 *
 * No state registry is embedded in the grammar.
 *
 * ============================================================================
 * STATE COMPOSITION
 * ============================================================================
 *
 * Composition belongs to the existing state-expression grammar.
 *
 * Examples:
 *
 *     state pair = tensor(a, b);
 *
 *     state combined = product(a, b);
 *
 *     state mixed = mixture(...);
 *
 *     state evolved = transform(input, operation);
 *
 * This file does not duplicate those expressions.
 *
 * ============================================================================
 * STATE MUTABILITY
 * ============================================================================
 *
 * A state declaration is not automatically interpreted as mutable.
 *
 * Mutation, ownership, borrowing, linearity, and resource lifecycle remain
 * governed by the universal type/effect/resource systems.
 *
 * This prevents state syntax from accidentally defining a second ownership
 * model.
 *
 * ============================================================================
 * STATE RESOURCE BOUNDARY
 * ============================================================================
 *
 * Declaring a state does not necessarily allocate a physical state.
 *
 * For example:
 *
 *     state symbolic = expression;
 *
 * may remain symbolic until later compilation.
 *
 * The semantic/runtime layers decide whether and when materialization is
 * necessary.
 *
 * ============================================================================
 * SYMBOLIC SCALE
 * ============================================================================
 *
 * State declarations may contain symbolic dimensions and parameters wherever
 * the underlying state-expression/type grammar permits them.
 *
 * Examples:
 *
 *     state input = prepare(width);
 *
 *     state input = construct(size);
 *
 * The grammar does not translate symbolic values into Rust `usize`, `u32`,
 * `u64`, machine words, register widths, or physical addresses.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing this grammar must be deterministic.
 *
 * There must be no:
 *
 *     - filesystem access;
 *     - network access;
 *     - hardware discovery;
 *     - runtime evaluation;
 *     - randomness;
 *     - environment inspection;
 *     - backend selection;
 *     - simulator selection;
 *     - target probing.
 *
 * The same source and grammar version must produce the same parse structure.
 *
 * ============================================================================
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * The parser/frontend should preserve:
 *
 *     - declaration source span;
 *     - state name;
 *     - parameter spans;
 *     - optional type span;
 *     - initializer span;
 *     - attribute spans;
 *     - source ordering.
 *
 * Exact source spelling should remain available wherever required for:
 *
 *     - diagnostics;
 *     - formatting;
 *     - IDE/LSP;
 *     - provenance;
 *     - reproducible builds;
 *     - compatibility tooling.
 *
 * ============================================================================
 * ERROR-DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * This grammar deliberately does not use a catch-all expression as a fallback
 * for malformed state declarations.
 *
 * A malformed state declaration should fail near the actual source error.
 *
 * Examples that should produce useful parser diagnostics:
 *
 *     state;
 *
 *     state = |0⟩;
 *
 *     state input;
 *
 *     state input = ;
 *
 *     state input = { malformed };
 *
 * Semantic errors such as duplicate names or invalid normalization remain
 * semantic diagnostics.
 *
 * ============================================================================
 * QUANTUM::IR CONTRACT
 * ============================================================================
 *
 * This file has NO dependency on `quantum::ir`.
 *
 * Required direction:
 *
 *     source
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic state model
 *       |
 *       v
 *     canonical quantum semantic representation
 *       |
 *       v
 *     quantum::ir
 *
 * There must be no reverse dependency from grammar to IR.
 *
 * ============================================================================
 * QEC / ZQN CONTRACT
 * ============================================================================
 *
 * State declarations may eventually be consumed by:
 *
 *     QEC
 *     ZQN
 *     resilience
 *
 * but they do not implement these systems.
 *
 * This grammar MUST NOT define:
 *
 *     error-correction algorithms;
 *     stabilizer decoding;
 *     syndrome extraction;
 *     noise mathematics;
 *     readout mitigation;
 *     fault recovery;
 *     calibration.
 *
 * ============================================================================
 * HARDWARE CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT contain:
 *
 *     physical qubit IDs;
 *     fixed device IDs;
 *     QPU names;
 *     vendor APIs;
 *     coupling maps;
 *     topology definitions;
 *     pulse timings;
 *     physical memory layouts;
 *     fixed register widths;
 *     fixed processor sizes.
 *
 * Those concerns belong to hardware/resource/compiler layers.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This is ANTLR grammar source and contains no embedded Rust actions.
 *
 * The integrating Rust implementation MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * Safe Rust only.
 *
 * No `unsafe` Rust is required or permitted.
 *
 * The grammar itself introduces no Rust implementation dependency.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing state-expression syntax remains owned by:
 *
 *     grammar/quantum/quantum-states.g4
 *
 * Existing state declarations that are promoted into this grammar MUST preserve
 * their semantic meaning unless a language-version migration explicitly
 * changes it.
 *
 * Deprecated forms must be handled by the compatibility subsystem rather than
 * silently changing their interpretation.
 *
 * ============================================================================
 * EXTENSION CONTRACT
 * ============================================================================
 *
 * Future state-domain features must be added through dedicated semantic
 * extensions rather than by turning this file into a catalogue of every
 * possible quantum state representation.
 *
 * Examples of possible future semantic domains include:
 *
 *     pure states
 *     mixed states
 *     stabilizer states
 *     graph states
 *     tensor-network states
 *     encoded states
 *     logical states
 *     distributed states
 *     photonic states
 *     bosonic states
 *     continuous-variable states
 *     measurement-based states
 *     analog states
 *     future quantum state abstractions
 *
 * Their concrete semantics belong to the relevant specification and semantic
 * subsystems.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Canonical lexical layer:
 *
 *     grammar/lexer/tokens.g4
 *
 * Production lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Universal type layer:
 *
 *     grammar/types/
 *
 * Quantum type layer:
 *
 *     grammar/types/quantum.g4
 *
 * Quantum state-expression layer:
 *
 *     grammar/quantum/quantum-states.g4
 *
 * Quantum domain orchestrator:
 *
 *     grammar/quantum/quantum.g4
 *
 * Qubit/resource syntax:
 *
 *     grammar/quantum/qubits.g4
 *     grammar/quantum/quantum-registers.g4
 *     grammar/quantum/logical-qubits.g4
 *     grammar/quantum/physical-qubits.g4
 *
 * Observable syntax:
 *
 *     grammar/quantum/observables.g4
 *
 * Measurement syntax:
 *
 *     grammar/quantum/measurement.g4
 *
 * Resource/capability requirements:
 *
 *     grammar/resources/
 *     grammar/quantum/quantum-resources.g4
 *     grammar/quantum/quantum-capabilities.g4
 *
 * Canonical quantum semantic boundary:
 *
 *     quantum::ir
 *
 * Downstream:
 *
 *     semantic analysis
 *          |
 *          v
 *     canonical semantic model
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
 *          +--> HAL
 *          +--> simulation
 *          |
 *          v
 *     target realization
 *
 * ============================================================================
 * IMPORTANT INTEGRATION RULE
 * ============================================================================
 *
 * `states.g4` MUST be imported/composed exactly once by the quantum grammar
 * aggregation path.
 *
 * It MUST NOT be independently imported as a competing root by arbitrary
 * downstream grammars.
 *
 * Recommended composition:
 *
 *     quantum.g4
 *          |
 *          +--> QuantumStatesDomain
 *          |
 *          +--> QuantumStates
 *          |
 *          +--> Qubits
 *          |
 *          +--> QuantumRegisters
 *          |
 *          +--> Operations
 *          |
 *          +--> Measurement
 *          |
 *          +--> Observables
 *          |
 *          +--> other specialized quantum grammars
 *
 * The universal Zamani parser then composes the quantum orchestrator.
 *
 * ============================================================================
 * PRODUCTION GRAMMAR
 * ============================================================================
 */

parser grammar QuantumStatesDomain;

options {
    tokenVocab = ZamaniTokens;
}


/*
 * ============================================================================
 * 1. PUBLIC STATE-DOMAIN ENTRY
 * ============================================================================
 *
 * This is the only public entry point owned by this file.
 *
 * It composes state declarations and state-domain extensions.
 */
quantumStateDomainElement
    : quantumStateDeclaration
    | quantumStateExtension
    ;


/*
 * ============================================================================
 * 2. STATE DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     state name = expression;
 *
 * Optional components:
 *
 *     visibility
 *     attributes
 *     parameters
 *     type annotation
 *
 * Visibility and attributes are intentionally delegated to the universal
 * declaration infrastructure.
 *
 * If the canonical parser supplies those wrappers at a higher level, this
 * production should be reached after those wrappers have been consumed.
 */
quantumStateDeclaration
    : K_STATE
      IDENTIFIER
      quantumStateParameterClause?
      quantumStateTypeAnnotation?
      ASSIGN
      quantumStateExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 3. STATE PARAMETERS
 * ============================================================================
 *
 * State parameters are source-level symbolic inputs.
 *
 * No maximum number of parameters is encoded.
 *
 * Parameter semantics are shared with the universal function/type system.
 */
quantumStateParameterClause
    : LPAREN
      quantumStateParameterList?
      RPAREN
    ;


quantumStateParameterList
    : quantumStateParameter
      (
          COMMA quantumStateParameter
      )*
      COMMA?
    ;


quantumStateParameter
    : IDENTIFIER
      quantumStateParameterType?
    ;


quantumStateParameterType
    : COLON
      quantumStateTypeReference
    ;


/*
 * ============================================================================
 * 4. OPTIONAL STATE TYPE ANNOTATION
 * ============================================================================
 *
 * Examples:
 *
 *     state input: quantum<State> = ...;
 *
 *     state input: quantum::State = ...;
 *
 * This file does not own general quantum type syntax.
 *
 * The reference is intentionally qualified/open-ended so that the type
 * subsystem remains the owner of complete type semantics.
 */
quantumStateTypeAnnotation
    : COLON
      quantumStateTypeReference
    ;


quantumStateTypeReference
    : quantumStateQualifiedTypeReference
    | IDENTIFIER
    ;


quantumStateQualifiedTypeReference
    : IDENTIFIER
      (
          DOUBLE_COLON IDENTIFIER
      )*
    ;


/*
 * ============================================================================
 * 5. STATE ALIAS
 * ============================================================================
 *
 * A state alias is represented by a normal state declaration whose initializer
 * is a state expression/reference.
 *
 * This explicit production exists only as an integration classification point.
 *
 * Example:
 *
 *     state initial = ground;
 *
 * The semantic layer determines whether the initializer is:
 *
 *     an alias;
 *     a value definition;
 *     a symbolic expression;
 *     a deferred state;
 *     another semantic category.
 *
 * The parser MUST NOT decide this.
 */
quantumStateAlias
    : K_STATE
      IDENTIFIER
      ASSIGN
      quantumStateExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 6. STATE INITIALIZATION
 * ============================================================================
 *
 * State initialization is represented by the ordinary state declaration
 * boundary rather than a second allocation language.
 *
 * This rule exists to provide a semantic extension point for tools that need
 * to classify initialization syntax without changing the declaration grammar.
 */
quantumStateInitialization
    : quantumStateDeclaration
    ;


/*
 * ============================================================================
 * 7. STATE EXPRESSION DELEGATION
 * ============================================================================
 *
 * IMPORTANT:
 *
 * `quantumStateExpression` is owned by:
 *
 *     grammar/quantum/quantum-states.g4
 *
 * It MUST NOT be redefined here.
 *
 * The imported grammar supplies:
 *
 *     |0⟩
 *     |1⟩
 *     |+⟩
 *     |-⟩
 *     symbolic states
 *     constructors
 *     superpositions
 *     mixtures
 *     tensor/product composition
 *     transformations
 *     parenthesized states
 *
 * This delegation is the central anti-duplication rule for this file.
 */


/*
 * ============================================================================
 * 8. STATE-DOMAIN EXTENSION
 * ============================================================================
 *
 * Future state-domain syntax enters through a dedicated extension production.
 *
 * This is deliberately narrow.
 *
 * Do NOT replace it with:
 *
 *     quantumStateExtension : expression ;
 *
 * because that would make arbitrary expressions appear to be valid state
 * declarations and would weaken diagnostics.
 */
quantumStateExtension
    : quantumStateDeclarationExtension
    ;


quantumStateDeclarationExtension
    : K_STATE
      IDENTIFIER
      ASSIGN
      quantumStateExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 9. STATE PROGRAM
 * ============================================================================
 *
 * Zero or more state-domain elements are allowed.
 *
 * No finite declaration count is imposed.
 *
 * The enclosing quantum/program grammar remains responsible for deciding
 * where this sequence is legal.
 */
quantumStateProgram
    : quantumStateDomainElement*
    ;


/*
 * ============================================================================
 * 10. STATE BLOCK
 * ============================================================================
 *
 * A state-domain block provides a reusable structural boundary for future
 * scoped state declarations.
 *
 * Example:
 *
 *     {
 *         state a = |0⟩;
 *         state b = |1⟩;
 *     }
 *
 * Block ownership may ultimately be centralized by the universal block
 * grammar. This production exists only when the quantum orchestrator needs a
 * state-specific composition boundary.
 */
quantumStateBlock
    : LBRACE
      quantumStateDomainElement*
      RBRACE
    ;


/*
 * ============================================================================
 * 11. STATE DECLARATION EXTENSION POINT
 * ============================================================================
 *
 * Future semantic state categories should enter here only when they require
 * syntax that cannot be expressed through the ordinary state declaration plus
 * state expression.
 *
 * Examples could include future source-level constructs for:
 *
 *     encoded state declarations
 *     distributed state declarations
 *     externally supplied state declarations
 *     deferred state declarations
 *
 * Their semantics must be specified before their syntax is promoted.
 */
quantumStateDeclarationExtensionPoint
    : quantumStateDeclarationExtension
    ;


/*
 * ============================================================================
 * 12. NO PHYSICAL STATE ALLOCATION
 * ============================================================================
 *
 * There is intentionally no production such as:
 *
 *     state on qpu0
 *
 *     state on physical_qubit0
 *
 *     state using_device(...)
 *
 *     state_vector_size(1024)
 *
 *     density_matrix_size(2048)
 *
 * Such constructs would mix source semantics with target realization.
 *
 * Physical realization belongs downstream.
 */


/*
 * ============================================================================
 * 13. NO STATE-VECTOR IMPLEMENTATION SYNTAX
 * ============================================================================
 *
 * This grammar does not define:
 *
 *     state_vector
 *     amplitude_array
 *     density_matrix
 *     simulator_buffer
 *     device_memory
 *     accelerator_memory
 *
 * as implementation primitives.
 *
 * Such representations may exist in compiler/runtime layers without becoming
 * part of the portable language syntax.
 */


/*
 * ============================================================================
 * 14. NO NORMALIZATION IMPLEMENTATION
 * ============================================================================
 *
 * The parser does not determine whether:
 *
 *     sum(|amplitude|^2) == 1
 *
 * or any other physical/mathematical validity condition holds.
 *
 * Such validation belongs to semantic analysis.
 */


/*
 * ============================================================================
 * 15. NO DIMENSION LIMIT
 * ============================================================================
 *
 * A state can conceptually have a symbolic or arbitrarily large dimension.
 *
 * The grammar must not introduce:
 *
 *     MAX_DIMENSION
 *     MAX_AMPLITUDES
 *     MAX_RANK
 *
 * or equivalent limits.
 */


/*
 * ============================================================================
 * 16. NO QUBIT LIMIT
 * ============================================================================
 *
 * State syntax does not impose:
 *
 *     MAX_QUBITS
 *
 * or any equivalent restriction.
 *
 * A state expression may depend on a symbolic resource cardinality.
 *
 * Resource feasibility is checked after parsing.
 */


/*
 * ============================================================================
 * 17. NO DEVICE LIMIT
 * ============================================================================
 *
 * The grammar does not assume:
 *
 *     one QPU
 *     two QPUs
 *     eight QPUs
 *     N fixed devices
 *
 * Distributed state realization is downstream.
 */


/*
 * ============================================================================
 * 18. NO VENDOR LIMIT
 * ============================================================================
 *
 * A state name or qualified state name may refer to a library, dialect, or
 * semantic extension.
 *
 * Vendor/device semantics must not be encoded as parser-level assumptions.
 */


/*
 * ============================================================================
 * 19. CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * Quantum state declarations can participate in:
 *
 *     classical computation
 *     hybrid computation
 *     quantum operations
 *     measurement
 *     observables
 *     effects
 *     resources
 *     capabilities
 *     concurrency
 *     distributed computation
 *     data/tensor computation
 *     hardware co-design
 *
 * This file only establishes the state declaration boundary.
 *
 * Example conceptual flow:
 *
 *     state input = |0⟩;
 *
 *             |
 *             v
 *
 *     quantum operation
 *
 *             |
 *             v
 *
 *     measurement
 *
 *             |
 *             v
 *
 *     classical control
 *
 *             |
 *             v
 *
 *     quantum operation
 *
 * The individual constructs remain owned by their respective grammars.
 */


/*
 * ============================================================================
 * 20. INTEROPERABILITY
 * ============================================================================
 *
 * State declarations may eventually be imported/exported through:
 *
 *     OpenQASM
 *     QIR
 *     simulator formats
 *     quantum IR adapters
 *     domain-specific formats
 *
 * Those formats are interoperability mechanisms.
 *
 * They do not replace the canonical Zamani semantic representation.
 */


/*
 * ============================================================================
 * 21. SECURITY
 * ============================================================================
 *
 * This grammar has no side effects.
 *
 * It performs no:
 *
 *     filesystem access
 *     network access
 *     process execution
 *     hardware access
 *     backend selection
 *     simulator execution
 *     code execution
 *     credential access
 *
 * Source text remains data until semantic/compiler stages process it.
 */


/*
 * ============================================================================
 * 22. PERFORMANCE
 * ============================================================================
 *
 * The grammar avoids:
 *
 *     semantic predicates requiring external state;
 *     embedded actions;
 *     hardware probing;
 *     network operations;
 *     filesystem operations;
 *     target-dependent decisions.
 *
 * Repeated state declarations are represented by ordinary ANTLR repetition.
 *
 * Practical parser limits remain implementation constraints rather than
 * language semantics.
 */


/*
 * ============================================================================
 * 23. DETERMINISM
 * ============================================================================
 *
 * The same:
 *
 *     source
 *     lexer version
 *     grammar version
 *     parser configuration
 *
 * MUST produce the same parse structure.
 *
 * No randomness or runtime-dependent branch is permitted.
 */


/*
 * ============================================================================
 * 24. COMPATIBILITY
 * ============================================================================
 *
 * Existing:
 *
 *     grammar/quantum/quantum-states.g4
 *
 * remains the owner of state-expression syntax.
 *
 * Existing:
 *
 *     grammar/types/quantum.g4
 *
 * remains the owner of quantum type syntax.
 *
 * Existing:
 *
 *     grammar/quantum/quantum.g4
 *
 * remains the quantum orchestrator.
 *
 * Existing state expressions must not be duplicated in this file.
 *
 * ============================================================================
 * 25. TEST CONTRACT
 * ============================================================================
 *
 * Positive tests
 * --------------
 *
 *     state zero = |0⟩;
 *     state one = |1⟩;
 *     state plus = |+⟩;
 *     state minus = |-⟩;
 *     state initial = zero;
 *     state initial = quantum::state::ground;
 *     state pair = tensor(zero, one);
 *     state mixed = mixture(...);
 *     state evolved = transform(initial, operation);
 *
 * Parameterized:
 *
 *     state make_state(n) = expression;
 *     state make_state(n: Integer) = expression;
 *
 * Typed:
 *
 *     state input: quantum::State = expression;
 *
 * Generic/qualified semantic names:
 *
 *     state input: quantum::StateType = expression;
 *
 * Negative syntax tests
 * ---------------------
 *
 *     state;
 *     state = |0⟩;
 *     state input = ;
 *     state input
 *     state input = ;
 *
 * Semantic negative tests belong downstream:
 *
 *     duplicate state name
 *     unresolved state reference
 *     invalid state type
 *     invalid dimensions
 *     invalid normalization
 *     invalid amplitude domain
 *     unavailable resource
 *     unavailable capability
 *
 * Boundary tests
 * --------------
 *
 *     empty state-domain block
 *     one state declaration
 *     many state declarations
 *     deeply nested state expressions
 *     symbolic state parameters
 *     qualified names
 *     large symbolic dimensions
 *
 * Scalability tests
 * -----------------
 *
 * Test increasing source/state complexity without defining a language-level
 * maximum.
 *
 * The tests must verify:
 *
 *     no fixed state count
 *     no fixed qubit count
 *     no fixed tensor rank
 *     no fixed state dimension
 *     no fixed namespace depth
 *
 * Determinism tests
 * -----------------
 *
 * Reparse identical source and verify equivalent parse structures.
 *
 * Compatibility tests
 * -------------------
 *
 * Existing accepted state-expression syntax must continue to parse through
 * `quantum-states.g4`.
 *
 * ============================================================================
 * 26. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST pass the following audit:
 *
 * [x] no MAX_QUBITS
 * [x] no MAX_STATES
 * [x] no MAX_STATE_DIMENSION
 * [x] no MAX_AMPLITUDES
 * [x] no MAX_TENSOR_RANK
 * [x] no MAX_REGISTER_SIZE
 * [x] no MAX_QPU_COUNT
 * [x] no MAX_DEVICE_COUNT
 * [x] no physical qubit IDs
 * [x] no device IDs
 * [x] no vendor IDs
 * [x] no topology
 * [x] no fixed memory size
 * [x] no fixed register width
 * [x] no simulator-specific representation
 * [x] no hardware allocation
 * [x] no QEC implementation
 * [x] no ZQN implementation
 * [x] no routing
 * [x] no scheduling
 * [x] no optimization
 * [x] no Rust actions
 * [x] no unsafe code
 *
 * ============================================================================
 * 27. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is considered complete when:
 *
 * [x] state-domain ownership is explicit;
 * [x] state-expression ownership remains in quantum-states.g4;
 * [x] quantum-type ownership remains in types/quantum.g4;
 * [x] quantum.g4 remains the orchestrator;
 * [x] no duplicate quantumStateExpression exists;
 * [x] no lexer rules are introduced;
 * [x] no hardware limits are introduced;
 * [x] no target-specific assumptions are introduced;
 * [x] AST ownership is domain neutral;
 * [x] semantic validation remains downstream;
 * [x] quantum::ir remains the canonical quantum IR;
 * [x] QEC remains downstream;
 * [x] ZQN remains downstream;
 * [x] routing remains downstream;
 * [x] scheduling remains downstream;
 * [x] resource/capability analysis remains downstream;
 * [x] deterministic parsing is preserved;
 * [x] source spans can be preserved;
 * [x] positive/negative/boundary/scalability/determinism tests are defined;
 * [x] Rust integration requires Rust 1.97/1.97.1 and safe Rust only.
 *
 * ============================================================================
 * END
 * ============================================================================
 */