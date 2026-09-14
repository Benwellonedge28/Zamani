/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/quantum-types.g4
 *
 * Grammar:
 *     QuantumTypes
 *
 * Purpose:
 *     Production parser grammar for SOURCE-LEVEL QUANTUM TYPE SYNTAX.
 *
 * Architectural position:
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniTokens
 *          |
 *          v
 *     canonical Zamani parser
 *          |
 *          +---- QuantumTypes
 *          |
 *          v
 *     frontend AST / TypeExpr
 *          |
 *          v
 *     semantic/type analysis
 *          |
 *          +---- capability analysis
 *          +---- resource analysis
 *          +---- effect analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          v
 *     quantum::ir
 *          |
 *          +---- optimization
 *          +---- routing
 *          +---- scheduling
 *          +---- QEC
 *          +---- ZQN
 *          +---- hardware lowering
 *          +---- simulation
 *          |
 *          v
 *     execution
 *
 * ============================================================================
 * LANGUAGE PRINCIPLE
 * ============================================================================
 *
 * Zamani source describes computational meaning and intent.
 *
 * Quantum types therefore describe:
 *
 *     WHAT KIND OF QUANTUM VALUE OR RESOURCE
 *     a program requires or manipulates.
 *
 * They do NOT describe:
 *
 *     HOW a quantum resource is physically implemented.
 *
 * This grammar MUST NOT encode:
 *
 *     - maximum qubit counts;
 *     - maximum register widths;
 *     - maximum state dimensions;
 *     - physical topology;
 *     - device identifiers;
 *     - vendor identifiers;
 *     - QPU names;
 *     - processor generations;
 *     - coupling maps;
 *     - calibration data;
 *     - pulse implementations;
 *     - routing;
 *     - scheduling;
 *     - error-correction algorithms;
 *     - noise models;
 *     - simulator implementations;
 *     - backend selection.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A quantum type is portable source-level intent.
 *
 * For example:
 *
 *     qubit
 *
 * means an abstract quantum resource.
 *
 * It does NOT mean:
 *
 *     physical qubit 0
 *     physical qubit 1
 *     QPU X
 *     topology Y
 *     vendor Z
 *
 * Similarly:
 *
 *     qubit[n]
 *
 * expresses a source-level collection/cardinality requirement.
 *
 * It does NOT mean:
 *
 *     allocate physical qubits 0 through n - 1
 *
 * Resource availability and target realization belong downstream.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - primitive quantum type syntax;
 *     - logical quantum type syntax;
 *     - abstract quantum type constructors;
 *     - quantum type qualification;
 *     - quantum type parameters;
 *     - symbolic quantum cardinality syntax;
 *     - quantum type composition syntax;
 *     - source-level quantum type modifiers that are genuinely semantic;
 *     - parser boundaries for quantum types.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - quantum register declarations;
 *     - qubit allocation;
 *     - qubit release;
 *     - physical qubit identifiers;
 *     - physical topology;
 *     - device selection;
 *     - gate definitions;
 *     - quantum operations;
 *     - circuits;
 *     - measurements;
 *     - observables;
 *     - reset;
 *     - dynamic circuits;
 *     - classical control;
 *     - QEC;
 *     - ZQN;
 *     - noise;
 *     - calibration;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - runtime execution;
 *     - quantum::ir.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Lexer:
 *
 *     grammar/lexer/tokens.g4
 *
 * Canonical token vocabulary:
 *
 *     ZamaniTokens
 *
 * Relevant tokens include:
 *
 *     K_QUANTUM
 *     K_QUBIT
 *     K_LOGICAL
 *     K_TYPE
 *     IDENTIFIER
 *     INTEGER_LITERAL
 *     LPAREN
 *     RPAREN
 *     LBRACKET
 *     RBRACKET
 *     COMMA
 *     DOUBLE_COLON
 *     LESS_THAN
 *     GREATER_THAN
 *     PLUS
 *     MINUS
 *     STAR
 *     SLASH
 *     PERCENT
 *     LEFT_SHIFT
 *     RIGHT_SHIFT
 *     AMPERSAND
 *     PIPE
 *     CARET
 *
 * This grammar declares NO lexer rules.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces parser structure only.
 *
 * The frontend may lower these structures into the repository's existing
 * TypeExpr representation.
 *
 * This grammar MUST NOT introduce:
 *
 *     QuantumTypeIR
 *     QuantumTypeNode
 *     QuantumHardwareTypeIR
 *     QuantumRegisterIR
 *     QubitId
 *     PhysicalQubitId
 *
 * solely for parsing.
 *
 * Existing repository AST/type structures remain authoritative.
 *
 * ============================================================================
 * QUANTUM::IR CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT depend on quantum::ir.
 *
 * Correct direction:
 *
 *     QuantumTypes
 *          |
 *          v
 *     AST / TypeExpr
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 * Incorrect:
 *
 *     QuantumTypes <-> quantum::ir
 *
 * The grammar therefore remains independent of quantum compilation,
 * optimization, routing, scheduling, QEC, ZQN and hardware.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This is ANTLR grammar source.
 *
 * It contains no Rust target-language actions.
 *
 * Generated/integrating Rust code MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * Repository Rust policy:
 *
 *     #![deny(unsafe_code)]
 *
 * This grammar itself requires no unsafe code.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There are NO finite quantum resource limits in this file.
 *
 * Forbidden examples:
 *
 *     MAX_QUBITS = 32
 *     MAX_QUBITS = 64
 *     MAX_REGISTER_SIZE = 1024
 *
 * Also forbidden:
 *
 *     q0
 *     q1
 *     physical_q0
 *     device_0
 *
 * as built-in resource assumptions.
 *
 * Cardinality expressions remain symbolic.
 *
 * ============================================================================
 */

