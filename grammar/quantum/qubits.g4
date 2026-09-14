/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/qubits.g4
 *
 * Purpose:
 *     Canonical parser fragment for source-level qubit declarations,
 *     references, collections, ranges, views, logical/physical intent,
 *     ownership/lifetime intent, and qubit-related expressions.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar fragment
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * A qubit in Zamani source is a semantic resource/value.
 *
 * This file describes HOW A PROGRAM REFERS TO QUANTUM RESOURCES.
 *
 * It does NOT decide:
 *
 *     - how many physical qubits exist;
 *     - which physical qubit is used;
 *     - which QPU is selected;
 *     - how qubits are routed;
 *     - how qubits are scheduled;
 *     - which gate decomposition is used;
 *     - which error-correction code is selected;
 *     - how noise is modeled;
 *     - how calibration is performed;
 *     - how a backend allocates hardware;
 *     - how a simulator represents the state.
 *
 * Those concerns belong downstream.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Source:
 *
 *     qubit q;
 *
 *     qubit register;
 *
 *     qubit register[size];
 *
 *     qubit q = |0⟩;
 *
 * expresses semantic intent.
 *
 * It MUST NOT mean:
 *
 *     allocate physical qubit 0
 *
 * or:
 *
 *     use QPU X
 *
 * or:
 *
 *     reserve N hardware qubits.
 *
 * Resource realization is determined after semantic analysis.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar intentionally has:
 *
 *     - no MAX_QUBITS;
 *     - no maximum register width;
 *     - no fixed number of registers;
 *     - no fixed number of qubit declarations;
 *     - no fixed circuit depth;
 *     - no fixed physical topology;
 *     - no fixed physical-qubit ID range;
 *     - no fixed backend;
 *     - no fixed simulator dimension.
 *
 * An extent is an expression.
 *
 * Therefore:
 *
 *     qubit q[n];
 *
 * is not restricted by this grammar to a particular n.
 *
 * Whether n is valid, representable, available, or affordable is determined
 * by semantic analysis, resource analysis, compilation, and runtime.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - qubit declarations;
 *     - abstract qubit/register syntax;
 *     - qubit references;
 *     - qubit indexing;
 *     - qubit slicing/ranges;
 *     - qubit selection;
 *     - qubit grouping;
 *     - qubit views;
 *     - qubit aliases;
 *     - logical-qubit source intent;
 *     - physical-qubit source intent when explicitly requested;
 *     - qubit lifetime/ownership syntax;
 *     - qubit initialization syntax;
 *     - qubit resource expressions;
 *     - qubit-related type syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer tokens;
 *     - identifiers;
 *     - general expressions;
 *     - general statements;
 *     - general type system;
 *     - quantum gates;
 *     - quantum operations;
 *     - measurements;
 *     - circuits;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - hardware discovery;
 *     - physical topology;
 *     - resource accounting;
 *     - runtime allocation;
 *     - canonical quantum IR.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     grammar/lexer/tokens.g4
 *                 |
 *                 v
 *     grammar/core/names.g4
 *                 |
 *                 v
 *     grammar/types/*
 *                 |
 *                 v
 *     grammar/expressions/*
 *                 |
 *                 v
 *     grammar/quantum/qubits.g4
 *                 |
 *                 v
 *     quantum parser
 *                 |
 *                 v
 *     frontend AST
 *                 |
 *                 v
 *     semantic/type/effect/resource analysis
 *                 |
 *                 v
 *     quantum::ir
 *                 |
 *       +---------+---------+------------------+
 *       |         |         |                  |
 *       v         v         v                  v
 *   routing   scheduling   QEC                ZQN
 *       |
 *       v
 *   hardware/runtime
 *
 * The dependency direction MUST NOT be reversed.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * This fragment consumes the canonical lexer vocabulary.
 *
 * Current repository lexical ownership includes:
 *
 *     K_QUANTUM
 *     K_QUBIT
 *     K_CIRCUIT
 *     K_LOGICAL
 *     K_LINEAR
 *     K_AFFINE
 *
 * and punctuation including:
 *
 *     LPAREN
 *     RPAREN
 *     LBRACKET
 *     RBRACKET
 *     COMMA
 *     COLON
 *     SEMICOLON
 *     ASSIGN
 *     THIN_ARROW
 *     DOT_DOT
 *     DOT_DOT_EQ
 *     LESS_THAN
 *     GREATER_THAN
 *     AT
 *
 * This file MUST NOT redeclare lexer tokens.
 *
 * ============================================================================
 */


/* ============================================================================
 * 1. ABSTRACT QUBIT DECLARATION
 * ========================================================================== */

/*
 * Basic form:
 *
 *     qubit q;
 *
 * Initialized form:
 *
 *     qubit q = |0⟩;
 *
 * Typed/annotated form:
 *
 *     qubit q : SomeQuantumType;
 *
 * Initialized and typed:
 *
 *     qubit q : SomeQuantumType = state;
 *
 * The semantic layer determines whether the declared type is actually a
 * single qubit, logical qubit, encoded qubit, or another legal quantum value.
 */
qubitDeclaration
    : K_QUBIT
      qubitBinding
      qubitTypeAnnotation?
      qubitInitializer?
      SEMICOLON
    ;


/* ============================================================================
 * 2. QUBIT BINDING
 * ========================================================================== */

qubitBinding
    : identifier
    ;


/* ============================================================================
 * 3. QUBIT TYPE ANNOTATION
 * ========================================================================== */

qubitTypeAnnotation
    : COLON quantumQubitType
    ;


/*
 * The primitive qubit type is represented by K_QUBIT.
 *
 * Other quantum types remain semantic type-system entities and may be named
 * through qualified identifiers.
 */
