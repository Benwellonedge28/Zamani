/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/logical-qubits.g4
 *
 * Purpose:
 *     Canonical source-level grammar for logical quantum resources.
 *
 * Language role:
 *     Express logical-qubit identity, declarations, collections, references,
 *     indexing, ranges, groups, aliases, views, and semantic resource intent.
 *
 * Grammar technology:
 *     ANTLR4 parser fragment
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * A logical qubit is a SOURCE-LEVEL COMPUTATIONAL RESOURCE.
 *
 * This grammar describes:
 *
 *     WHAT logical quantum resources the program refers to.
 *
 * It does NOT determine:
 *
 *     - which physical qubits carry the logical qubit;
 *     - how many physical qubits are required;
 *     - which QEC code is selected;
 *     - code distance;
 *     - decoder;
 *     - syndrome-extraction circuit;
 *     - lattice geometry;
 *     - hardware topology;
 *     - backend;
 *     - QPU;
 *     - calibration;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - noise model;
 *     - physical allocation;
 *     - runtime placement.
 *
 * Those concerns belong to downstream semantic and compilation layers.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Logical-qubit source syntax describes stable computational intent.
 *
 * Example:
 *
 *     logical qubit q;
 *
 * means:
 *
 *     "the program requires/manipulates one logical quantum resource named q."
 *
 * It MUST NOT mean:
 *
 *     physical qubit 0
 *     physical qubits 0..6
 *     QPU device X
 *     surface-code distance 3
 *     a fixed number of physical qubits
 *
 * Physical realization is selected downstream from capabilities, resources,
 * compilation policy, QEC policy, routing, scheduling and hardware state.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There is intentionally NO:
 *
 *     MAX_LOGICAL_QUBITS
 *     MAX_QUBITS
 *     MAX_LOGICAL_REGISTER_SIZE
 *     fixed logical-register width
 *     fixed code distance
 *     fixed number of logical registers
 *     fixed number of physical carriers
 *     fixed topology
 *     fixed backend
 *     fixed device identifier
 *
 * Cardinality and indices are expressions.
 *
 * Therefore:
 *
 *     logical qubit q;
 *     logical qubit q[n];
 *     logical qubit q[width];
 *     logical qubit q[start .. end];
 *
 * are not constrained by this grammar to a particular machine size.
 *
 * The semantic/resource layers determine whether the requested computation
 * can actually be represented, compiled and executed with available
 * resources.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - logical-qubit declarations;
 *     - logical-qubit collections;
 *     - logical-qubit references;
 *     - logical-qubit indexing;
 *     - logical-qubit ranges;
 *     - logical-qubit groups;
 *     - logical-qubit aliases;
 *     - logical-qubit views;
 *     - logical-qubit binding syntax;
 *     - logical-qubit initialization syntax;
 *     - logical-qubit semantic annotations expressed through the surrounding
 *       grammar;
 *     - source-level logical-resource intent.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer tokens;
 *     - identifiers;
 *     - generic expressions;
 *     - general types;
 *     - physical-qubit declarations;
 *     - physical topology;
 *     - QEC algorithms;
 *     - QEC code implementations;
 *     - syndrome extraction;
 *     - decoders;
 *     - ZQN noise models;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - hardware discovery;
 *     - calibration;
 *     - backend selection;
 *     - runtime allocation;
 *     - canonical quantum IR;
 *     - logical-qubit IDs.
 *
 * ============================================================================
 * CANONICAL IDENTITY BOUNDARY
 * ============================================================================
 *
 * Source syntax does not create a new logical-qubit identity type.
 *
 * After semantic analysis, logical-qubit identities MUST lower to the
 * repository's canonical quantum identity:
 *
 *     crate::quantum::ir::qubit::QubitId
 *
 * The grammar MUST NOT introduce:
 *
 *     LogicalQubitId
 *     GrammarQubitId
 *     ParserQubitId
 *     LogicalQubitIndexId
 *
 * or any equivalent duplicate identity.
 *
 * The repository already establishes QubitId as the canonical logical-qubit
 * identity. Logical-qubit grammar is therefore only syntax.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     grammar/lexer/tokens.g4
 *                    |
 *                    v
 *     grammar/core/names.g4
 *                    |
 *                    v
 *     grammar/expressions/*
 *                    |
 *                    v
 *     grammar/types/quantum-types.g4
 *                    |
 *                    v
 *     grammar/quantum/logical-qubits.g4
 *                    |
 *                    v
 *     frontend AST
 *                    |
 *                    v
 *     semantic analysis
 *       |       |       |
 *       |       |       +---- resource/capability analysis
 *       |       +------------ type/effect analysis
 *       +-------------------- name resolution
 *                    |
 *                    v
 *             quantum::ir
 *                    |
 *       +------------+-------------+-------------+
 *       |            |             |             |
 *       v            v             v             v
 *   optimization   routing     scheduling       QEC
 *                                                |
 *                                                v
 *                                               ZQN
 *                                                |
 *                                                v
 *                                         hardware/runtime
 *
 * The dependency direction MUST NOT be reversed.
 *
 * ============================================================================
 * QUANTUM::IR CONTRACT
 * ============================================================================
 *
 * This file MUST NOT import or depend on quantum::ir.
 *
 * Correct:
 *
 *     source
 *       -> parser
 *       -> AST
 *       -> semantic analysis
 *       -> quantum::ir
 *
 * Incorrect:
 *
 *     parser
 *       -> quantum::ir
 *       -> parser
 *
 * The grammar therefore remains reusable independently of the quantum
 * compilation implementation.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file consumes the canonical Zamani lexer vocabulary.
 *
 * Relevant existing tokens include:
 *
 *     K_LOGICAL
 *     K_QUBIT
 *
 *     IDENTIFIER
 *
 *     LPAREN
 *     RPAREN
 *     LBRACKET
 *     RBRACKET
 *     COMMA
 *     COLON
 *     SEMICOLON
 *
 *     ASSIGN
 *     DOT_DOT
 *     DOT_DOT_EQ
 *
 *     PLUS
 *     MINUS
 *
 *     AT
 *
 * This file MUST NOT redeclare lexer tokens.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Quantum type syntax is owned by:
 *
 *     grammar/quantum/quantum-types.g4
 *
 * In particular, the repository already defines:
 *
 *     logicalQubitType
 *
 * as:
 *
 *     logical qubit
 *
 * This file therefore does not create another logical-qubit type grammar.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * General expressions remain owned by the expression grammar.
 *
 * This file only identifies places where an expression is required:
 *
 *     - collection extent;
 *     - index;
 *     - range boundary;
 *     - initializer;
 *     - alias/view target expressions where appropriate.
 *
 * Logical-qubit syntax MUST NOT create a second expression language.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser produces syntax structure only.
 *
 * The frontend AST should preserve:
 *
 *     - source span;
 *     - declared logical-qubit name;
 *     - optional collection extent;
 *     - optional type annotation;
 *     - optional initializer;
 *     - alias/view relationships;
 *     - index/range/group structure.
 *
 * The AST MUST NOT contain physical allocation decisions.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis MUST determine:
 *
 *     - whether the name is declared;
 *     - whether it is a logical-qubit binding;
 *     - whether the extent has a valid cardinality type;
 *     - whether the extent is compile-time or runtime;
 *     - whether an index is valid;
 *     - whether a range is valid;
 *     - whether an alias is legal;
 *     - whether a view is legal;
 *     - whether copying violates quantum resource semantics;
 *     - whether ownership/linearity rules apply;
 *     - whether the requested logical resources are satisfiable.
 *
 * The grammar itself MUST NOT perform these checks.
 *
 * ============================================================================
 * QEC CONTRACT
 * ============================================================================
 *
 * Logical-qubit syntax may identify a logical quantum resource.
 *
 * It MUST NOT select or implement:
 *
 *     - surface code;
 *     - repetition code;
 *     - color code;
 *     - LDPC code;
 *     - subsystem code;
 *     - code distance;
 *     - decoder;
 *     - syndrome extraction;
 *     - logical gate synthesis.
 *
 * If the language later supports source-level QEC requirements, those should
 * be expressed through the resource/capability/QEC policy grammar and lowered
 * into the QEC subsystem rather than embedding QEC implementation in this
 * file.
 *
 * ============================================================================
 * ZQN CONTRACT
 * ============================================================================
 *
 * Logical-qubit syntax does not define noise.
 *
 * ZQN remains responsible for:
 *
 *     - fault classification;
 *     - fault locations;
 *     - correlated faults;
 *     - leakage;
 *     - loss;
 *     - erasure;
 *     - noise channels;
 *     - fault semantics;
 *     - calibration-related noise information.
 *
 * This grammar contains no ZQN model.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *     MAX_LOGICAL_QUBITS
 *     MAX_QUBITS
 *     32
 *     64
 *     fixed register width
 *     fixed code distance
 *     fixed physical expansion factor
 *     fixed QPU
 *     fixed device ID
 *     fixed topology
 *     fixed number of physical carriers
 *
 * Numeric literals occurring in user expressions are data supplied by the
 * program. They are not grammar-level resource limits.
 *
 * ============================================================================
 */