parser grammar QuantumTypes;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ========================================================================== */

/**
 * Canonical quantum type expression.
 *
 * This rule is intentionally compositional.
 *
 * It supports:
 *
 *     qubit
 *     logical qubit
 *     qubit[n]
 *     logical qubit[n]
 *     quantum<T>
 *     quantum<T, P>
 *     quantum::State
 *     quantum::State<T>
 *     user-defined qualified quantum types
 *
 * The semantic/type system determines which forms are legal.
 */
quantumType
    : quantumPrimitiveType
    | logicalQubitType
    | quantumCollectionType
    | quantumConstructorType
    | quantumQualifiedType
    | quantumNamedType
    ;


/* ============================================================================
 * 2. ABSTRACT QUBIT TYPE
 * ========================================================================== */

/**
 * Primitive quantum resource.
 *
 * Example:
 *
 *     qubit
 *
 * This is an abstract computational resource.
 */
quantumPrimitiveType
    : K_QUBIT
    ;


/* ============================================================================
 * 3. LOGICAL QUBIT TYPE
 * ========================================================================== */

/**
 * Logical qubit.
 *
 * Example:
 *
 *     logical qubit
 *
 * This does not select:
 *
 *     - a QEC code;
 *     - a code distance;
 *     - a decoder;
 *     - physical qubit count;
 *     - physical topology.
 *
 * Those decisions belong to semantic compilation and resource layers.
 */
logicalQubitType
    : K_LOGICAL K_QUBIT
    ;


/* ============================================================================
 * 4. QUANTUM COLLECTION TYPE
 * ========================================================================== */

/**
 * Scalable quantum collection.
 *
 * Examples:
 *
 *     qubit[n]
 *     qubit[N + 1]
 *     qubit[2 * N]
 *     qubit[register_width]
 *
 * The expression inside [] is NOT evaluated by the grammar.
 */
quantumCollectionType
    : quantumCollectionElementType
      LBRACKET
      quantumCardinalityExpression
      RBRACKET
    ;


/**
 * Element type of a quantum collection.
 *
 * Deliberately limited to semantic quantum resource types.
 *
 * More complex collections can be represented through the general type
 * system rather than recursively expanding this rule.
 */