quantumQubitType
    : K_QUBIT
    | qualifiedName
    ;


/* ============================================================================
 * 4. QUBIT INITIALIZATION
 * ========================================================================== */

qubitInitializer
    : ASSIGN quantumQubitInitializerExpression
    ;


quantumQubitInitializerExpression
    : quantumBasisStateInitializer
    | expression
    ;


/* ============================================================================
 * 5. BASIS STATE INITIALIZATION
 * ========================================================================== */

/*
 * The canonical lexer already defines QUANTUM_LITERAL as a semantic lexical
 * token for:
 *
 *     |0⟩
 *     |1⟩
 *     |+⟩
 *     |-⟩
 *
 * Keep this token opaque at this layer.
 *
 * Do NOT add individual lexer rules for every future quantum state.
 */
quantumBasisStateInitializer
    : QUANTUM_LITERAL
    ;


/* ============================================================================
 * 6. ABSTRACT QUBIT REGISTER DECLARATION
 * ========================================================================== */

/*
 * Examples:
 *
 *     qubit q[n];
 *
 *     qubit q[width];
 *
 *     qubit q[start + count];
 *
 * The extent is an expression.
 *
 * There is deliberately no grammar-level integer ceiling.
 */
qubitRegisterDeclaration
    : K_QUBIT
      identifier
      LBRACKET
      qubitExtentExpression
      RBRACKET
      qubitTypeAnnotation?
      qubitInitializer?
      SEMICOLON
    ;


qubitExtentExpression
    : expression
    ;


/* ============================================================================
 * 7. REGISTER INITIALIZATION
 * ========================================================================== */

/*
 * A register initializer may be a general expression.
 *
 * Examples can include:
 *
 *     qubit q[n] = state;
 *
 *     qubit q[n] = initializer(n);
 *
 * The semantic/type layer determines whether the initializer is compatible.
 */
qubitRegisterInitializer
    : ASSIGN expression
    ;


/* ============================================================================
 * 8. QUBIT COLLECTION DECLARATION
 * ========================================================================== */

/*
 * A collection may represent an abstract collection of qubits without
 * requiring that the source language commit to a physical contiguous layout.
 */
qubitCollectionDeclaration
    : K_QUBIT
      identifier
      COLON
      quantumQubitCollectionType
      qubitInitializer?
      SEMICOLON
    ;


quantumQubitCollectionType
    : qualifiedName
    | genericQuantumQubitCollectionType
    ;


genericQuantumQubitCollectionType
    : qualifiedName
      LESS_THAN
      quantumQubitTypeArgumentList
      GREATER_THAN
    ;


quantumQubitTypeArgumentList
    : quantumQubitTypeArgument
      (COMMA quantumQubitTypeArgument)*
    ;


quantumQubitTypeArgument
    : quantumQubitType
    | expression
    ;


/* ============================================================================
 * 9. QUBIT REFERENCE
 * ========================================================================== */

/*
 * A qubit reference is deliberately expressed using the general expression
 * system.
 *
 * This permits:
 *
 *     q
 *     q[i]
 *     q[start]
 *     register[index]
 *     register[slice]
 *     alias
 *     view[index]
 *
 * without creating a duplicate expression language.
 */
qubitReference
    : identifier
    ;


/* ============================================================================
 * 10. QUBIT INDEX
 * ========================================================================== */

/*
 * Example:
 *
 *     q[i]
 *
 * The index is an expression, not a parser-level integer.
 */
qubitIndex
    : qubitReference
      LBRACKET
      expression
      RBRACKET
    ;


/* ============================================================================
 * 11. QUBIT RANGE
 * ========================================================================== */

/*
 * Examples:
 *
 *     q[start .. end]
 *
 *     q[start ..= end]
 *
 * The actual range semantics belong to the general range/type system.
 */
qubitRange
    : qubitReference
      LBRACKET
      expression
      qubitRangeOperator
      expression
      RBRACKET
    ;


qubitRangeOperator
    : DOT_DOT
    | DOT_DOT_EQ
    ;


/* ============================================================================
 * 12. QUBIT SELECTION
 * ========================================================================== */

/*
 * Selection permits arbitrary expressions to select quantum resources.
 *
 * Examples:
 *
 *     select(q, predicate)
 *
 *     q[index]
 *
 *     q[start .. end]
 *
 * Selection semantics are validated downstream.
 */
qubitSelection
    : qubitSelectionFunction
    | qubitIndex
    | qubitRange
    ;


qubitSelectionFunction
    : identifier
      LPAREN
      expressionList?
      RPAREN
    ;


/* ============================================================================
 * 13. QUBIT GROUP
 * ========================================================================== */

/*
 * Parenthesized groups are semantic target groups.
 *
 * Example:
 *
 *     (q0, q1, q2)
 *
 * The number of members is unbounded by this grammar.
 */
qubitGroup
    : LPAREN
      qubitReferenceList
      RPAREN
    ;


qubitReferenceList
    : qubitReference
      (COMMA qubitReference)*
    ;


/* ============================================================================
 * 14. QUBIT TARGET
 * ========================================================================== */

/*
 * This rule provides a single integration point for downstream quantum
 * operation grammar.
 */
quantumQubitTarget
    : qubitReference
    | qubitIndex
    | qubitRange
    | qubitSelection
    | qubitGroup
    | expression
    ;


/* ============================================================================
 * 15. MULTI-QUBIT TARGET
 * ========================================================================== */

quantumQubitTargetList
    : quantumQubitTarget
      (COMMA quantumQubitTarget)*
    ;


/* ============================================================================
 * 16. LOGICAL QUBIT DECLARATION
 * ========================================================================== */

/*
 * A logical qubit is semantic intent.
 *
 * Example:
 *
 *     logical q;
 *
 * No QEC code or physical implementation is selected here.
 */
