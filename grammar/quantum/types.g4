/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/types.g4
 *
 * Grammar:
 *     QuantumTypes
 *
 * Status:
 *     CANONICAL QUANTUM SOURCE-TYPE LEAF
 *
 * Purpose:
 *     Own the source-level syntax that is intrinsically quantum-specific:
 *
 *         - abstract qubit type;
 *         - logical-qubit type;
 *         - scalable quantum-resource collections;
 *         - symbolic quantum cardinality;
 *         - quantum type qualification;
 *         - open quantum type constructors where they are genuinely
 *           quantum-specific.
 *
 * This file is deliberately NOT the universal type-system owner.
 *
 * Universal type composition remains owned by:
 *
 *     grammar/types/types.g4
 *
 * Universal generic application remains owned by the universal type grammar.
 *
 * Quantum declaration syntax remains owned by:
 *
 *     grammar/quantum/qubits.g4
 *     grammar/quantum/quantum-registers.g4
 *     grammar/quantum/logical-qubits.g4
 *     grammar/quantum/physical-qubits.g4
 *
 * Quantum operations remain owned by:
 *
 *     grammar/quantum/operations.g4
 *     grammar/quantum/controlled-operations.g4
 *     grammar/quantum/parameterized-operations.g4
 *
 * Quantum IR remains owned by:
 *
 *     quantum::ir
 *
 * ============================================================================
 * ARCHITECTURAL CONTRACT
 * ============================================================================
 *
 * Source
 *   |
 *   v
 * canonical lexer
 *   |
 *   v
 * canonical parser
 *   |
 *   v
 * QuantumTypes
 *   |
 *   v
 * domain-neutral frontend AST / TypeExpr
 *   |
 *   v
 * semantic type analysis
 *   |
 *   +---------------------+-----------------------+
 *   |                     |                       |
 *   v                     v                       v
 * type/resource       capability/effect       quantum semantics
 * analysis             analysis
 *   |                     |                       |
 *   +---------------------+-----------------------+
 *                         |
 *                         v
 *                  canonical semantic model
 *                         |
 *                         v
 *                     quantum::ir
 *                         |
 *              +----------+----------+
 *              |          |         |
 *              v          v         v
 *          optimize    route     schedule
 *              |          |         |
 *              +----------+---------+
 *                         |
 *                         v
 *                   QEC / resilience
 *                         |
 *                         v
 *                        ZQN
 *                         |
 *                         v
 *                        HAL
 *                         |
 *                         v
 *                  target realization
 *
 * This grammar never constructs or depends on quantum::ir directly.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - quantumType;
 *   - quantumPrimitiveType;
 *   - logicalQubitType;
 *   - quantumCollectionType;
 *   - quantumCollectionElementType;
 *   - quantumCardinalityExpression;
 *   - quantumCardinalityAdditiveExpression;
 *   - quantumCardinalityMultiplicativeExpression;
 *   - quantumCardinalityUnaryExpression;
 *   - quantumCardinalityPrimary;
 *   - quantumQualifiedType;
 *   - quantumQualifiedName;
 *   - quantumQuantumConstructorType;
 *   - quantumTypeValueArgument;
 *   - quantumTypeArgumentList;
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - universal typeExpression;
 *   - primitive classical types;
 *   - universal genericType;
 *   - universal tupleType;
 *   - universal arrayType;
 *   - universal sliceType;
 *   - universal referenceType;
 *   - universal pointerType;
 *   - universal functionType;
 *   - universal Result/Option types;
 *   - ownership/borrowing;
 *   - declarations;
 *   - registers;
 *   - operations;
 *   - gates;
 *   - measurements;
 *   - states;
 *   - observables;
 *   - QEC;
 *   - ZQN;
 *   - routing;
 *   - scheduling;
 *   - calibration;
 *   - hardware;
 *   - target selection;
 *   - backend selection;
 *   - runtime representation.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * The production parser must expose exactly one quantum-specific type owner.
 *
 * The existing historical/duplicate files:
 *
 *     grammar/quantum/quantum-types.g4
 *     grammar/types/quantum.g4
 *
 * MUST NOT both remain production owners of `quantumType`.
 *
 * Integration target:
 *
 *     grammar/quantum/types.g4
 *             |
 *             v
 *     grammar/quantum/quantum.g4
 *             |
 *             v
 *     grammar/antlr/ZamaniParser.g4
 *
 * `grammar/types/types.g4` remains the universal type composition owner.
 *
 * During migration, the old files may remain in the repository as explicitly
 * deprecated compatibility/design artifacts, but they MUST NOT be imported
 * into the same generated parser together with this file.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file declares NO lexer rules.
 *
 * The canonical parser-facing vocabulary is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The parser-facing token vocabulary is therefore:
 *
 *     ZamaniLexer
 *
 * This corrects the former inconsistency where quantum leaf grammars referred
 * directly to ZamaniTokens while the canonical parser uses ZamaniLexer.
 *
 * Required lexical categories:
 *
 *     K_QUBIT
 *     K_LOGICAL
 *     K_QUANTUM
 *
 *     IDENTIFIER
 *     INTEGER_LITERAL
 *
 *     LBRACKET
 *     RBRACKET
 *     LPAREN
 *     RPAREN
 *     COMMA
 *     DOUBLE_COLON
 *
 *     PLUS
 *     MINUS
 *     STAR
 *     SLASH
 *     PERCENT
 *
 * No token representing an individual quantum gate is required.
 *
 * ============================================================================
 * CRITICAL TYPE-SYSTEM BOUNDARY
 * ============================================================================
 *
 * This file deliberately does NOT redefine the entire universal type system.
 *
 * In particular, it does NOT define:
 *
 *     typeExpression
 *
 * because that belongs to:
 *
 *     grammar/types/types.g4
 *
 * This avoids a circular parser dependency of the form:
 *
 *     Types
 *       -> QuantumTypes
 *       -> typeExpression
 *       -> Types
 *
 * Quantum-specific syntax therefore remains a leaf boundary.
 *
 * ============================================================================
 * GENERIC TYPE OWNERSHIP
 * ============================================================================
 *
 * Universal generic application remains owned by:
 *
 *     grammar/types/types.g4
 *
 * Therefore a source construct such as:
 *
 *     quantum::State<T>
 *
 * is conceptually:
 *
 *     qualified name
 *          +
 *     universal generic application
 *
 * rather than a second quantum-specific generic type system.
 *
 * This file only owns the genuinely quantum-specific open constructor:
 *
 *     quantum<...>
 *
 * where that spelling is part of the Zamani quantum type language.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * The grammar describes source-level semantics.
 *
 * It MUST NOT encode target-specific resource ceilings.
 *
 * Forbidden universal language limits include:
 *
 *     MAX_QUBITS
 *     MAX_LOGICAL_QUBITS
 *     MAX_PHYSICAL_QUBITS
 *     MAX_REGISTER_WIDTH
 *     MAX_REGISTER_COUNT
 *     MAX_STATE_DIMENSION
 *     MAX_QPU_SIZE
 *     MAX_QPUS
 *     MAX_DEVICES
 *     MAX_CONTROLS
 *     MAX_TENSOR_RANK
 *
 * No finite hardware capacity is encoded by this grammar.
 *
 * ============================================================================
 * RESOURCE SEMANTICS
 * ============================================================================
 *
 * These are source-level semantic forms:
 *
 *     qubit
 *     logical qubit
 *     qubit[n]
 *     logical qubit[n]
 *
 * They do NOT mean:
 *
 *     physical qubit 0
 *     physical qubits 0..n-1
 *     a particular QPU
 *     a particular vendor
 *     a particular topology
 *     a particular QEC code
 *
 * Resource feasibility is determined downstream.
 *
 * The distinction is:
 *
 *     source type
 *         !=
 *     resource requirement
 *         !=
 *     target capability
 *         !=
 *     physical realization
 *
 * ============================================================================
 * CARDINALITY SEMANTICS
 * ============================================================================
 *
 * The expression in:
 *
 *     qubit[n]
 *
 * is syntax only.
 *
 * Examples:
 *
 *     qubit[n]
 *     qubit[N + 1]
 *     qubit[2 * N]
 *     qubit[width]
 *     qubit[algorithm::width]
 *     qubit[(N + M) * factor]
 *
 * The parser MUST NOT:
 *
 *     - evaluate the expression;
 *     - convert it to usize;
 *     - compare it with a hardware maximum;
 *     - allocate memory;
 *     - inspect a target;
 *     - inspect a QPU;
 *     - require compile-time evaluation.
 *
 * Semantic analysis determines whether a cardinality is:
 *
 *     - valid;
 *     - integral;
 *     - non-negative;
 *     - compile-time known;
 *     - symbolic;
 *     - runtime dependent;
 *     - materializable;
 *     - resource-feasible.
 *
 * ============================================================================
 * INTEGER MAGNITUDE
 * ============================================================================
 *
 * INTEGER_LITERAL is a lexical token.
 *
 * This grammar imposes no numeric magnitude limit.
 *
 * A parser implementation may have explicit hostile-input/resource policies,
 * but those policies MUST NOT become Zamani language semantics.
 *
 * In particular, this grammar does not assume:
 *
 *     u32
 *     u64
 *     usize
 *
 * for cardinality.
 *
 * Representation policy belongs to the frontend implementation and semantic
 * value system.
 *
 * ============================================================================
 * SOURCE AST CONTRACT
 * ============================================================================
 *
 * This grammar produces parser structure only.
 *
 * Expected semantic mapping is:
 *
 *     quantumPrimitiveType
 *         -> existing frontend TypeExpr quantum representation
 *
 *     logicalQubitType
 *         -> existing frontend TypeExpr logical-quantum representation
 *
 *     quantumCollectionType
 *         -> existing TypeExpr collection/array representation with a
 *            symbolic TypeValueExpr cardinality
 *
 *     quantumQualifiedType
 *         -> existing qualified/named TypeExpr
 *
 *     quantumQuantumConstructorType
 *         -> existing generic/named TypeExpr representation
 *
 * No new:
 *
 *     QuantumTypeIR
 *     QuantumTypeNode
 *     QuantumRegisterIR
 *     QubitId
 *     PhysicalQubitId
 *
 * is introduced merely because this grammar exists.
 *
 * ============================================================================
 * QUANTUM::IR CONTRACT
 * ============================================================================
 *
 * Correct:
 *
 *     parser
 *       ->
 *     frontend TypeExpr
 *       ->
 *     semantic quantum type
 *       ->
 *     quantum::ir
 *
 * Forbidden:
 *
 *     parser
 *       ->
 *     quantum::ir
 *
 * The grammar therefore remains independent of:
 *
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *     calibration
 *     HAL
 *     backend selection
 *     runtime
 *
 * ============================================================================
 * LOGICAL VS PHYSICAL
 * ============================================================================
 *
 * `logical qubit` is a semantic source type.
 *
 * It does not encode:
 *
 *     - a physical qubit;
 *     - a code distance;
 *     - a surface-code implementation;
 *     - a stabilizer layout;
 *     - a decoder;
 *     - physical-qubit overhead.
 *
 * Physical intent belongs to the physical-qubits/hardware/resource layers.
 *
 * ============================================================================
 * FUTURE QUANTUM COMPUTING MODELS
 * ============================================================================
 *
 * Quantum computing is broader than circuit-only qubits.
 *
 * Future source types may include:
 *
 *     photonic modes
 *     bosonic modes
 *     continuous-variable systems
 *     neutral atoms
 *     trapped ions
 *     spin systems
 *     topological resources
 *     measurement-based resources
 *     analog quantum resources
 *     quantum-network resources
 *
 * Such types should normally enter through:
 *
 *     qualified names
 *     user-defined types
 *     dialect-defined types
 *     universal generic types
 *
 * rather than forcing every future technology into a new built-in keyword.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains no target-language actions.
 *
 * It performs:
 *
 *     no filesystem access;
 *     no network access;
 *     no environment inspection;
 *     no hardware discovery;
 *     no runtime execution;
 *     no random selection;
 *     no backend invocation.
 *
 * Generated Rust integration must remain:
 *
 *     Rust 2021
 *     Rust 1.97
 *     Rust 1.97.1
 *     safe Rust only
 *     no unsafe
 *
 * Repository Rust code should enforce this independently with:
 *
 *     #![forbid(unsafe_code)]
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Identical source, grammar version, and lexical configuration MUST produce
 * identical parse structure.
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware;
 *     target availability;
 *     QPU state;
 *     filesystem state;
 *     network state;
 *     environment variables;
 *     wall-clock time;
 *     randomness;
 *     runtime state.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * This grammar intentionally rejects malformed quantum type structure rather
 * than accepting arbitrary expressions as quantum types.
 *
 * Examples that must be rejected syntactically:
 *
 *     qubit[]
 *     logical
 *     qubit[
 *     qubit]
 *     qubit[n
 *     qubit[n]]
 *     logical qubit[]
 *
 * Semantic errors such as:
 *
 *     qubit[-1]
 *
 * are deliberately left to semantic validation because the parser recognizes
 * the expression structure.
 *
 * ============================================================================
 */