quantumCollectionElementType
    : quantumPrimitiveType
    | logicalQubitType
    ;


/* ============================================================================
 * 5. GENERAL QUANTUM TYPE CONSTRUCTOR
 * ========================================================================== */

/**
 * Generic quantum type constructor.
 *
 * Examples:
 *
 *     quantum<State>
 *     quantum<Observable>
 *     quantum<Register>
 *     quantum<Amplitude>
 *     quantum<CustomQuantumType>
 *
 * The constructor is open-ended.
 *
 * The parser does not maintain a closed list of quantum types.
 */
quantumConstructorType
    : K_QUANTUM
      LESS_THAN
      quantumTypeArgumentList
      GREATER_THAN
    ;


/**
 * One or more type constructor arguments.
 *
 * A trailing comma is accepted deliberately to make generated/source code
 * formatting stable and extensible.
 */
quantumTypeArgumentList
    : quantumTypeArgument
      (COMMA quantumTypeArgument)*
      COMMA?
    ;


/**
 * Quantum type constructor argument.
 *
 * Type arguments may be semantic types or symbolic cardinality expressions.
 *
 * Whether a particular combination is meaningful is a semantic question.
 */
quantumTypeArgument
    : quantumTypeArgumentType
    | quantumCardinalityExpression
    ;


/**
 * Type-valued quantum constructor argument.
 */
quantumTypeArgumentType
    : quantumType
    ;


/* ============================================================================
 * 6. QUALIFIED QUANTUM TYPES
 * ========================================================================== */

/**
 * Explicitly qualified quantum type.
 *
 * Examples:
 *
 *     quantum::State
 *     quantum::Observable
 *     quantum::future::State
 *
 * Qualification is syntax.
 *
 * Name resolution belongs to the semantic/name-resolution layer.
 */
quantumQualifiedType
    : K_QUANTUM
      DOUBLE_COLON
      quantumQualifiedTypeTail
    ;


/**
 * Arbitrarily deep qualified quantum type.
 *
 * No namespace-depth limit is imposed.
 */
quantumQualifiedTypeTail
    : IDENTIFIER
      (DOUBLE_COLON IDENTIFIER)*
    ;


/* ============================================================================
 * 7. NAMED QUANTUM TYPES
 * ========================================================================== */

/**
 * User-defined or imported quantum type.
 *
 * Examples:
 *
 *     State
 *     LogicalState
 *     MyQuantumType
 *
 * The parser does not need to know whether the name denotes a built-in,
 * imported, dialect-defined, or user-defined type.
 */
quantumNamedType
    : IDENTIFIER
    ;


/* ============================================================================
 * 8. QUANTUM CARDINALITY EXPRESSIONS
 * ========================================================================== */

/**
 * Symbolic cardinality expression.
 *
 * Examples:
 *
 *     qubit[N]
 *     qubit[n + 1]
 *     qubit[2 * n]
 *     qubit[width]
 *     qubit[width + offset]
 *     qubit[(N + M) * factor]
 *
 * This grammar intentionally does NOT define a maximum value.
 *
 * It also does not require the value to fit into a Rust usize, machine word,
 * register, address space, or target-specific integer representation.
 *
 * Semantic analysis determines:
 *
 *     - whether the expression is a valid cardinality;
 *     - whether it is compile-time known;
 *     - whether it is symbolic;
 *     - whether it is runtime-dependent;
 *     - whether it can be materialized;
 *     - whether available resources satisfy it.
 */
quantumCardinalityExpression
    : quantumCardinalityAdditiveExpression
    ;


/**
 * Addition/subtraction level.
 *
 * Kept separate from multiplication/division so the parse tree preserves
 * arithmetic precedence.
 */
quantumCardinalityAdditiveExpression
    : quantumCardinalityMultiplicativeExpression
      (
          PLUS quantumCardinalityMultiplicativeExpression
        | MINUS quantumCardinalityMultiplicativeExpression
      )*
    ;