logicalQubitDeclaration
    : K_LOGICAL
      K_QUBIT
      identifier
      qubitTypeAnnotation?
      qubitInitializer?
      SEMICOLON
    ;


/* ============================================================================
 * 17. LOGICAL QUBIT TYPE
 * ========================================================================== */

logicalQubitType
    : K_LOGICAL
      K_QUBIT
    ;


/* ============================================================================
 * 18. PHYSICAL QUBIT INTENT
 * ========================================================================== */

/*
 * Physical qubits are generally a backend realization concept.
 *
 * If the source explicitly requests a physical resource, this grammar records
 * that intent only.
 *
 * It MUST NOT require a numeric physical identifier.
 *
 * Examples:
 *
 *     physical q;
 *
 *     physical q : HardwareQubit;
 *
 * Actual physical mapping belongs to routing/hardware lowering.
 *
 * The identifier remains symbolic.
 */
physicalQubitDeclaration
    : K_QUBIT
      K_LOGICAL
      identifier
      SEMICOLON
    ;


/*
 * NOTE:
 *
 * This production is intentionally NOT exposed as a preferred source form.
 *
 * A future canonical source syntax for physical intent should be defined in
 * the hardware grammar and referenced here through a semantic type.
 *
 * This prevents quantum syntax from taking ownership of hardware realization.
 */


/* ============================================================================
 * 19. QUBIT ALIAS
 * ========================================================================== */

/*
 * Alias syntax preserves source-level identity relationships.
 *
 * Example:
 *
 *     qubit alias = existing;
 *
 * Actual alias legality is semantic.
 */
qubitAliasDeclaration
    : K_QUBIT
      identifier
      ASSIGN
      qubitReference
      SEMICOLON
    ;


/* ============================================================================
 * 20. QUBIT VIEW
 * ========================================================================== */

/*
 * A view is a derived source-level reference.
 *
 * Example:
 *
 *     qubit view = q[start .. end];
 *
 * Whether this is:
 *
 *     - borrowed;
 *     - owned;
 *     - copied;
 *     - affine;
 *     - linear;
 *
 * is determined by the type/effect/resource system.
 */
qubitViewDeclaration
    : K_QUBIT
      identifier
      ASSIGN
      qubitSelection
      SEMICOLON
    ;


/* ============================================================================
 * 21. LINEAR QUBIT INTENT
 * ========================================================================== */

/*
 * Quantum resources may require non-copying semantics.
 *
 * The grammar records the modifier.
 *
 * The semantic/type/effect system enforces it.
 */
linearQubitDeclaration
    : K_LINEAR
      K_QUBIT
      identifier
      qubitTypeAnnotation?
      qubitInitializer?
      SEMICOLON
    ;


/* ============================================================================
 * 22. AFFINE QUBIT INTENT
 * ========================================================================== */

affineQubitDeclaration
    : K_AFFINE
      K_QUBIT
      identifier
      qubitTypeAnnotation?
      qubitInitializer?
      SEMICOLON
    ;


/* ============================================================================
 * 23. QUBIT RESOURCE BINDING
 * ========================================================================== */

/*
 * This is intentionally symbolic.
 *
 * It allows the semantic layer to represent a qubit as a resource-bearing
 * value without putting allocation policy into the parser.
 */
qubitResourceBinding
    : identifier
      THIN_ARROW
      qubitReference
    ;


/* ============================================================================
 * 24. QUBIT RESOURCE LIST
 * ========================================================================== */

qubitResourceList
    : qubitResourceBinding
      (COMMA qubitResourceBinding)*
    ;


/* ============================================================================
 * 25. QUBIT DECLARATION UNION
 * ========================================================================== */

/*
 * Canonical integration point for quantum declaration grammar.
 *
 * The canonical quantum parser can reference this single rule instead of
 * independently reproducing every qubit declaration production.
 */
quantumQubitDeclaration
    : qubitDeclaration
    | qubitRegisterDeclaration
    | qubitCollectionDeclaration
    | logicalQubitDeclaration
    | qubitAliasDeclaration
    | qubitViewDeclaration
    | linearQubitDeclaration
    | affineQubitDeclaration
    ;


/* ============================================================================
 * 26. QUBIT REFERENCE UNION
 * ========================================================================== */

quantumQubitReference
    : qubitReference
    | qubitIndex
    | qubitRange
    | qubitSelection
    | qubitGroup
    ;


/* ============================================================================
 * 27. QUBIT COUNT / EXTENT INTENT
 * ========================================================================== */

/*
 * This is an expression rather than a literal integer.
 *
 * Examples:
 *
 *     q[n]
 *
 *     q[problem_size]
 *
 *     q[2 * n]
 *
 *     q[resource.qubits]
 *
 * Resource validation happens later.
 */
qubitCountExpression
    : expression
    ;


/* ============================================================================
 * 28. QUBIT CAPACITY QUERY
 * ========================================================================== */

/*
 * A program may query semantic/resource information through normal language
 * expressions.
 *
 * The grammar does not define a fixed capacity property.
 */
qubitCapacityExpression
    : identifier
      DOT
      identifier
    ;


/* ============================================================================
 * 29. QUBIT PROPERTY ACCESS
 * ========================================================================== */

qubitPropertyAccess
    : quantumQubitReference
      DOT
      identifier
    ;


/* ============================================================================
 * 30. QUBIT RESOURCE EXPRESSION
 * ========================================================================== */

/*
 * Resource expressions are symbolic and target-independent.
 */
qubitResourceExpression
    : qubitCountExpression
    | qubitCapacityExpression
    | qubitPropertyAccess
    | expression
    ;