parser grammar QuantumTypes;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ========================================================================== */

/**
 * Canonical quantum-specific source type.
 *
 * This rule contains only forms whose syntax is intrinsically quantum-specific.
 *
 * Universal named/generic/tuple/array/reference/function types remain owned by
 * the universal type grammar.
 */
quantumType
    : quantumPrimitiveType
    | logicalQubitType
    | quantumCollectionType
    | quantumQualifiedType
    | quantumQuantumConstructorType
    ;


/* ============================================================================
 * 2. ABSTRACT QUBIT
 * ========================================================================== */

/**
 * Abstract quantum resource.
 *
 *     qubit
 *
 * This is not a physical resource identifier.
 */
quantumPrimitiveType
    : K_QUBIT
    ;


/* ============================================================================
 * 3. LOGICAL QUBIT
 * ========================================================================== */

/**
 * Error-corrected/logical quantum resource at the semantic level.
 *
 *     logical qubit
 *
 * No QEC implementation is selected by the syntax.
 */
logicalQubitType
    : K_LOGICAL
      K_QUBIT
    ;


/* ============================================================================
 * 4. SCALABLE QUANTUM COLLECTION
 * ========================================================================== */

/**
 * Quantum resource collection with a source-level symbolic extent.
 *
 * Examples:
 *
 *     qubit[n]
 *     qubit[N + 1]
 *     logical qubit[width]
 *
 * The extent is an expression, not a machine capacity.
 */