/**
 * Multiplication/division/modulo level.
 */
quantumCardinalityMultiplicativeExpression
    : quantumCardinalityShiftExpression
      (
          STAR quantumCardinalityShiftExpression
        | SLASH quantumCardinalityShiftExpression
        | PERCENT quantumCardinalityShiftExpression
      )*
    ;


/**
 * Shift level.
 *
 * Shifts are syntactically accepted because the source-level expression
 * language may use symbolic integer arithmetic.
 *
 * Semantic validation determines whether a shift is a valid cardinality
 * expression.
 */
quantumCardinalityShiftExpression
    : quantumCardinalityBitwiseExpression
      (
          LEFT_SHIFT quantumCardinalityBitwiseExpression
        | RIGHT_SHIFT quantumCardinalityBitwiseExpression
      )*
    ;


/**
 * Bitwise expression level.
 */
quantumCardinalityBitwiseExpression
    : quantumCardinalityUnaryExpression
      (
          AMPERSAND quantumCardinalityUnaryExpression
        | PIPE quantumCardinalityUnaryExpression
        | CARET quantumCardinalityUnaryExpression
      )*
    ;


/**
 * Unary cardinality operators.
 */
quantumCardinalityUnaryExpression
    : PLUS quantumCardinalityUnaryExpression
    | MINUS quantumCardinalityUnaryExpression
    | quantumCardinalityPrimary
    ;


/**
 * Primary cardinality expression.
 *
 * Integer literals are accepted without imposing a finite upper bound in the
 * grammar.
 *
 * Identifiers permit symbolic cardinality.
 */
quantumCardinalityPrimary
    : INTEGER_LITERAL
    | IDENTIFIER
    | quantumQualifiedCardinalityName
    | quantumCardinalityParenthesized
    ;


/**
 * Qualified symbolic cardinality.
 *
 * Examples:
 *
 *     config::width
 *     algorithm::qubits
 *     module::parameter::N
 */
quantumQualifiedCardinalityName
    : IDENTIFIER
      DOUBLE_COLON
      IDENTIFIER
      (DOUBLE_COLON IDENTIFIER)*
    ;


/**
 * Parenthesized cardinality expression.
 */
quantumCardinalityParenthesized
    : LPAREN
      quantumCardinalityExpression
      RPAREN
    ;


/* ============================================================================
 * 9. GENERIC QUANTUM TYPE ARGUMENTS
 * ========================================================================== */

/**
 * Quantum type parameter list.
 *
 * Examples:
 *
 *     quantum<State, N>
 *     quantum<Register, Width>
 *
 * The grammar does not prescribe the meaning of individual parameters.
 */
quantumTypeParameters
    : LESS_THAN
      quantumTypeParameterList
      GREATER_THAN
    ;


quantumTypeParameterList
    : quantumTypeParameter
      (COMMA quantumTypeParameter)*
      COMMA?
    ;


quantumTypeParameter
    : IDENTIFIER
    | quantumCardinalityExpression
    ;


/* ============================================================================
 * 10. QUANTUM TYPE APPLICATION
 * ========================================================================== */

/**
 * Generic application of a named quantum type.
 *
 * Examples:
 *
 *     Register<N>
 *     LogicalRegister<Width>
 *     quantum::Register<N>
 *
 * This is syntactic only.
 */
quantumParameterizedNamedType
    : quantumTypeName
      LESS_THAN
      quantumTypeArgumentList
      GREATER_THAN
    ;


/**
 * Name that may participate in generic application.
 */
quantumTypeName
    : IDENTIFIER
    | quantumQualifiedName
    ;


/**
 * Arbitrarily deep qualified type name.
 */
quantumQualifiedName
    : IDENTIFIER
      (DOUBLE_COLON IDENTIFIER)*
    ;


/* ============================================================================
 * 11. QUANTUM TYPE COMPOSITION
 * ========================================================================== */