/* ============================================================================
 * 31. QUBIT CONSTRAINT EXPRESSION
 * ========================================================================== */

/*
 * Constraints do not become hardware allocations.
 *
 * Example:
 *
 *     requirement(qubits >= required);
 *
 * The actual constraint language is owned by resources/constraints.g4.
 *
 * This rule only provides a qubit expression integration point.
 */
qubitConstraintExpression
    : expression
    ;


/* ============================================================================
 * 32. QUBIT CAPABILITY EXPRESSION
 * ========================================================================== */

/*
 * Capability information is discovered from the target/environment.
 *
 * It is not hard-coded here.
 */
qubitCapabilityExpression
    : expression
    ;


/* ============================================================================
 * 33. QUBIT ATTRIBUTE
 * ========================================================================== */

/*
 * Source-level metadata can annotate qubit declarations.
 *
 * Example:
 *
 *     @logical
 *     qubit q;
 *
 * The generic attribute grammar owns the actual attribute syntax.
 *
 * This rule is provided as an integration point only.
 */
qubitAttributedDeclaration
    : attribute*
      quantumQubitDeclaration
    ;


/* ============================================================================
 * 34. QUBIT STATE REFERENCE
 * ========================================================================== */

/*
 * A state reference is an ordinary identifier/expression.
 *
 * State representation belongs to the quantum type/semantic layer.
 */
qubitStateReference
    : identifier
    | qualifiedName
    ;


/* ============================================================================
 * 35. QUBIT INITIALIZATION FROM STATE
 * ========================================================================== */

qubitStateInitializer
    : ASSIGN
      qubitStateReference
    ;


/* ============================================================================
 * 36. QUBIT DESTRUCTION / RELEASE INTENT
 * ========================================================================== */

/*
 * Release syntax belongs to the broader quantum/resource grammar.
 *
 * This fragment exposes only the target.
 *
 * The actual release statement should be owned by:
 *
 *     grammar/quantum/operations.g4
 *
 * or:
 *
 *     grammar/resources/
 *
 * depending on the final language authority decision.
 */
qubitReleaseTarget
    : quantumQubitReference
    ;


/* ============================================================================
 * 37. QUBIT BORROW INTENT
 * ========================================================================== */

/*
 * Borrowing/ownership semantics are ultimately owned by the memory/effect/type
 * system.
 *
 * This production permits a quantum-specific source annotation without
 * implementing ownership semantics here.
 */
qubitBorrowExpression
    : identifier
      LPAREN
      quantumQubitReference
      RPAREN
    ;


/* ============================================================================
 * 38. QUBIT OWNERSHIP MODIFIER
 * ========================================================================== */

qubitOwnershipModifier
    : K_LINEAR
    | K_AFFINE
    ;


/* ============================================================================
 * 39. QUBIT GENERIC PARAMETER
 * ========================================================================== */

/*
 * Quantum algorithms often parameterize over a qubit container/type.
 *
 * Example:
 *
 *     circuit Algorithm<Q>(...) { ... }
 *
 * Generic declaration ownership belongs to the generic/type grammar.
 *
 * This rule exists only for quantum-specific consumption.
 */
quantumQubitGenericArgument
    : quantumQubitType
    | expression
    ;


/* ============================================================================
 * 40. QUBIT RANGE EXPRESSION
 * ========================================================================== */

/*
 * Range syntax remains compatible with the general expression range model.
 */
quantumQubitRangeExpression
    : expression
      qubitRangeOperator
      expression
    ;


/* ============================================================================
 * 41. QUBIT SLICE
 * ========================================================================== */

qubitSlice
    : quantumQubitReference
      LBRACKET
      quantumQubitRangeExpression
      RBRACKET
    ;


/* ============================================================================
 * 42. QUBIT COLLECTION EXPRESSION
 * ========================================================================== */

qubitCollectionExpression
    : quantumQubitReference
    | qubitSlice
    | qubitGroup
    | qubitSelection
    ;


/* ============================================================================
 * 43. QUBIT TARGET EXPRESSION
 * ========================================================================== */

qubitTargetExpression
    : qubitCollectionExpression
    | expression
    ;


/* ============================================================================
 * 44. QUBIT TARGET LIST
 * ========================================================================== */

qubitTargetList
    : qubitTargetExpression
      (COMMA qubitTargetExpression)*
    ;


/* ============================================================================
 * 45. QUBIT DECLARATION BLOCK
 * ========================================================================== */

/*
 * This is deliberately generic.
 *
 * The parent quantum grammar decides whether a block is a circuit, operation,
 * module, or other quantum region.
 */
qubitDeclarationBlock
    : LBRACE
      quantumQubitDeclaration*
      RBRACE
    ;


/* ============================================================================
 * 46. QUBIT TYPE REFERENCE
 * ========================================================================== */

qubitTypeReference
    : quantumQubitType
    ;


/* ============================================================================
 * 47. QUANTUM RESOURCE TYPE
 * ========================================================================== */

/*
 * Resource types may be introduced by future semantic type definitions.
 *
 * They remain identifier-based here.
 */
quantumQubitResourceType
    : qualifiedName
    ;


/* ============================================================================
 * 48. QUANTUM QUBIT DECLARATION WITH RESOURCE TYPE
 * ========================================================================== */

quantumQubitResourceDeclaration
    : K_QUBIT
      identifier
      COLON
      quantumQubitResourceType
      SEMICOLON
    ;


/* ============================================================================
 * 49. QUBIT PATTERN
 * ========================================================================== */

/*
 * Pattern matching is owned by the general pattern grammar.
 *
 * Quantum-specific patterns may use these semantic references.
 */
qubitPattern
    : qubitReference
    | qubitIndex
    | qubitRange
    | qubitGroup
    ;