/* ============================================================================
 * 1. LOGICAL QUBIT DECLARATION
 * ========================================================================== */

/*
 * Canonical single logical-qubit declaration:
 *
 *     logical qubit q;
 *
 * Typed form:
 *
 *     logical qubit q : logical qubit;
 *
 * Initialized form:
 *
 *     logical qubit q = initializer;
 *
 * Typed and initialized:
 *
 *     logical qubit q : logical qubit = initializer;
 *
 * The type system determines whether the annotation is meaningful.
 */
logicalQubitDeclaration
    : logicalQubitKeyword
      identifier
      logicalQubitTypeAnnotation?
      logicalQubitInitializer?
      SEMICOLON
    ;


/* ============================================================================
 * 2. LOGICAL QUBIT KEYWORD
 * ========================================================================== */

/*
 * Kept separate so future grammar composition has one stable entry point.
 */
logicalQubitKeyword
    : K_LOGICAL
      K_QUBIT
    ;


/* ============================================================================
 * 3. LOGICAL QUBIT TYPE ANNOTATION
 * ========================================================================== */

/*
 * Quantum type ownership remains in quantum-types.g4.
 *
 * Example:
 *
 *     logical qubit q : logical qubit;
 *
 *     logical qubit q : MyLogicalQubitType;
 */