/**
 * Composed quantum type.
 *
 * This rule provides a stable parser extension point for future quantum
 * type constructors without forcing physical implementation concepts into
 * the grammar.
 *
 * Examples may eventually include:
 *
 *     quantum<state>
 *     quantum<logical>
 *     quantum<observable>
 *     user::QuantumDomain<T>
 *
 * Composition remains a type-system concern.
 */
quantumComposedType
    : quantumType
    | quantumParameterizedNamedType
    ;


/* ============================================================================
 * 12. QUANTUM REFERENCE TYPE
 * ========================================================================== */

/**
 * Reference-like quantum type syntax.
 *
 * This is intentionally represented as a source-level type constructor rather
 * than a runtime pointer/reference representation.
 *
 * Examples:
 *
 *     quantum::ref<qubit>
 *     quantum::ref<logical qubit>
 *
 * The semantic/type layer determines ownership, borrowing, aliasing and
 * lifetime rules.
 */
quantumReferenceType
    : K_QUANTUM
      DOUBLE_COLON
      IDENTIFIER
      LESS_THAN
      quantumType
      GREATER_THAN
    ;


/* ============================================================================
 * 13. QUANTUM TYPE UNION
 * ========================================================================== */

/**
 * Type-level alternatives.
 *
 * The actual algebraic type system remains owned by the universal type
 * grammar. This rule only provides a quantum-specific parser boundary when
 * the canonical parser explicitly requests it.
 */
quantumTypeUnion
    : quantumTypeUnionMember
      (PIPE quantumTypeUnionMember)+
    ;


quantumTypeUnionMember
    : quantumType
    | quantumParameterizedNamedType
    ;


/* ============================================================================
 * 14. QUANTUM TYPE TUPLES
 * ========================================================================== */

/**
 * Quantum type tuple.
 *
 * Examples:
 *
 *     (qubit, qubit)
 *     (logical qubit, classical_type)
 *
 * The parser only establishes tuple structure.
 *
 * General tuple semantics remain owned by the universal type system.
 */
quantumTypeTuple
    : LPAREN
      quantumType
      COMMA
      quantumType
      (COMMA quantumType)*
      COMMA?
      RPAREN
    ;


/* ============================================================================
 * 15. QUANTUM TYPE ARRAY FORM
 * ========================================================================== */

/**
 * General type-level quantum array.
 *
 * This is distinct from the quantum register syntax:
 *
 *     qubit[N]
 *
 * because a generic quantum value can itself participate in an array type.
 *
 * The universal type system may later normalize both representations.
 */
quantumTypeArray
    : quantumType
      LBRACKET
      quantumCardinalityExpression?
      RBRACKET
    ;


/* ============================================================================
 * 16. QUANTUM TYPE MODIFIERS
 * ========================================================================== */

/**
 * Reserved extension point for semantic quantum type modifiers.
 *
 * IMPORTANT:
 *
 * This rule deliberately does not enumerate hardware properties.
 *
 * A modifier such as:
 *
 *     physical
 *     calibrated
 *     fault_tolerant
 *
 * must not be added here merely because a particular backend exposes it.
 *
 * Such concepts belong to the appropriate quantum/hardware/capability
 * grammar when they become stable Zamani language semantics.
 */
quantumTypeModifier
    : IDENTIFIER
    ;


/**
 * Quantum type with one or more semantic modifiers.
 *
 * This rule is intentionally not part of the default `quantumType` entry
 * point until the language specification formally assigns meaning to the
 * modifiers.
 *
 * It exists as a controlled extension boundary.
 */
modifiedQuantumType
    : quantumType
      quantumTypeModifier+
    ;


/* ============================================================================
 * 17. TYPE-LEVEL QUANTUM CARDINALITY CONSTRAINT
 * ========================================================================== */

/**
 * Symbolic type-level cardinality constraint.
 *
 * Examples:
 *
 *     qubit[N]
 *
 * and, when integrated with the universal constraint grammar:
 *
 *     qubit[N + 1]
 *
 * This rule does not enforce satisfiability.
 */