/* ============================================================================
 * 50. QUBIT PATTERN LIST
 * ========================================================================== */

qubitPatternList
    : qubitPattern
      (COMMA qubitPattern)*
    ;


/* ============================================================================
 * 51. QUBIT SOURCE-LEVEL IDENTIFICATION
 * ========================================================================== */

/*
 * IMPORTANT:
 *
 * This is a SOURCE identifier, not a physical hardware identifier.
 *
 * For example:
 *
 *     q
 *
 * does not mean physical qubit #q.
 *
 * The semantic layer maps source identity into canonical IR identity.
 */
qubitSourceIdentifier
    : identifier
    ;


/* ============================================================================
 * 52. QUBIT RESOURCE IDENTITY
 * ========================================================================== */

/*
 * Canonical resource identity is deliberately NOT represented as a physical
 * integer here.
 *
 * The frontend may preserve the source identity and semantic resolver may
 * assign a canonical IR resource identity later.
 */
qubitResourceIdentity
    : qubitSourceIdentifier
    ;


/* ============================================================================
 * 53. QUBIT REFERENCE PATH
 * ========================================================================== */

qubitReferencePath
    : qualifiedName
    ;


/* ============================================================================
 * 54. QUBIT MEMBER REFERENCE
 * ========================================================================== */

qubitMemberReference
    : quantumQubitReference
      DOT
      identifier
    ;


/* ============================================================================
 * 55. QUBIT EXPRESSION
 * ========================================================================== */

/*
 * Canonical integration point for expressions involving qubits.
 */
quantumQubitExpression
    : quantumQubitReference
    | qubitMemberReference
    | qubitCollectionExpression
    | qubitResourceExpression
    | expression
    ;


/* ============================================================================
 * 56. QUBIT DECLARATION ENTRY POINT
 * ========================================================================== */

/*
 * The canonical quantum grammar should reference this rule.
 */
quantumQubitDeclarationEntry
    : quantumQubitDeclaration
    ;


/* ============================================================================
 * 57. QUBIT REFERENCE ENTRY POINT
 * ========================================================================== */

quantumQubitReferenceEntry
    : quantumQubitReference
    ;


/* ============================================================================
 * 58. QUBIT TARGET ENTRY POINT
 * ========================================================================== */

quantumQubitTargetEntry
    : quantumQubitTarget
    ;


/* ============================================================================
 * 59. QUBIT SCALABILITY CONTRACT
 * ========================================================================== */

/*
 * The following are intentionally absent:
 *
 *     MAX_QUBITS
 *     MAX_REGISTER_SIZE
 *     MAX_QUBIT_INDEX
 *     MAX_QUBIT_GROUP_SIZE
 *     MAX_QUBIT_RANGE
 *     MAX_CIRCUIT_QUBITS
 *     MAX_LOGICAL_QUBITS
 *     MAX_PHYSICAL_QUBITS
 *
 * A parser grammar must not impose such limits.
 *
 * Any implementation limit belongs to:
 *
 *     parser/runtime limits;
 *     resource limits;
 *     compilation limits;
 *     hardware capabilities;
 *     execution policy.
 *
 * Those limits must be represented outside this grammar.
 */


/* ============================================================================
 * 60. NO PHYSICAL HARD-CODING
 * ========================================================================== */

/*
 * This file MUST NOT contain:
 *
 *     q[0]
 *     q[1]
 *     q[2]
 *
 * as predefined semantic resources.
 *
 * q[0] is legal source syntax when a user writes it.
 *
 * But the grammar must never interpret it as a built-in physical resource.
 *
 * Likewise, the grammar must not contain:
 *
 *     physical_qubit_0
 *     physical_qubit_1
 *     device_q0
 *     ibm_q0
 *     topology_...
 *
 * Hardware identity belongs to hardware/target lowering.
 */


/* ============================================================================
 * 61. NO GATE OWNERSHIP
 * ========================================================================== */

/*
 * This file does NOT define:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     S
 *     T
 *     CNOT
 *     CZ
 *     SWAP
 *     U
 *     RXX
 *     RYY
 *     RZZ
 *
 * as qubit syntax.
 *
 * Operation identity belongs to the quantum operation grammar and semantic
 * operation registry.
 */


/* ============================================================================
 * 62. NO ROUTING OWNERSHIP
 * ========================================================================== */

/*
 * The distinction:
 *
 *     source qubit
 *
 * versus:
 *
 *     logical qubit
 *
 * versus:
 *
 *     physical qubit
 *
 * must remain explicit.
 *
 * This grammar can represent source intent.
 *
 * Routing owns physical realization.
 */


/* ============================================================================
 * 63. NO SCHEDULING OWNERSHIP
 * ========================================================================== */

/*
 * This file never represents:
 *
 *     duration;
 *     start time;
 *     cycle;
 *     hardware slot;
 *     pulse;
 *     alignment.
 *
 * Those belong to scheduling/timing/pulse layers.
 */


/* ============================================================================
 * 64. NO QEC OWNERSHIP
 * ========================================================================== */

/*
 * `logical` is source-level intent only.
 *
 * This file does not define:
 *
 *     surface code;
 *     repetition code;
 *     color code;
 *     stabilizer decoder;
 *     syndrome extraction;
 *     correction algorithm.
 *
 * QEC owns those semantics.
 */


/* ============================================================================
 * 65. NO ZQN OWNERSHIP
 * ========================================================================== */

/*
 * A qubit declaration does not contain a noise model.
 *
 * ZQN owns:
 *
 *     fault semantics;
 *     channels;
 *     correlated faults;
 *     leakage;
 *     loss;
 *     erasure;
 *     calibration-related noise semantics.
 */


/* ============================================================================
 * 66. AST CONTRACT
 * ========================================================================== */