logicalQubitTypeAnnotation
    : COLON quantumType
    ;


/* ============================================================================
 * 4. LOGICAL QUBIT INITIALIZER
 * ========================================================================== */

/*
 * Initialization is syntactic only.
 *
 * State preparation semantics belong downstream.
 */
logicalQubitInitializer
    : ASSIGN expression
    ;


/* ============================================================================
 * 5. LOGICAL QUBIT COLLECTION DECLARATION
 * ========================================================================== */

/*
 * Scalable collection form:
 *
 *     logical qubit q[n];
 *
 *     logical qubit q[width];
 *
 *     logical qubit q[algorithm_width + ancilla_count];
 *
 * The extent is an expression.
 *
 * There is no parser-level maximum.
 */
logicalQubitCollectionDeclaration
    : logicalQubitKeyword
      identifier
      LBRACKET
      logicalQubitExtentExpression
      RBRACKET
      logicalQubitTypeAnnotation?
      logicalQubitInitializer?
      SEMICOLON
    ;


/* ============================================================================
 * 6. LOGICAL QUBIT EXTENT
 * ========================================================================== */

/*
 * Extent semantics are delegated to the general expression/type system.
 */
logicalQubitExtentExpression
    : expression
    ;


/* ============================================================================
 * 7. LOGICAL QUBIT DECLARATION FAMILY
 * ========================================================================== */