quantumCardinalityConstraint
    : quantumCardinalityConstraintOperator
      quantumCardinalityExpression
    ;


quantumCardinalityConstraintOperator
    : LESS_THAN
    | GREATER_THAN
    ;


/* ============================================================================
 * 18. QUANTUM TYPE DECLARATION REFERENCE
 * ========================================================================== */

/**
 * Reference to a declared quantum type.
 *
 * Declaration ownership remains with:
 *
 *     declarations/types.g4
 *
 * and the universal type system.
 */
quantumTypeDeclarationReference
    : quantumTypeName
    ;


/* ============================================================================
 * 19. OPEN QUANTUM TYPE EXTENSION
 * ========================================================================== */

/**
 * Open semantic type reference.
 *
 * This allows future quantum domains to use the general namespace/type
 * mechanism without requiring a new lexer token for every new technology.
 *
 * Examples:
 *
 *     photonic::Mode
 *     neutral_atom::Register
 *     topological::LogicalQubit
 *     future::QuantumResource
 *
 * These names are NOT interpreted here.
 *
 * The grammar therefore remains future-compatible without making these
 * technologies permanent built-in language keywords.
 */
openQuantumType
    : quantumQualifiedName
    ;


/* ============================================================================
 * 20. TYPE SEMANTICS BOUNDARY
 * ========================================================================== */

/*
 * The following semantic properties MUST NOT be decided by this grammar:
 *
 *     - whether qubit[N] can be allocated;
 *     - whether N is finite;
 *     - whether N fits on a target;
 *     - whether N is statically known;
 *     - whether N is runtime-dependent;
 *     - whether logical qubits require QEC;
 *     - which QEC code is selected;
 *     - how many physical qubits encode a logical qubit;
 *     - which hardware supports a type;
 *     - which simulator represents a type;
 *     - whether a quantum type is copyable;
 *     - whether a quantum value is movable;
 *     - whether a quantum value is borrowable;
 *     - whether aliasing is legal;
 *     - whether a type is compatible with an operation;
 *     - whether a type can cross a classical/quantum boundary.
 *
 * Those questions belong to:
 *
 *     semantic analysis
 *     type checking
 *     effect analysis
 *     ownership analysis
 *     resource analysis
 *     capability analysis
 *     quantum semantics
 *     compiler lowering
 *
 * and must not be embedded into parser actions.
 */


/* ============================================================================
 * 21. HARD-CODING PROHIBITION
 * ========================================================================== */

/*
 * NEVER add rules such as:
 *
 *     smallQubitRegister : K_QUBIT '[' '32' ']'
 *                         ;
 *
 *     standardRegister : K_QUBIT '[' '64' ']'
 *                      ;
 *
 *     physicalQubit : 'q0' | 'q1' | 'q2' ...
 *                   ;
 *
 *     supportedQubits : '32' | '64' | '127'
 *                     ;
 *
 *     ibmQubit : ...
 *              ;
 *
 *     topology : ...
 *             ;
 *
 * A resource count belongs to program semantics, resource constraints,
 * compilation context or runtime capabilities.
 *
 * The parser must remain independent of the available machine.
 */


/* ============================================================================
 * 22. DIAGNOSTIC BOUNDARY
 * ========================================================================== */

/*
 * Parser diagnostics are syntax diagnostics only.
 *
 * This grammar MUST NOT emit semantic diagnostics such as:
 *
 *     "target has too few qubits"
 *     "device does not support this type"
 *     "QEC distance is too large"
 *     "register exceeds hardware capacity"
 *
 * Those are semantic/resource/capability diagnostics.
 *
 * Parser recovery is delegated to the canonical ANTLR parser/frontend.
 *
 * No stdout/stderr output, filesystem access, network access or runtime
 * execution is permitted from this grammar.
 */


/* ============================================================================
 * 23. DETERMINISM CONTRACT
 * ========================================================================== */