/*
 * Every qubit declaration/reference parsed from this file must preserve at
 * least:
 *
 *     - source span;
 *     - source identifier;
 *     - declaration kind;
 *     - optional type annotation;
 *     - optional initializer;
 *     - optional extent expression;
 *     - optional index expression;
 *     - optional range expressions;
 *     - ownership modifiers;
 *     - attributes;
 *     - generic information where applicable.
 *
 * The AST must preserve symbolic expressions rather than prematurely
 * evaluating resource quantities.
 */


/* ============================================================================
 * 67. SEMANTIC CONTRACT
 * ========================================================================== */

/*
 * Semantic analysis must determine:
 *
 *     - whether a qubit name is declared;
 *     - whether a reference is valid;
 *     - whether an index is valid;
 *     - whether an extent is valid;
 *     - whether a range is valid;
 *     - whether a value is a qubit;
 *     - whether a collection contains qubits;
 *     - whether ownership rules are satisfied;
 *     - whether aliasing is legal;
 *     - whether initialization is legal;
 *     - whether logical/physical intent is valid;
 *     - whether resource requirements can be satisfied.
 *
 * None of those validations belong in this parser fragment.
 */


/* ============================================================================
 * 68. QUANTUM IR CONTRACT
 * ========================================================================== */

/*
 * The frontend lowers parsed qubit constructs into the canonical quantum IR.
 *
 * This file MUST NOT define a second QubitId type.
 *
 * This file MUST NOT define:
 *
 *     struct QubitId
 *     struct PhysicalQubitId
 *     struct QuantumRegister
 *     struct QuantumCircuit
 *
 * in Rust or ANTLR.
 *
 * The canonical IR owns semantic identities.
 *
 * The parser preserves source-level structure until lowering.
 */


/* ============================================================================
 * 69. RESOURCE SYSTEM CONTRACT
 * ========================================================================== */

/*
 * For:
 *
 *     qubit q[n];
 *
 * the parser produces the expression `n`.
 *
 * Resource analysis later determines:
 *
 *     required resource;
 *     minimum resource;
 *     available resource;
 *     target capability;
 *     admission decision.
 *
 * The grammar does not evaluate or cap n.
 */


/* ============================================================================
 * 70. HARDWARE CONTRACT
 * ========================================================================== */

/*
 * Hardware may provide:
 *
 *     available qubits;
 *     logical qubit support;
 *     connectivity;
 *     measurement capabilities;
 *     reset capabilities;
 *     coherence properties;
 *     native operations.
 *
 * Those properties are discovered after parsing.
 *
 * No hardware provider is a dependency of this grammar.
 */


/* ============================================================================
 * 71. COMPILER CONTRACT
 * ========================================================================== */

/*
 * Compilation may transform:
 *
 *     abstract source qubits
 *
 * into:
 *
 *     logical resources
 *
 * and eventually:
 *
 *     physical resources.
 *
 * This file must remain unchanged when a new hardware target is introduced,
 * provided the source language semantics remain unchanged.
 */


/* ============================================================================
 * 72. RUNTIME CONTRACT
 * ========================================================================== */

/*
 * Runtime may:
 *
 *     allocate;
 *     lease;
 *     release;
 *     migrate;
 *     remap;
 *     recover;
 *     retry;
 *     route;
 *
 * qubit resources.
 *
 * Runtime behavior is outside grammar ownership.
 */


/* ============================================================================
 * 73. CROSS-DOMAIN CONTRACT
 * ========================================================================== */

/*
 * Qubits may participate in:
 *
 *     classical control;
 *     distributed execution;
 *     hardware descriptions;
 *     accelerator workflows;
 *     AI/ML hybrid computation;
 *     networking;
 *     resilience.
 *
 * This file exposes qubit references to those domains without importing their
 * implementation semantics.
 */


/* ============================================================================
 * 74. DETERMINISM CONTRACT
 * ========================================================================== */

/*
 * Parsing the same source with the same grammar/version must produce the same
 * parse tree.
 *
 * This file therefore contains no:
 *
 *     - backend queries;
 *     - runtime decisions;
 *     - hardware discovery;
 *     - semantic callbacks;
 *     - external state;
 *     - filesystem access;
 *     - network access.
 */


/* ============================================================================
 * 75. SECURITY CONTRACT
 * ========================================================================== */

/*
 * Parsing qubit syntax must never:
 *
 *     - allocate hardware;
 *     - connect to a QPU;
 *     - open files;
 *     - access a network;
 *     - execute user code;
 *     - execute quantum operations.
 *
 * The grammar is purely syntactic.
 */


/* ============================================================================
 * 76. TEST CONTRACT
 * ========================================================================== */

/*
 * Positive tests required:
 *
 *     qubit q;
 *     qubit q = |0⟩;
 *     qubit q = |1⟩;
 *     qubit q = |+⟩;
 *     qubit q = |-⟩;
 *     qubit q[n];
 *     qubit q[expression];
 *     qubit q[start .. end];
 *     qubit q[start ..= end];
 *     logical qubit q;
 *     linear qubit q;
 *     affine qubit q;
 *     qubit alias = q;
 *     qubit view = q[start .. end];
 *
 * Positive tests must also cover:
 *
 *     nested expressions;
 *     generic types;
 *     qualified names;
 *     Unicode source identifiers where supported by the canonical lexer;
 *     very large symbolic extents;
 *     very large reference lists.
 *
 * Negative tests required:
 *
 *     missing identifier;
 *     missing extent;
 *     missing closing bracket;
 *     malformed range;
 *     missing semicolon;
 *     malformed initializer;
 *     malformed alias;
 *     malformed group;
 *     malformed generic type.
 *
 * Boundary tests required:
 *
 *     one qubit;
 *     empty/zero extent where syntactically legal;
 *     extremely large symbolic extent;
 *     deeply nested valid expressions;
 *     very large qubit lists;
 *     very large range expressions.
 *
 * Semantic negative tests belong to semantic-analysis tests rather than parser
 * tests, for example:
 *
 *     - undeclared qubit;
 *     - invalid qubit index;
 *     - invalid extent type;
 *     - illegal alias;
 *     - illegal ownership duplication;
 *     - unavailable resource.
 */


