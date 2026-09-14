/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/quantum/quantum-registers.g4
 *
 * Purpose:
 *     Canonical reusable parser fragment for quantum-register syntax.
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser-fragment source consumed by the canonical Zamani parser.
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * This file owns the SOURCE SYNTAX of quantum registers.
 *
 * It describes:
 *
 *     - register declarations;
 *     - register bindings;
 *     - register type annotations;
 *     - register extents;
 *     - symbolic extents;
 *     - runtime-dependent extents;
 *     - register initialization;
 *     - register references;
 *     - register indexing;
 *     - register slicing;
 *     - register selection;
 *     - register grouping;
 *     - register aliases;
 *     - register views;
 *     - logical-register intent;
 *     - register resource annotations;
 *     - register metadata syntax;
 *     - register-qualified qubit references.
 *
 * It does NOT determine:
 *
 *     - physical qubit allocation;
 *     - physical topology;
 *     - device selection;
 *     - QPU selection;
 *     - hardware capacity;
 *     - routing;
 *     - scheduling;
 *     - gate decomposition;
 *     - calibration;
 *     - QEC implementation;
 *     - ZQN noise models;
 *     - simulator representation;
 *     - runtime allocation;
 *     - canonical quantum IR representation.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - register source syntax;
 *     - register declaration syntax;
 *     - register reference syntax;
 *     - register extent syntax;
 *     - register selection syntax;
 *     - register slicing syntax;
 *     - register alias/view syntax;
 *     - logical register syntax;
 *     - register initialization syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer definitions;
 *     - token definitions;
 *     - identifiers;
 *     - general expressions;
 *     - general types;
 *     - general declarations;
 *     - individual qubit semantics;
 *     - gates;
 *     - operations;
 *     - circuits;
 *     - measurement;
 *     - reset;
 *     - observables;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - hardware discovery;
 *     - resource accounting;
 *     - runtime allocation;
 *     - canonical IR.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A register declaration expresses a PROGRAM-LEVEL COLLECTION OF QUANTUM
 * RESOURCES.
 *
 * For example:
 *
 *     qubit q[n];
 *
 * expresses that the program requires a quantum register whose extent is
 * described by the expression `n`.
 *
 * It does NOT mean:
 *
 *     allocate physical qubit 0..n-1
 *
 * and does NOT select:
 *
 *     a QPU;
 *     a topology;
 *     a device;
 *     a vendor;
 *     a simulator;
 *     a physical address.
 *
 * Therefore:
 *
 *     source register
 *         ->
 *     semantic register
 *         ->
 *     resource/capability analysis
 *         ->
 *     target realization
 *
 * rather than:
 *
 *     source register
 *         ->
 *     fixed hardware allocation.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There is intentionally NO:
 *
 *     MAX_QUBITS
 *     MAX_REGISTER_WIDTH
 *     MAX_REGISTERS
 *     MAX_INDEX
 *     MAX_RANGE
 *     MAX_DIMENSION
 *     MAX_DEVICE_COUNT
 *     MAX_QPU_SIZE
 *     MAX_TOPOLOGY_SIZE
 *
 * in this grammar.
 *
 * Register extents are expressions.
 *
 * Consequently all of the following are syntactically possible, subject to
 * the general expression grammar:
 *
 *     qubit q[n];
 *     qubit q[width];
 *     qubit q[2 * n];
 *     qubit q[resource_size];
 *     qubit q[configuration.register_width];
 *
 * Whether an extent is:
 *
 *     valid;
 *     finite;
 *     representable;
 *     available;
 *     affordable;
 *     supported;
 *     schedulable;
 *
 * is NOT a grammar concern.
 *
 * Those questions belong to:
 *
 *     semantic analysis;
 *     type checking;
 *     resource analysis;
 *     capability checking;
 *     compilation;
 *     scheduling;
 *     hardware abstraction;
 *     runtime.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     canonical lexer
 *          |
 *          v
 *     core names / qualified names
 *          |
 *          v
 *     types
 *          |
 *          v
 *     expressions / ranges / indexing
 *          |
 *          v
 *     quantum/qubits.g4
 *          |
 *          v
 *     quantum/quantum-registers.g4
 *          |
 *          v
 *     quantum.g4 / canonical parser
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
 *          +---- optimization
 *          +---- routing
 *          +---- scheduling
 *          +---- QEC
 *          +---- ZQN
 *          +---- resilience
 *          +---- hardware HAL
 *          |
 *          v
 *     target lowering / runtime
 *
 * Dependency direction MUST NOT be reversed.
 *
 * ============================================================================
 * ANTLR DESIGN
 * ============================================================================
 *
 * This file contains parser rules only.
 *
 * It MUST NOT:
 *
 *     - declare a lexer grammar;
 *     - define lexer tokens;
 *     - contain target-language actions;
 *     - execute Rust;
 *     - perform filesystem access;
 *     - perform network access;
 *     - allocate hardware;
 *     - construct quantum IR directly.
 *
 * The generated parser is therefore target-neutral at the grammar level.
 *
 * Rust 1.97 / 1.97.1 compatibility is enforced by the generated-parser
 * integration/build configuration, not by embedding Rust code in this file.
 *
 * ============================================================================
 */