/*
 * Single declaration and collection declaration share one integration point.
 *
 * This is the preferred rule for quantum declaration dispatch.
 */
logicalQubitDeclarationFamily
    : logicalQubitDeclaration
    | logicalQubitCollectionDeclaration
    ;


/* ============================================================================
 * 8. LOGICAL QUBIT REFERENCE
 * ========================================================================== */

/*
 * A bare logical-qubit reference:
 *
 *     q
 *
 * Identity resolution is semantic.
 */
logicalQubitReference
    : identifier
    ;


/* ============================================================================
 * 9. LOGICAL QUBIT INDEX
 * ========================================================================== */

/*
 * Examples:
 *
 *     q[i]
 *
 *     q[index]
 *
 * The index is an expression.
 *
 * No integer width or maximum index is encoded here.
 */
logicalQubitIndex
    : logicalQubitReference
      LBRACKET
      expression
      RBRACKET
    ;


/* ============================================================================
 * 10. LOGICAL QUBIT RANGE
 * ========================================================================== */

/*
 * Examples:
 *
 *     q[start .. end]
 *
 *     q[start ..= end]
 *
 * Range direction, bounds, validity and materialization are semantic issues.
 */
logicalQubitRange
    : logicalQubitReference
      LBRACKET
      expression
      logicalQubitRangeOperator
      expression
      RBRACKET
    ;


logicalQubitRangeOperator
    : DOT_DOT
    | DOT_DOT_EQ
    ;


/* ============================================================================
 * 11. LOGICAL QUBIT GROUP
 * ========================================================================== */

/*
 * Example:
 *
 *     (q0, q1, q2)
 *
 * The grammar imposes no maximum group size.
 *
 * The actual identities are resolved semantically.
 */
logicalQubitGroup
    : LPAREN
      logicalQubitReferenceList
      RPAREN
    ;


logicalQubitReferenceList
    : logicalQubitReference
      (COMMA logicalQubitReference)*
    ;


/* ============================================================================
 * 12. LOGICAL QUBIT TARGET
 * ========================================================================== */

/*
 * This is the integration boundary for gates, operations, measurements,
 * observables, dynamic circuits and other quantum constructs that specifically
 * require logical resources.
 *
 * It deliberately does not contain physical-qubit syntax.
 */
logicalQubitTarget
    : logicalQubitReference
    | logicalQubitIndex
    | logicalQubitRange
    | logicalQubitGroup
    ;


/* ============================================================================
 * 13. LOGICAL QUBIT TARGET LIST
 * ========================================================================== */

/*
 * The number of targets is not fixed.
 */
logicalQubitTargetList
    : logicalQubitTarget
      (COMMA logicalQubitTarget)*
    ;


/* ============================================================================
 * 14. LOGICAL QUBIT ALIAS
 * ========================================================================== */

/*
 * Source-level identity alias:
 *
 *     logical qubit alias = q;
 *
 * This does not allocate or copy a physical resource.
 *
 * Whether an alias is legal is determined by quantum ownership/linearity
 * semantics.
 */
logicalQubitAliasDeclaration
    : logicalQubitKeyword
      identifier
      ASSIGN
      logicalQubitReference
      SEMICOLON
    ;


/* ============================================================================
 * 15. LOGICAL QUBIT VIEW
 * ========================================================================== */

/*
 * A view refers to a logical resource without implying physical copying.
 *
 * Examples:
 *
 *     logical qubit view = q[i];
 *
 *     logical qubit view = q[start .. end];
 *
 * Ownership, borrowing, aliasing and lifetime semantics are downstream.
 */
logicalQubitViewDeclaration
    : logicalQubitKeyword
      identifier
      ASSIGN
      logicalQubitSelection
      SEMICOLON
    ;


/* ============================================================================
 * 16. LOGICAL QUBIT SELECTION
 * ========================================================================== */