quantumCollectionType
    : quantumCollectionElementType
      LBRACKET
      quantumCardinalityExpression
      RBRACKET
    ;


/**
 * Element types for the dedicated quantum collection shorthand.
 *
 * Universal arrays remain available through the universal type system.
 *
 * Keeping this rule narrow prevents quantumType from consuming unrelated
 * universal types.
 */
quantumCollectionElementType
    : quantumPrimitiveType
    | logicalQubitType
    ;


/* ============================================================================
 * 5. SYMBOLIC QUANTUM CARDINALITY
 * ========================================================================== */

/**
 * Canonical cardinality expression.
 *
 * Examples:
 *
 *     N
 *     n + 1
 *     2 * n
 *     width + offset
 *     algorithm::width
 *     (N + M) * factor
 *
 * The parser does not evaluate the expression.
 */
quantumCardinalityExpression
    : quantumCardinalityAdditiveExpression
    ;


/**
 * Addition/subtraction precedence level.
 */
quantumCardinalityAdditiveExpression
    : quantumCardinalityMultiplicativeExpression
      (
          PLUS quantumCardinalityMultiplicativeExpression
        | MINUS quantumCardinalityMultiplicativeExpression
      )*
    ;


/**
 * Multiplication/division/modulo precedence level.
 */
quantumCardinalityMultiplicativeExpression
    : quantumCardinalityUnaryExpression
      (
          STAR quantumCardinalityUnaryExpression
        | SLASH quantumCardinalityUnaryExpression
        | PERCENT quantumCardinalityUnaryExpression
      )*
    ;