/* ============================================================================
 * 77. SCALABILITY TEST CONTRACT
 * ========================================================================== */

/*
 * The parser test suite MUST demonstrate that no source-level fixed qubit
 * ceiling exists.
 *
 * Examples should be generated parametrically rather than storing only:
 *
 *     q[0]
 *     q[1]
 *     ...
 *     q[32]
 *
 * Tests should cover symbolic sizes such as:
 *
 *     q[n]
 *     q[problem_size]
 *     q[resource.qubits]
 *
 * The test itself must not become the language limit.
 */


/* ============================================================================
 * 78. COMPATIBILITY CONTRACT
 * ========================================================================== */

/*
 * Existing valid Zamani quantum syntax must not be silently removed.
 *
 * Before changing a production:
 *
 *     1. identify existing consumers;
 *     2. identify documented syntax;
 *     3. preserve compatible syntax;
 *     4. provide migration if syntax moves;
 *     5. update grammar specification;
 *     6. update parser fixtures;
 *     7. update AST lowering;
 *     8. update compatibility tests.
 *
 * This file therefore becomes part of the versioned language contract.
 */


/* ============================================================================
 * 79. GRAMMAR AUTHORITY
 * ========================================================================== */

/*
 * This file is authoritative ONLY for qubit-specific parser structure.
 *
 * It is subordinate to:
 *
 *     grammar/lexer/tokens.g4
 *
 * for tokens.
 *
 * It is consumed by the canonical parser.
 *
 * It is NOT an alternative complete Zamani grammar.
 *
 * It MUST NOT be independently treated as the source root.
 */


/* ============================================================================
 * 80. INTEGRATION WITH grammar/quantum/quantum.g4
 * ========================================================================== */

/*
 * The parent quantum grammar should integrate this fragment through:
 *
 *     quantumQubitDeclaration
 *
 *     quantumQubitReference
 *
 *     quantumQubitTarget
 *
 * rather than copying these productions.
 *
 * Conceptually:
 *
 *     quantum.g4
 *          |
 *          +--> qubits.g4
 *          |
 *          +--> operations.g4
 *          |
 *          +--> measurement.g4
 *          |
 *          +--> circuits.g4
 *          |
 *          +--> quantum-types.g4
 *          |
 *          +--> quantum-resources.g4
 *
 * This keeps qubit ownership centralized.
 */


/* ============================================================================
 * 81. INTEGRATION WITH GENERAL EXPRESSIONS
 * ========================================================================== */

/*
 * This file intentionally consumes:
 *
 *     expression
 *
 * rather than defining:
 *
 *     qubitExpression
 *
 * as a second general expression language.
 *
 * Therefore:
 *
 *     q[index]
 *
 *     q[start .. end]
 *
 *     q[computed_index]
 *
 * can reuse the canonical expression/type infrastructure.
 */


/* ============================================================================
 * 82. INTEGRATION WITH TYPES
 * ========================================================================== */

/*
 * The type system must eventually distinguish concepts such as:
 *
 *     Qubit
 *     QubitRegister
 *     QubitView
 *     LogicalQubit
 *     PhysicalQubit
 *     EncodedQubit
 *     QubitReference
 *     QubitCollection
 *
 * without requiring this parser file to duplicate their complete semantic
 * definitions.
 */


/* ============================================================================
 * 83. INTEGRATION WITH MEMORY / OWNERSHIP
 * ========================================================================== */

/*
 * Quantum resources can have ownership/lifetime constraints.
 *
 * `linear` and `affine` are therefore syntax-level modifiers only.
 *
 * The memory/effect/type system determines:
 *
 *     move;
 *     borrow;
 *     alias;
 *     release;
 *     lifetime;
 *     ownership;
 *     uniqueness.
 */


/* ============================================================================
 * 84. INTEGRATION WITH QEC
 * ========================================================================== */

/*
 * Logical qubit syntax is intentionally independent of any particular QEC
 * implementation.
 *
 * Example:
 *
 *     logical qubit q;
 *
 * does NOT select:
 *
 *     surface code;
 *     repetition code;
 *     color code;
 *     subsystem code;
 *     a particular decoder.
 *
 * QEC chooses/validates those downstream.
 */


/* ============================================================================
 * 85. INTEGRATION WITH ZQN
 * ========================================================================== */

/*
 * Qubit declarations have no intrinsic noise model.
 *
 * ZQN may associate observed or modeled faults with canonical quantum IR
 * resources after lowering.
 */


/* ============================================================================
 * 86. INTEGRATION WITH RESILIENCE
 * ========================================================================== */

/*
 * Resilience may later decide to:
 *
 *     remap;
 *     reroute;
 *     reschedule;
 *     recompile;
 *     change QEC;
 *     retry;
 *     recover;
 *     switch backend;
 *
 * without requiring the source qubit declarations to change.
 *
 * This is a core POCO-REAF property.
 */


/* ============================================================================
 * 87. INTEGRATION WITH SCHEDULING
 * ========================================================================== */

/*
 * A qubit reference does not contain:
 *
 *     time;
 *     duration;
 *     cycle;
 *     schedule slot;
 *     pulse.
 *
 * Scheduling consumes canonical quantum operations/resources later.
 */