/*
 * Selection syntax is deliberately limited to logical-resource forms.
 *
 * Generic collection-selection functions belong to the general expression
 * language rather than this grammar.
 */
logicalQubitSelection
    : logicalQubitIndex
    | logicalQubitRange
    | logicalQubitReference
    ;


/* ============================================================================
 * 17. LOGICAL QUBIT RESOURCE BINDING
 * ========================================================================== */

/*
 * Integration point for future resource/capability grammar.
 *
 * The grammar records a symbolic logical-resource binding only.
 *
 * It does not choose:
 *
 *     - QEC code;
 *     - physical expansion;
 *     - hardware;
 *     - topology;
 *     - backend.
 */
logicalQubitResourceBinding
    : logicalQubitKeyword
      identifier
      COLON
      quantumType
      SEMICOLON
    ;


/* ============================================================================
 * 18. LOGICAL QUBIT DECLARATION ELEMENT
 * ========================================================================== */

/*
 * Central dispatch point used by the quantum declaration grammar.
 */
logicalQubitElement
    : logicalQubitDeclarationFamily
    | logicalQubitAliasDeclaration
    | logicalQubitViewDeclaration
    | logicalQubitResourceBinding
    ;


/* ============================================================================
 * 19. LOGICAL QUBIT EXPRESSION
 * ========================================================================== */

/*
 * Stable integration point for operations, measurements and observables.
 *
 * This rule intentionally does not fall back to arbitrary expression.
 * Doing so would allow a classical value to silently masquerade as a logical
 * quantum resource.
 */
logicalQubitExpression
    : logicalQubitReference
    | logicalQubitIndex
    | logicalQubitRange
    | logicalQubitGroup
    ;


/* ============================================================================
 * 20. LOGICAL QUBIT INITIALIZATION TARGET
 * ========================================================================== */

/*
 * Initialization may eventually be connected to preparation/state grammar.
 *
 * For now the expression is preserved without imposing a particular
 * state-representation model.
 */
logicalQubitInitializationTarget
    : logicalQubitExpression
    ;


/* ============================================================================
 * 21. LOGICAL QUBIT PARAMETER
 * ========================================================================== */

/*
 * Quantum functions/circuits may accept logical resources as parameters.
 *
 * Parameter type semantics belong to quantum-types.g4 and the type system.
 */
logicalQubitParameterType
    : quantumType
    ;


/* ============================================================================
 * 22. LOGICAL QUBIT COLLECTION ELEMENT
 * ========================================================================== */

/*
 * This rule is useful to downstream quantum operation grammars that need to
 * accept either an entire logical collection or one member.
 */
logicalQubitCollectionElement
    : logicalQubitReference
    | logicalQubitIndex
    ;


/* ============================================================================
 * 23. LOGICAL QUBIT RANGE ELEMENT
 * ========================================================================== */

/*
 * Kept separate because range handling may later receive richer semantic
 * validation without changing the public target grammar.
 */
logicalQubitRangeElement
    : logicalQubitRange
    ;


/* ============================================================================
 * 24. LOGICAL QUBIT GROUP ELEMENT
 * ========================================================================== */

logicalQubitGroupElement
    : logicalQubitGroup
    ;


/* ============================================================================
 * 25. LOGICAL QUBIT SEMANTIC BOUNDARY
 * ========================================================================== */

/*
 * Everything after this grammar boundary is semantic rather than syntactic:
 *
 *     logical source binding
 *             |
 *             v
 *     name resolution
 *             |
 *             v
 *     logical-resource validation
 *             |
 *             v
 *     canonical QubitId
 *             |
 *             v
 *     quantum::ir
 *
 * No parser rule below this boundary may introduce a physical allocation.
 */


/* ============================================================================
 * 26. INTENTIONAL NON-OWNERSHIP OF PHYSICAL QUBITS
 * ========================================================================== */