/**
 * Unary arithmetic.
 *
 * Semantic analysis determines whether a negative result is legal for a
 * particular cardinality context.
 */
quantumCardinalityUnaryExpression
    : PLUS quantumCardinalityUnaryExpression
    | MINUS quantumCardinalityUnaryExpression
    | quantumCardinalityPrimary
    ;


/**
 * Cardinality expression primary.
 *
 * Integer literals remain lexical values.
 *
 * Identifiers remain unresolved names.
 */
quantumCardinalityPrimary
    : INTEGER_LITERAL
    | IDENTIFIER
    | quantumQualifiedName
    | quantumCardinalityParenthesized
    ;


/**
 * Qualified symbolic cardinality name.
 *
 * Examples:
 *
 *     config::width
 *     algorithm::qubits
 *     module::parameter::N
 *
 * Namespace depth is not bounded by this grammar.
 */
quantumQualifiedName
    : IDENTIFIER
      (
          DOUBLE_COLON IDENTIFIER
      )*
    ;


/**
 * Parenthesized cardinality.
 */
quantumCardinalityParenthesized
    : LPAREN
      quantumCardinalityExpression
      RPAREN
    ;


/* ============================================================================
 * 6. QUALIFIED QUANTUM TYPES
 * ========================================================================== */

/**
 * Explicit quantum namespace qualification.
 *
 * Examples:
 *
 *     quantum::State
 *     quantum::Observable
 *     quantum::future::State
 *
 * Name resolution is semantic.
 */