/*
 * This grammar contains no:
 *
 *     - semantic actions;
 *     - external calls;
 *     - mutable global state;
 *     - random behavior;
 *     - target discovery;
 *     - runtime inspection.
 *
 * Therefore parsing is deterministic for a fixed token stream and grammar
 * version.
 */


/* ============================================================================
 * 24. VERSIONING CONTRACT
 * ========================================================================== */

/*
 * Changes to this grammar must follow Zamani language-version policy.
 *
 * In particular:
 *
 *     - changing `qubit` syntax is a language compatibility change;
 *     - changing `logical qubit` syntax is a language compatibility change;
 *     - changing cardinality syntax is a language compatibility change;
 *     - adding a new reserved keyword belongs to the lexer/versioning process;
 *     - changing semantic meaning without syntax changes requires a semantic
 *       specification/version update.
 *
 * Hardware additions MUST NOT require changing this grammar unless they
 * introduce a genuinely new source-level semantic concept.
 */


/* ============================================================================
 * 25. CROSS-DOMAIN INTEGRATION
 * ========================================================================== */

/*
 * Classical:
 *
 *     Classical expressions may supply symbolic cardinality parameters
 *     through the semantic/type system.
 *
 * Quantum:
 *
 *     This file supplies quantum type syntax to quantum declarations,
 *     operations, circuits and resource constructs.
 *
 * Hybrid:
 *
 *     Quantum types may appear in hybrid declarations and quantum/classical
 *     boundary constructs.
 *
 * HDL:
 *
 *     HDL may refer to quantum interfaces through explicit interoperability
 *     contracts, but HDL semantics do not depend on this file.
 *
 * Hardware:
 *
 *     Hardware capability analysis consumes semantic types; this grammar
 *     does not consume hardware descriptions.
 *
 * Resources:
 *
 *     Resource analysis may evaluate symbolic cardinality expressions.
 *
 * Compilation:
 *
 *     Compilation lowers semantic types to target-independent or
 *     target-specific representations.
 *
 * Runtime:
 *
 *     Runtime capability/resource information may determine whether a
 *     semantic quantum type can actually be realized.
 *
 * QEC:
 *
 *     QEC consumes semantic quantum requirements after type analysis.
 *
 * ZQN:
 *
 *     ZQN describes noise/fault behavior and does not depend on this grammar.
 *
 * Scheduling:
 *
 *     Scheduling consumes operations/dependencies after semantic lowering.
 *
 * Optimization:
 *
 *     Optimization consumes canonical semantic/IR representations.
 *
 * Routing:
 *
 *     Routing consumes logical computation and hardware topology.
 *
 * None of those downstream systems should import or depend directly on this
 * parser grammar.
 */


/* ============================================================================
 * 26. INTEGRATION WITH QUANTUM REGISTER GRAMMAR
 * ========================================================================== */

/*
 * `grammar/quantum/quantum-registers.g4` owns:
 *
 *     - register declarations;
 *     - register bindings;
 *     - register initializers;
 *     - register references;
 *     - register slices;
 *     - register selections;
 *     - abstract allocation syntax where appropriate.
 *
 * It consumes this file's:
 *
 *     quantumType
 *     quantumPrimitiveType
 *     logicalQubitType
 *     quantumCollectionType
 *
 * It must NOT redefine those concepts.
 *
 * Dependency:
 *
 *     QuantumTypes
 *          |
 *          v
 *     QuantumRegisters
 *
 * There is no reverse dependency.
 */


/* ============================================================================
 * 27. INTEGRATION WITH PHYSICAL QUBITS
 * ========================================================================== */

/*
 * `grammar/quantum/physical-qubits.g4` owns physical-qubit syntax.
 *
 * This file intentionally does NOT define physical qubit identifiers.
 *
 * A physical qubit is a target-level resource.
 *
 * If Zamani eventually permits explicit physical resource references,
 * physical-qubits.g4 must represent them through an opaque/qualified source
 * reference and semantic validation must resolve the resource.
 *
 * No device-specific physical identifiers belong here.
 */