/* ============================================================================
 * 1. CANONICAL REGISTER DECLARATION
 * ========================================================================== */

/*
 * Canonical abstract quantum-register declaration.
 *
 * Examples:
 *
 *     qubit q[n];
 *
 *     qubit q[width];
 *
 *     qubit q[n] = initializer;
 *
 *     qubit q[n] : QuantumRegister;
 *
 *     qubit q[n] : QuantumRegister = initializer;
 *
 * The grammar accepts the extent as a general expression.
 *
 * No integer-width or machine-size assumption is encoded here.
 */
quantumRegisterDeclaration
    : K_QUBIT
      identifier
      LBRACKET
      quantumRegisterExtent
      RBRACKET
      quantumRegisterTypeAnnotation?
      quantumRegisterInitializer?
      SEMICOLON
    ;


/* ============================================================================
 * 2. REGISTER EXTENT
 * ========================================================================== */

/*
 * A register extent is a source-level expression.
 *
 * This intentionally permits:
 *
 *     constants;
 *     parameters;
 *     compile-time expressions;
 *     runtime values;
 *     configuration-derived values;
 *     resource-derived values;
 *     generic expressions.
 *
 * Semantic analysis determines whether the resulting extent is legal for
 * the particular execution context.
 */
quantumRegisterExtent
    : expression
    ;


/* ============================================================================
 * 3. REGISTER TYPE ANNOTATION
 * ========================================================================== */

/*
 * Optional semantic type annotation.
 *
 * Examples:
 *
 *     qubit q[n] : QuantumRegister;
 *
 *     qubit q[n] : logical_register;
 *
 *     qubit q[n] : quantum::Register;
 *
 * Type identity is resolved by the canonical type system.
 */
quantumRegisterTypeAnnotation
    : COLON
      quantumRegisterType
    ;


quantumRegisterType
    : quantumRegisterPrimitiveType
    | qualifiedName
    | genericTypeExpression
    ;


quantumRegisterPrimitiveType
    : K_QUBIT
    ;


/* ============================================================================
 * 4. REGISTER INITIALIZATION
 * ========================================================================== */

/*
 * Initialization is intentionally expression-based.
 *
 * This allows future state/initializer abstractions without changing the
 * register grammar every time a new semantic initializer is introduced.
 *
 * The semantic layer must validate:
 *
 *     initializer type;
 *     initializer extent;
 *     ownership;
 *     linearity;
 *     aliasing;
 *     state compatibility.
 */
quantumRegisterInitializer
    : ASSIGN
      quantumRegisterInitializerExpression
    ;


quantumRegisterInitializerExpression
    : expression
    ;


/* ============================================================================
 * 5. REGISTER BINDING
 * ========================================================================== */

/*
 * A register binding is an ordinary source-level identifier.
 *
 * It is deliberately NOT:
 *
 *     QPU ID;
 *     hardware ID;
 *     physical register address;
 *     backend identifier.
 */
quantumRegisterBinding
    : identifier
    ;


/* ============================================================================
 * 6. REGISTER REFERENCE
 * ========================================================================== */

/*
 * A register reference is a semantic reference to a register binding.
 *
 * Qualified references are supported so that registers can be addressed
 * through module/namespace paths without introducing a second name system.
 */
quantumRegisterReference
    : identifier
    | qualifiedName
    ;


/* ============================================================================
 * 7. REGISTER INDEX
 * ========================================================================== */

/*
 * Single-element selection.
 *
 * Examples:
 *
 *     q[i]
 *     q[index]
 *     module.q[index]
 *
 * The index is an expression.
 *
 * The grammar imposes no fixed integer range.
 */
quantumRegisterIndex
    : quantumRegisterReference
      LBRACKET
      quantumRegisterIndexExpression
      RBRACKET
    ;


quantumRegisterIndexExpression
    : expression
    ;


/* ============================================================================
 * 8. REGISTER SLICE
 * ========================================================================== */