/*
 * DO NOT add productions such as:
 *
 *     physicalQubit0
 *     physicalQubit1
 *     physicalQubit[32]
 *     qpu0
 *     qpu1
 *
 * or any fixed hardware vocabulary.
 *
 * Physical resources belong to:
 *
 *     grammar/quantum/physical-qubits.g4
 *     grammar/hardware/*
 *
 * and ultimately the hardware/resource/compiler layers.
 */


/* ============================================================================
 * 27. INTENTIONAL NON-OWNERSHIP OF QEC
 * ========================================================================== */

/*
 * DO NOT add productions such as:
 *
 *     logical qubit q using surface code distance 3
 *
 * as a built-in implementation rule.
 *
 * If Zamani eventually provides source-level QEC policy syntax, it must be
 * represented by the dedicated QEC/resource/capability grammar and interpreted
 * by the QEC subsystem.
 *
 * Logical-qubits.g4 identifies the logical computational resource only.
 */


/* ============================================================================
 * 28. INTENTIONAL NON-OWNERSHIP OF ZQN
 * ========================================================================== */

/*
 * This file does not define:
 *
 *     noise;
 *     fault;
 *     error channel;
 *     leakage;
 *     loss;
 *     erasure;
 *     correlation;
 *     calibration error.
 *
 * Those remain ZQN semantics.
 */


/* ============================================================================
 * 29. INTENTIONAL NON-OWNERSHIP OF ROUTING
 * ========================================================================== */

/*
 * A logical-qubit reference never implies:
 *
 *     physical qubit placement;
 *     nearest-neighbour mapping;
 *     coupling graph;
 *     SWAP insertion;
 *     routing strategy.
 *
 * Routing consumes canonical semantic/IR information downstream.
 */


/* ============================================================================
 * 30. INTENTIONAL NON-OWNERSHIP OF SCHEDULING
 * ========================================================================== */

/*
 * A logical-qubit declaration never specifies:
 *
 *     duration;
 *     start time;
 *     end time;
 *     pulse time;
 *     alignment grid;
 *     scheduling priority.
 *
 * Scheduling consumes semantic operations and hardware/resource capabilities.
 */


/* ============================================================================
 * 31. INTEGRATION CONTRACT FOR quantum/qubits.g4
 * ========================================================================== */

/*
 * grammar/quantum/qubits.g4 currently contains logical-qubit productions.
 *
 * To establish single ownership, those logical-specific productions MUST be
 * removed from qubits.g4 and replaced at its declaration/target dispatch
 * boundary with references to the rules in this file.
 *
 * Specifically, qubits.g4 MUST NOT independently define:
 *
 *     logicalQubitDeclaration
 *     logicalQubitType
 *
 * when this file and quantum-types.g4 are authoritative for those concepts.
 *
 * Generic qubit syntax remains in qubits.g4:
 *
 *     qubit
 *     qubit register
 *     qubit reference
 *     generic qubit indexing
 *
 * Logical syntax remains here.
 */


/* ============================================================================
 * 32. INTEGRATION CONTRACT FOR quantum/quantum-types.g4
 * ========================================================================== */

/*
 * quantum-types.g4 remains authoritative for:
 *
 *     logicalQubitType
 *
 * This file consumes:
 *
 *     quantumType
 *
 * and does not redefine:
 *
 *     logicalQubitType
 *
 * This prevents duplicate type ownership.
 */


/* ============================================================================
 * 33. INTEGRATION CONTRACT FOR quantum/operations.g4
 * ========================================================================== */

/*
 * Operations that specifically require logical operands SHOULD consume:
 *
 *     logicalQubitTarget
 *     logicalQubitTargetList
 *
 * instead of redefining logical-qubit operand syntax.
 *
 * Generic quantum operations may continue to consume the generic quantum
 * target rules from qubits.g4.
 */


/* ============================================================================
 * 34. INTEGRATION CONTRACT FOR quantum/measurement.g4
 * ========================================================================== */