/* ============================================================================
 * 28. INTEGRATION WITH LOGICAL QUBITS
 * ========================================================================== */

/*
 * `grammar/quantum/logical-qubits.g4` owns logical-qubit operations,
 * bindings, mappings and logical-resource semantics.
 *
 * This file only owns the TYPE:
 *
 *     logical qubit
 *
 * It does not own:
 *
 *     logical-to-physical mapping;
 *     QEC encoding;
 *     decoding;
 *     code distance;
 *     syndrome extraction;
 *     physical realization.
 */


/* ============================================================================
 * 29. INTEGRATION WITH UNIVERSAL TYPES
 * ========================================================================== */

/*
 * The canonical universal type system remains the authority for:
 *
 *     - generic types;
 *     - tuples;
 *     - arrays;
 *     - maps;
 *     - references;
 *     - options;
 *     - results;
 *     - algebraic types;
 *     - function types;
 *     - constraints.
 *
 * This grammar provides quantum-specific syntax at the quantum boundary.
 *
 * The frontend should normalize the resulting parse tree into the existing
 * universal TypeExpr representation rather than creating a parallel quantum
 * type hierarchy solely because this grammar exists.
 */


/* ============================================================================
 * 30. COMPLETION CONTRACT
 * ========================================================================== */

/*
 * This file is COMPLETE when all of the following are true:
 *
 * [ ] It compiles as an ANTLR parser grammar using ZamaniTokens.
 *
 * [ ] It contains no lexer rules.
 *
 * [ ] It contains no Rust actions.
 *
 * [ ] It contains no unsafe code.
 *
 * [ ] It introduces no machine-size limits.
 *
 * [ ] It introduces no fixed qubit count.
 *
 * [ ] It introduces no fixed register count.
 *
 * [ ] It introduces no physical device identifiers.
 *
 * [ ] It introduces no hardware topology.
 *
 * [ ] It introduces no vendor-specific syntax.
 *
 * [ ] `qubit` parses.
 *
 * [ ] `logical qubit` parses.
 *
 * [ ] `qubit[N]` parses.
 *
 * [ ] `logical qubit[N]` parses.
 *
 * [ ] Symbolic cardinality parses.
 *
 * [ ] Arithmetic cardinality parses.
 *
 * [ ] Qualified cardinality names parse.
 *
 * [ ] Generic quantum types parse.
 *
 * [ ] Qualified quantum types parse.
 *
 * [ ] Arbitrary namespace depth is supported.
 *
 * [ ] The grammar does not evaluate cardinalities.
 *
 * [ ] The grammar does not allocate resources.
 *
 * [ ] The grammar does not inspect hardware.
 *
 * [ ] The grammar does not construct quantum::ir.
 *
 * [ ] Existing TypeExpr infrastructure remains authoritative.
 *
 * [ ] QuantumRegisters consumes this grammar rather than redefining it.
 *
 * [ ] LogicalQubits consumes the logical type boundary rather than redefining
 *     `logical qubit`.
 *
 * [ ] PhysicalQubits remains separate.
 *
 * [ ] Gates and operations remain separate.
 *
 * [ ] QEC remains separate.
 *
 * [ ] ZQN remains separate.
 *
 * [ ] Routing remains separate.
 *
 * [ ] Scheduling remains separate.
 *
 * [ ] Hardware discovery remains separate.
 *
 * [ ] Resource feasibility remains semantic/runtime work.
 *
 * [ ] Positive parser tests pass.
 *
 * [ ] Negative parser tests pass.
 *
 * [ ] Boundary/scalability tests pass.
 *
 * [ ] Deterministic parsing tests pass.
 *
 * [ ] Round-trip tests pass where a canonical printer exists.
 *
 * [ ] Language-version compatibility tests pass.
 *
 * [ ] Hard-coding audit passes.
 */


/* ============================================================================
 * END
 * ========================================================================== */