/*
 * Register slicing creates a source-level view/range.
 *
 * Examples:
 *
 *     q[start .. end]
 *
 *     q[start ..= end]
 *
 * The endpoints remain expressions.
 *
 * Actual:
 *
 *     bounds;
 *     direction;
 *     emptiness;
 *     overflow;
 *     lifetime;
 *     aliasing;
 *     contiguity;
 *     physical mapping;
 *
 * are semantic concerns.
 */
quantumRegisterSlice
    : quantumRegisterReference
      LBRACKET
      quantumRegisterSliceStart?
      quantumRegisterRangeOperator
      quantumRegisterSliceEnd?
      RBRACKET
    ;


quantumRegisterSliceStart
    : expression
    ;


quantumRegisterSliceEnd
    : expression
    ;


quantumRegisterRangeOperator
    : DOT_DOT
    | DOT_DOT_EQ
    ;


/* ============================================================================
 * 9. REGISTER ELEMENT SELECTION
 * ========================================================================== */

/*
 * A register selection is a generalized source-level selection.
 *
 * It may be represented by:
 *
 *     index;
 *     slice;
 *     selection expression;
 *
 * without requiring the grammar to know how the backend realizes it.
 */
quantumRegisterSelection
    : quantumRegisterIndex
    | quantumRegisterSlice
    | quantumRegisterSelectionExpression
    ;


quantumRegisterSelectionExpression
    : quantumRegisterReference
      LBRACKET
      expression
      RBRACKET
    ;


/* ============================================================================
 * 10. REGISTER TARGET
 * ========================================================================== */

/*
 * This is the canonical integration point for quantum operations that need
 * a register-oriented target.
 *
 * It deliberately accepts:
 *
 *     whole register;
 *     individual element;
 *     slice;
 *     selection.
 *
 * The operation grammar decides what target cardinality is semantically legal.
 */
quantumRegisterTarget
    : quantumRegisterReference
    | quantumRegisterIndex
    | quantumRegisterSlice
    | quantumRegisterSelection
    ;


/* ============================================================================
 * 11. REGISTER TARGET LIST
 * ========================================================================== */

/*
 * The list is unbounded by grammar.
 *
 * Example:
 *
 *     q
 *
 *     q, r
 *
 *     q[i], r[j], s[a .. b]
 *
 * Any practical limit comes from parser/runtime/resource implementation,
 * never from a language-level fixed register count.
 */
quantumRegisterTargetList
    : quantumRegisterTarget
      (COMMA quantumRegisterTarget)*
    ;


/* ============================================================================
 * 12. REGISTER GROUP
 * ========================================================================== */

/*
 * Explicit source grouping.
 *
 * Example:
 *
 *     (q, r, s)
 *
 * This is semantic grouping, not physical adjacency.
 */
quantumRegisterGroup
    : LPAREN
      quantumRegisterTargetList
      RPAREN
    ;


/* ============================================================================
 * 13. REGISTER ALIAS DECLARATION
 * ========================================================================== */

/*
 * Creates a source-level alias/reference.
 *
 * Example:
 *
 *     qubit alias = q;
 *
 * This grammar records syntax only.
 *
 * The semantic layer determines whether the alias:
 *
 *     - borrows;
 *     - aliases;
 *     - moves;
 *     - copies;
 *     - creates a view;
 *     - violates linearity;
 *     - violates affine ownership.
 */
quantumRegisterAliasDeclaration
    : K_QUBIT
      identifier
      ASSIGN
      quantumRegisterReference
      SEMICOLON
    ;


/* ============================================================================
 * 14. REGISTER VIEW DECLARATION
 * ========================================================================== */

/*
 * A view is a derived source-level reference.
 *
 * Examples:
 *
 *     qubit view = q[start .. end];
 *
 *     qubit view = q[index];
 *
 * View ownership and lifetime belong to the semantic/type/resource system.
 */
quantumRegisterViewDeclaration
    : K_QUBIT
      identifier
      ASSIGN
      quantumRegisterViewSource
      SEMICOLON
    ;


quantumRegisterViewSource
    : quantumRegisterIndex
    | quantumRegisterSlice
    | quantumRegisterSelection
    ;


/* ============================================================================
 * 15. LOGICAL REGISTER DECLARATION
 * ========================================================================== */

/*
 * Logical registers express logical quantum computation.
 *
 * Example:
 *
 *     logical qubit q[n];
 *
 * The grammar does NOT select:
 *
 *     a surface code;
 *     a repetition code;
 *     a color code;
 *     a particular decoder;
 *     physical qubits;
 *     a QPU.
 *
 * Those decisions belong to QEC, resource analysis, hardware abstraction,
 * routing, and compilation.
 */