/*
 * Measurements that explicitly operate on logical resources SHOULD consume:
 *
 *     logicalQubitTarget
 *
 * rather than introducing another logical-qubit reference syntax.
 *
 * Measurement semantics remain outside this grammar.
 */


/* ============================================================================
 * 35. INTEGRATION CONTRACT FOR quantum/error-correction.g4
 * ========================================================================== */

/*
 * QEC grammar may consume logicalQubitExpression to identify the logical
 * resources to which a QEC policy applies.
 *
 * It MUST NOT modify the identity semantics of this file.
 *
 * Example conceptual boundary:
 *
 *     logical resource
 *          |
 *          +--> QEC policy
 *          |
 *          +--> semantic analysis
 *          |
 *          +--> canonical quantum::ir
 */


/* ============================================================================
 * 36. INTEGRATION CONTRACT FOR quantum/physical-qubits.g4
 * ========================================================================== */

/*
 * physical-qubits.g4 owns physical resource syntax.
 *
 * This file may be referenced by semantic lowering, but logical-qubits.g4
 * MUST NOT import or depend on physical-qubits.g4 to define logical identity.
 *
 * Correct relationship:
 *
 *     logical source
 *          |
 *          v
 *     semantic representation
 *          |
 *          +---- physical realization
 *
 * Not:
 *
 *     logical grammar
 *          |
 *          v
 *     physical grammar
 *          |
 *          v
 *     logical meaning
 */


/* ============================================================================
 * 37. INTEGRATION CONTRACT FOR HARDWARE
 * ========================================================================== */

/*
 * No hardware grammar is a dependency of this file.
 *
 * Hardware capabilities may constrain whether a logical-qubit computation can
 * execute, but that is a downstream capability/resource decision.
 */


/* ============================================================================
 * 38. INTEGRATION CONTRACT FOR RESOURCE GRAMMAR
 * ========================================================================== */

/*
 * Resource grammar may consume logical-qubit declarations and references as
 * semantic requirements.
 *
 * Resource grammar owns:
 *
 *     requirement;
 *     capability;
 *     constraint;
 *     preference;
 *     placement;
 *     performance;
 *     scalability;
 *     portability.
 *
 * This file owns none of those policy semantics.
 */


/* ============================================================================
 * 39. INTEGRATION CONTRACT FOR FRONTEND AST
 * ========================================================================== */

/*
 * Every logical-qubit parser node should retain:
 *
 *     source span;
 *     identifier;
 *     declaration kind;
 *     optional extent;
 *     optional type annotation;
 *     optional initializer;
 *     alias/view relationship.
 *
 * The frontend must lower these into existing AST structures rather than
 * inventing a parallel quantum AST hierarchy.
 */


/* ============================================================================
 * 40. INTEGRATION CONTRACT FOR quantum::ir
 * ========================================================================== */

/*
 * Logical source bindings ultimately map to:
 *
 *     crate::quantum::ir::qubit::QubitId
 *
 * The grammar itself never constructs QubitId.
 *
 * No grammar-generated value may contain:
 *
 *     physical address;
 *     backend ID;
 *     topology location;
 *     calibration identity.
 */


/* ============================================================================
 * 41. DETERMINISM
 * ========================================================================== */

/*
 * These productions are deterministic with respect to their lexical
 * prefixes:
 *
 *     logical qubit ...
 *
 * Collection syntax is distinguished by:
 *
 *     LBRACKET
 *
 * and alias/view syntax by:
 *
 *     ASSIGN
 *
 * No unordered grammar construct is introduced.
 */


/* ============================================================================
 * 42. SCALABILITY TEST CONTRACT
 * ========================================================================== */