quantumQualifiedType
    : K_QUANTUM
      DOUBLE_COLON
      quantumQualifiedTypeTail
    ;


/**
 * Tail of an explicitly quantum-qualified type.
 *
 * Qualification depth is unbounded by grammar.
 */
quantumQualifiedTypeTail
    : IDENTIFIER
      (
          DOUBLE_COLON IDENTIFIER
      )*
    ;


/* ============================================================================
 * 7. OPEN QUANTUM CONSTRUCTOR
 * ========================================================================== */

/**
 * Quantum semantic constructor.
 *
 * Examples:
 *
 *     quantum<state>
 *     quantum<observable>
 *     quantum<resource>
 *
 * This is intentionally open-ended.
 *
 * It does NOT enumerate:
 *
 *     state;
 *     observable;
 *     register;
 *     amplitude;
 *     etc.
 *
 * as keywords.
 *
 * The semantic/type layer determines the meaning of each constructor.
 *
 * NOTE:
 *
 * This production deliberately uses a quantum-specific argument grammar rather
 * than universal `typeExpression`, preventing a Types <-> QuantumTypes parser
 * import cycle.
 */
quantumQuantumConstructorType
    : K_QUANTUM
      LESS_THAN
      quantumTypeArgumentList
      GREATER_THAN
    ;


/**
 * One or more quantum constructor arguments.
 *
 * A trailing comma is intentionally accepted.
 */
quantumTypeArgumentList
    : quantumTypeValueArgument
      (
          COMMA quantumTypeValueArgument
      )*
      COMMA?
    ;


/**
 * Quantum constructor argument.
 *
 * The grammar accepts source-level names, qualified names, literals, and
 * nested quantum type forms.
 *
 * Semantic validation determines whether an individual argument is:
 *
 *     type;
 *     value;
 *     shape;
 *     cardinality;
 *     unit;
 *     symbolic parameter;
 *     dialect-defined parameter.
 *
 * This is intentionally not a closed list.
 */
quantumTypeValueArgument
    : quantumTypeArgumentAtom
    ;


/**
 * Atomic quantum constructor argument.
 *
 * Nested `quantum<...>` values are supported without invoking universal
 * `typeExpression`, thereby preserving the single-direction grammar
 * dependency.
 */
quantumTypeArgumentAtom
    : INTEGER_LITERAL
    | IDENTIFIER
    | quantumQualifiedName
    | quantumPrimitiveType
    | logicalQubitType
    | quantumQualifiedType
    | quantumQuantumConstructorType
    | quantumCardinalityParenthesized
    ;


/* ============================================================================
 * 8. EXPLICIT EXTENSION POINT
 * ========================================================================== */

/**
 * Future quantum-specific type forms can be added only when they have a
 * formally specified language-level semantic meaning.
 *
 * This rule intentionally does not accept arbitrary identifiers.
 *
 * Ordinary user-defined quantum types remain represented by the universal
 * named/generic type machinery.
 */
quantumTypeExtension
    : quantumQualifiedType
    | quantumQuantumConstructorType
    ;


/* ============================================================================
 * 9. SEMANTIC BOUNDARY
 * ========================================================================== */

/*
 * The following questions are NOT answered here:
 *
 *     Is the cardinality positive?
 *     Is the cardinality finite?
 *     Is the cardinality statically known?
 *     Does it fit the target?
 *     Does the target have enough qubits?
 *     Is a logical qubit backed by a QEC code?
 *     Which QEC code is selected?
 *     How many physical qubits encode it?
 *     Which topology is used?
 *     Which device is selected?
 *     Which vendor implements it?
 *     Is the value linear?
 *     Is it affine?
 *     Can it be copied?
 *     Can it be borrowed?
 *     Can it cross a classical/quantum boundary?
 *
 * Those decisions belong downstream semantic systems.
 */


/* ============================================================================
 * 10. HARD-CODING AUDIT
 * ========================================================================== */

/*
 * This grammar intentionally contains no:
 *
 *     MAX_QUBITS
 *     MAX_LOGICAL_QUBITS
 *     MAX_PHYSICAL_QUBITS
 *     MAX_QPU_SIZE
 *     MAX_REGISTER_WIDTH
 *     MAX_REGISTER_COUNT
 *     MAX_STATE_DIMENSION
 *     MAX_TENSOR_RANK
 *     MAX_DEVICES
 *     MAX_CONTROLS
 *
 * Nor does it enumerate physical resources:
 *
 *     qubit0
 *     qubit1
 *     qpu0
 *     device0
 *
 * Numeric literals remain ordinary program/type arguments.
 */