logicalQuantumRegisterDeclaration
    : K_LOGICAL
      K_QUBIT
      identifier
      LBRACKET
      quantumRegisterExtent
      RBRACKET
      quantumRegisterTypeAnnotation?
      quantumRegisterInitializer?
      SEMICOLON
    ;


/* ============================================================================
 * 16. LINEAR REGISTER DECLARATION
 * ========================================================================== */

/*
 * Linear ownership is a source-level semantic intent.
 *
 * The grammar records the modifier.
 *
 * The type/effect/ownership system enforces:
 *
 *     non-duplication;
 *     legal movement;
 *     legal consumption;
 *     legal lifetime.
 */
linearQuantumRegisterDeclaration
    : K_LINEAR
      K_QUBIT
      identifier
      LBRACKET
      quantumRegisterExtent
      RBRACKET
      quantumRegisterTypeAnnotation?
      quantumRegisterInitializer?
      SEMICOLON
    ;


/* ============================================================================
 * 17. AFFINE REGISTER DECLARATION
 * ========================================================================== */

/*
 * Affine ownership permits at-most-once consumption.
 *
 * Enforcement is semantic, not syntactic.
 */
affineQuantumRegisterDeclaration
    : K_AFFINE
      K_QUBIT
      identifier
      LBRACKET
      quantumRegisterExtent
      RBRACKET
      quantumRegisterTypeAnnotation?
      quantumRegisterInitializer?
      SEMICOLON
    ;


/* ============================================================================
 * 18. REGISTER DECLARATION FAMILY
 * ========================================================================== */

/*
 * This rule is the preferred integration point for declarations.
 *
 * It makes the register forms explicit without making the parent grammar
 * understand every individual production.
 */
quantumRegisterDeclarationFamily
    : quantumRegisterDeclaration
    | logicalQuantumRegisterDeclaration
    | linearQuantumRegisterDeclaration
    | affineQuantumRegisterDeclaration
    | quantumRegisterAliasDeclaration
    | quantumRegisterViewDeclaration
    ;


/* ============================================================================
 * 19. REGISTER-QUALIFIED QUBIT REFERENCE
 * ========================================================================== */

/*
 * A register can qualify a qubit element.
 *
 * Examples:
 *
 *     q[i]
 *
 *     q[start .. end]
 *
 *     namespace.q[i]
 *
 *     module::register[index]
 *
 * The exact semantic identity is resolved through canonical name resolution.
 */
quantumRegisterQualifiedQubitReference
    : quantumRegisterIndex
    | quantumRegisterSlice
    ;


/* ============================================================================
 * 20. REGISTER EXTENT PARAMETER
 * ========================================================================== */

/*
 * Extent parameters are ordinary expressions.
 *
 * This rule exists as a named integration boundary so generic/compile-time
 * machinery can recognize register extents without duplicating expression
 * grammar.
 */
quantumRegisterExtentParameter
    : expression
    ;


/* ============================================================================
 * 21. REGISTER CONSTRAINT EXPRESSION
 * ========================================================================== */

/*
 * Register-specific constraints are source-level constraints.
 *
 * Example:
 *
 *     n > 0
 *
 *     width == configuration.width
 *
 * This rule does NOT evaluate the constraint.
 *
 * Evaluation belongs to semantic/resource analysis.
 */
quantumRegisterConstraint
    : expression
    ;


/* ============================================================================
 * 22. REGISTER DECLARATION WITH CONSTRAINT
 * ========================================================================== */

/*
 * Optional constrained register form.
 *
 * Example:
 *
 *     qubit q[n] where n > 0;
 *
 * The exact policy for `where` remains shared with the canonical constraint
 * grammar. This rule exists only as a quantum-register integration boundary.
 */
quantumRegisterConstrainedDeclaration
    : K_QUBIT
      identifier
      LBRACKET
      quantumRegisterExtent
      RBRACKET
      quantumRegisterTypeAnnotation?
      K_WHERE
      quantumRegisterConstraint
      quantumRegisterInitializer?
      SEMICOLON
    ;


/* ============================================================================
 * 23. REGISTER COLLECTION EXPRESSION
 * ========================================================================== */

/*
 * A collection expression can be used where quantum APIs accept a collection
 * of registers.
 *
 * Examples:
 *
 *     (q, r)
 *
 *     q[start .. end]
 *
 *     q[i]
 *
 * Collection semantics remain downstream.
 */
quantumRegisterCollectionExpression
    : quantumRegisterGroup
    | quantumRegisterTarget
    ;


/* ============================================================================
 * 24. REGISTER RESOURCE REFERENCE
 * ========================================================================== */