/* ============================================================================
 * 88. INTEGRATION WITH ROUTING
 * ========================================================================== */

/*
 * A source reference:
 *
 *     q
 *
 * is not a physical location.
 *
 * Routing maps canonical logical resources to target resources after semantic
 * analysis.
 */


/* ============================================================================
 * 89. INTEGRATION WITH OPTIMIZATION
 * ========================================================================== */

/*
 * Optimization may transform operations involving qubits while preserving
 * semantic identity and source provenance.
 *
 * It does not modify this grammar.
 */


/* ============================================================================
 * 90. INTEGRATION WITH HDL / HARDWARE
 * ========================================================================== */

/*
 * HDL/hardware grammar may reference quantum resources for hybrid
 * hardware/software co-design.
 *
 * Such integration should occur through shared semantic/resource/type
 * contracts rather than importing hardware implementation rules into this
 * file.
 */


/* ============================================================================
 * 91. INTEGRATION WITH DISTRIBUTED COMPUTING
 * ========================================================================== */

/*
 * Distributed quantum resources may exist across execution domains.
 *
 * This grammar does not encode:
 *
 *     node IDs;
 *     network topology;
 *     communication latency;
 *     distributed hardware count.
 *
 * Those are resource/deployment/runtime concerns.
 */


/* ============================================================================
 * 92. INTEGRATION WITH AI / DATA
 * ========================================================================== */

/*
 * Quantum registers may be used as inputs/outputs of hybrid algorithms.
 *
 * This file remains independent of AI/data grammar.
 *
 * Shared integration occurs through:
 *
 *     expression;
 *     type;
 *     resource;
 *     effect;
 *     canonical IR.
 */


/* ============================================================================
 * 93. INTEGRATION WITH INTEROPERABILITY
 * ========================================================================== */

/*
 * External quantum languages such as OpenQASM may lower into the canonical
 * frontend/IR representation.
 *
 * They must not create a second qubit identity model.
 *
 * Source adapters must map into Zamani's canonical semantic boundary.
 */


/* ============================================================================
 * 94. INTEGRATION WITH SERIALIZATION
 * ========================================================================== */

/*
 * Serialization must preserve:
 *
 *     source identity;
 *     declaration kind;
 *     symbolic extent;
 *     index/range expressions;
 *     type information;
 *     attributes;
 *     provenance.
 *
 * Serialization must not serialize a physical resource merely because the
 * source declaration exists.
 */


/* ============================================================================
 * 95. INTEGRATION WITH DIAGNOSTICS
 * ========================================================================== */

/*
 * Diagnostics must point to exact source spans for:
 *
 *     undeclared identifiers;
 *     malformed declarations;
 *     malformed indexing;
 *     malformed ranges;
 *     invalid declaration forms;
 *     incompatible initialization.
 *
 * Parser errors remain syntax errors.
 *
 * Semantic errors must remain distinguishable from parser errors.
 */


/* ============================================================================
 * 96. NO RUNTIME ACTIONS
 * ========================================================================== */

/*
 * Nothing in this grammar causes:
 *
 *     allocation;
 *     initialization;
 *     measurement;
 *     reset;
 *     routing;
 *     scheduling;
 *     execution.
 *
 * It only parses source syntax.
 */


/* ============================================================================
 * 97. COMPLETION CRITERIA
 * ========================================================================== */

/*
 * This file is COMPLETE only when ALL of the following are true:
 *
 * [ ] canonical lexer vocabulary is used;
 * [ ] no lexer token is redeclared;
 * [ ] no fixed qubit limit exists;
 * [ ] no fixed register limit exists;
 * [ ] no fixed index limit exists;
 * [ ] no physical topology exists;
 * [ ] no backend identifier exists;
 * [ ] no gate catalogue exists;
 * [ ] no QEC algorithm exists;
 * [ ] no ZQN model exists;
 * [ ] no routing implementation exists;
 * [ ] no scheduling implementation exists;
 * [ ] no runtime action exists;
 * [ ] general expressions are reused;
 * [ ] general type infrastructure is reused;
 * [ ] AST mapping is defined;
 * [ ] canonical quantum IR mapping is defined;
 * [ ] logical/physical distinction is preserved;
 * [ ] ownership semantics are delegated to the appropriate semantic layer;
 * [ ] resource constraints are delegated to resource analysis;
 * [ ] positive tests exist;
 * [ ] negative tests exist;
 * [ ] boundary tests exist;
 * [ ] scalability tests exist;
 * [ ] deterministic parsing tests exist;
 * [ ] compatibility tests exist;
 * [ ] cross-domain tests exist;
 * [ ] the parent quantum grammar imports this fragment exactly once;
 * [ ] no duplicate qubit grammar remains authoritative elsewhere.
 */


/* ============================================================================
 * 98. FINAL INVARIANT
 * ========================================================================== */

/*
 * The fundamental invariant of this file is:
 *
 *
 *     SOURCE QUBIT
 *          |
 *          v
 *     SEMANTIC RESOURCE
 *          |
 *          v
 *     CANONICAL QUANTUM IR
 *          |
 *          v
 *     TARGET REALIZATION
 *
 *
 * NOT:
 *
 *     SOURCE QUBIT
 *          |
 *          v
 *     PHYSICAL QUBIT
 *
 *
 * The latter would violate POCO-REAF.
 *
 * Zamani source describes computation.
 *
 * Resource availability determines realization.
 *
 * Therefore the same source-level qubit semantics can scale across:
 *
 *     one qubit;
 *     many qubits;
 *     logical qubits;
 *     large quantum systems;
 *     heterogeneous systems;
 *     simulators;
 *     future quantum architectures;
 *
 * subject only to the capabilities and resources available to the compilation
 * and execution environment.
 *
 * ============================================================================
 */