/*
 * The grammar test suite MUST include:
 *
 *     logical qubit q;
 *
 *     logical qubit q[n];
 *
 *     logical qubit q[width];
 *
 *     logical qubit q[start + count];
 *
 *     logical qubit q[very_large_symbolic_extent];
 *
 *     logical qubit q[index];
 *
 *     logical qubit q[start .. end];
 *
 *     logical qubit q[start ..= end];
 *
 *     (q0, q1, q2, ...);
 *
 * The test generator may produce arbitrarily large collections subject only
 * to test-runner resource availability.
 *
 * The grammar must not reject a program because a machine-size constant was
 * exceeded.
 */


/* ============================================================================
 * 43. NEGATIVE TEST CONTRACT
 * ========================================================================== */

/*
 * The parser/semantic test suite MUST reject or diagnose:
 *
 *     logical;
 *
 *     qubit logical;
 *
 *     logical qubit;
 *
 *     logical qubit q[;
 *
 *     logical qubit q[];
 *
 *     logical qubit q[...];
 *
 *     logical qubit q[start ..];
 *
 *     logical qubit q =;
 *
 *     logical qubit = q;
 *
 * The exact diagnostic classification belongs to the parser/semantic
 * diagnostic layer.
 */


/* ============================================================================
 * 44. SEMANTIC NEGATIVE TESTS
 * ========================================================================== */

/*
 * These may parse but MUST be rejected by semantic analysis when invalid:
 *
 *     logical qubit q[-1];
 *
 *     logical qubit q[non_integral_value];
 *
 *     invalid logical-qubit alias;
 *
 *     illegal duplicated ownership;
 *
 *     invalid logical-qubit use after consuming ownership;
 *
 *     invalid conversion of classical data into logical resource;
 *
 *     incompatible logical-qubit type.
 *
 * Such checks MUST NOT be embedded as parser actions.
 */


/* ============================================================================
 * 45. ROUND-TRIP CONTRACT
 * ========================================================================== */

/*
 * Where the repository provides a canonical formatter:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> formatter
 *       -> parser
 *
 * must preserve logical-qubit semantic structure.
 *
 * Formatting differences are acceptable.
 *
 * Semantic identity is not.
 */


/* ============================================================================
 * 46. RUST SAFETY CONTRACT
 * ========================================================================== */

/*
 * This grammar contains no embedded Rust actions.
 *
 * Generated Rust integration MUST use:
 *
 *     Rust 1.97 / 1.97.1
 *     Rust 2021
 *
 * and repository policy:
 *
 *     #![deny(unsafe_code)]
 *
 * No unsafe Rust is required or permitted for this grammar.
 */


/* ============================================================================
 * 47. COMPLETION CRITERIA
 * ========================================================================== */

/*
 * logical-qubits.g4 is COMPLETE only when:
 *
 * [ ] logical-qubit declarations parse;
 * [ ] logical-qubit collections parse;
 * [ ] symbolic extents parse;
 * [ ] logical references parse;
 * [ ] indexing parses;
 * [ ] ranges parse;
 * [ ] groups parse;
 * [ ] aliases parse;
 * [ ] views parse;
 * [ ] logical targets have one canonical grammar entry point;
 * [ ] no machine-size limit exists;
 * [ ] no physical identity is embedded;
 * [ ] no QEC implementation is embedded;
 * [ ] no ZQN model is embedded;
 * [ ] no routing is embedded;
 * [ ] no scheduling is embedded;
 * [ ] no hardware selection is embedded;
 * [ ] no duplicate logical type grammar exists;
 * [ ] no duplicate logical-qubit declaration grammar remains in qubits.g4;
 * [ ] frontend AST integration is defined;
 * [ ] semantic lowering to canonical QubitId is defined;
 * [ ] quantum::ir remains downstream;
 * [ ] positive tests exist;
 * [ ] negative tests exist;
 * [ ] boundary tests exist;
 * [ ] scalability tests exist;
 * [ ] deterministic parsing tests exist;
 * [ ] round-trip tests exist where supported;
 * [ ] Rust integration remains compatible with Rust 1.97/1.97.1;
 * [ ] no unsafe code is introduced.
 */


/* ============================================================================
 * END OF FILE
 * ============================================================================
 */