/*
 * This production intentionally does not encode a physical resource ID.
 *
 * A resource reference may be interpreted by resource/capability analysis.
 *
 * It must remain symbolic at the grammar layer.
 */
quantumRegisterResourceReference
    : quantumRegisterReference
    ;


/* ============================================================================
 * 25. REGISTER CARDINALITY EXPRESSION
 * ========================================================================== */

/*
 * Cardinality is semantic.
 *
 * It may be:
 *
 *     statically known;
 *     symbolically known;
 *     runtime-dependent;
 *     configuration-dependent;
 *     resource-dependent.
 *
 * The grammar accepts all valid expressions.
 */
quantumRegisterCardinality
    : expression
    ;


/* ============================================================================
 * 26. REGISTER WHOLE-RESOURCE TARGET
 * ========================================================================== */

/*
 * Explicit named rule for downstream operation grammars.
 *
 * Example:
 *
 *     apply operation to q;
 */
quantumWholeRegisterTarget
    : quantumRegisterReference
    ;


/* ============================================================================
 * 27. REGISTER ELEMENT TARGET
 * ========================================================================== */

quantumRegisterElementTarget
    : quantumRegisterIndex
    ;


/* ============================================================================
 * 28. REGISTER RANGE TARGET
 * ========================================================================== */

quantumRegisterRangeTarget
    : quantumRegisterSlice
    ;


/* ============================================================================
 * 29. REGISTER TARGET UNION
 * ========================================================================== */

/*
 * Stable downstream integration point.
 */
quantumRegisterOperationTarget
    : quantumWholeRegisterTarget
    | quantumRegisterElementTarget
    | quantumRegisterRangeTarget
    | quantumRegisterGroup
    ;


/* ============================================================================
 * 30. REGISTER TARGET LIST FOR OPERATIONS
 * ========================================================================== */

quantumRegisterOperationTargetList
    : quantumRegisterOperationTarget
      (COMMA quantumRegisterOperationTarget)*
    ;


/* ============================================================================
 * 31. REGISTER SEMANTIC BOUNDARY
 * ========================================================================== */

/*
 * This production intentionally contains no semantic action.
 *
 * It exists as a named boundary for AST lowering.
 *
 * The AST builder should map the parse context to a semantic representation
 * containing concepts such as:
 *
 *     register binding;
 *     extent expression;
 *     type annotation;
 *     initializer;
 *     selection;
 *     source location.
 *
 * It MUST NOT directly create:
 *
 *     PhysicalQubitId;
 *     hardware allocation;
 *     routing decisions;
 *     schedule entries;
 *     QEC objects;
 *     ZQN channels.
 */
quantumRegisterSemanticBoundary
    : quantumRegisterDeclarationFamily
    ;


/* ============================================================================
 * 32. RESERVED EXTENSION POINT
 * ========================================================================== */

/*
 * Future register constructs should preferably extend this boundary through
 * explicit language-versioned additions.
 *
 * This prevents hardware-specific or vendor-specific concepts from leaking
 * into the core register grammar.
 */
quantumRegisterExtension
    : quantumRegisterSemanticBoundary
    ;


/*
 * ============================================================================
 * END OF FILE
 * ============================================================================
 *
 * COMPLETION CONTRACT
 *
 * This file is complete when:
 *
 *   [ ] all referenced shared parser rules exist in the canonical grammar;
 *   [ ] no lexer token is redefined here;
 *   [ ] no target-language action exists here;
 *   [ ] no hardware limit exists here;
 *   [ ] no physical topology exists here;
 *   [ ] no physical device is named here;
 *   [ ] no QEC algorithm is implemented here;
 *   [ ] no ZQN model is implemented here;
 *   [ ] register extents remain expression-based;
 *   [ ] register selection remains expression-based;
 *   [ ] logical/linear/affine intent remains semantic;
 *   [ ] AST lowering consumes these contexts;
 *   [ ] quantum::ir remains the canonical semantic boundary;
 *   [ ] routing consumes IR rather than this grammar;
 *   [ ] scheduling consumes IR rather than this grammar;
 *   [ ] hardware consumes capability/resource information rather than this
 *       grammar;
 *   [ ] runtime allocation remains outside this grammar;
 *   [ ] positive tests exist;
 *   [ ] negative tests exist;
 *   [ ] boundary/scalability tests exist;
 *   [ ] cross-domain tests exist;
 *   [ ] deterministic parsing tests exist;
 *   [ ] compatibility tests exist;
 *   [ ] no unsafe Rust is introduced by the integration;
 *   [ ] Rust 1.97 / 1.97.1 builds successfully;
 *   [ ] generated parser integration passes the repository test suite.
 *
 * ============================================================================
 */