/* ============================================================================
 * 11. INTEGRATION CHECKLIST
 * ========================================================================== */

/*
 * This file is complete independently when all of the following are true:
 *
 * [x] Parser-only grammar.
 * [x] Canonical parser-facing token vocabulary.
 * [x] No lexer rules.
 * [x] No Rust actions.
 * [x] No unsafe requirement.
 * [x] No hardware dependency.
 * [x] No quantum::ir dependency.
 * [x] No QEC dependency.
 * [x] No routing dependency.
 * [x] No scheduling dependency.
 * [x] No target dependency.
 * [x] No fixed gate enumeration.
 * [x] No fixed qubit limit.
 * [x] No fixed register limit.
 * [x] No fixed namespace depth.
 * [x] Symbolic cardinality supported.
 * [x] Arbitrary source-level cardinality expressions supported.
 * [x] Logical and abstract qubit semantics separated.
 * [x] Universal type system not duplicated.
 * [x] Generic universal type system not recursively reimplemented.
 * [x] AST ownership defined.
 * [x] semantic ownership defined.
 * [x] quantum::ir boundary defined.
 * [x] POCO-REAF boundary defined.
 *
 * Integration then consists only of composition:
 *
 *     Quantum
 *         imports QuantumTypes
 *
 * and:
 *
 *     ZamaniParser
 *         imports Quantum
 *
 * No change to this file is required merely because another downstream
 * subsystem is subsequently implemented.
 */


/* ============================================================================
 * 12. REQUIRED CONFORMANCE EXAMPLES
 * ========================================================================== */

/*
 * The following forms are intended to parse through this grammar:
 *
 *     qubit
 *
 *     logical qubit
 *
 *     qubit[n]
 *
 *     logical qubit[n]
 *
 *     qubit[N + 1]
 *
 *     qubit[2 * N]
 *
 *     qubit[algorithm::width]
 *
 *     qubit[(N + M) * factor]
 *
 *     quantum::State
 *
 *     quantum::future::State
 *
 *     quantum<state>
 *
 *     quantum<resource, N>
 *
 *     quantum<logical qubit>
 *
 *     quantum<quantum<state>>
 *
 *     quantum<Register, width>
 *
 * The following are intentionally NOT owned by this grammar:
 *
 *     Tensor<float, shape>
 *
 *     Vec<qubit>
 *
 *     Option<qubit>
 *
 *     Result<qubit, Error>
 *
 *     fn(qubit) -> bit
 *
 * Those use the universal type system and may contain quantum types as
 * arguments.
 */


/* ============================================================================
 * 13. NEGATIVE STRUCTURAL CASES
 * ========================================================================== */

/*
 * The following must not be accepted by this grammar as complete quantum
 * types:
 *
 *     qubit[]
 *
 *     logical
 *
 *     qubit]
 *
 *     [qubit]
 *
 *     qubit[n
 *
 *     qubit[n]]
 *
 *     logical qubit[]
 *
 *     logical logical qubit
 *
 *     quantum::
 *
 *     quantum<>
 *
 *     quantum<State
 *
 *     quantum<, State>
 *
 *     quantum<State,,N>
 *
 * Semantic-invalid-but-structurally-parseable examples such as:
 *
 *     qubit[-1]
 *
 * remain intentionally delegated to semantic cardinality validation.
 */


/* ============================================================================
 * 14. FINAL ARCHITECTURAL INVARIANT
 * ========================================================================== */

/*
 * This file describes WHAT a quantum type is.
 *
 * It does not decide:
 *
 *     WHERE it executes;
 *     WHEN it executes;
 *     HOW it is physically represented;
 *     WHICH QPU executes it;
 *     WHICH physical qubits are selected;
 *     WHICH QEC code is used;
 *     WHICH topology is used;
 *     WHICH routing algorithm is used;
 *     WHICH schedule is selected;
 *     WHICH calibration is required.
 *
 * Those decisions occur after parsing.
 *
 * Therefore the canonical flow remains:
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
 *     domain-neutral AST / TypeExpr
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
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     QEC / resilience
 *          |
 *          v
 *     ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target realization
 *
 * This is the quantum-type boundary required for POCO-REAF.
 */