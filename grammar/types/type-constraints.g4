/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/type-constraints.g4
 *
 * Role:
 *     Canonical parser delegate for TYPE-SYSTEM CONSTRAINTS.
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, semantic predicates,
 *     filesystem access, network access, runtime calls, or unsafe code.
 *
 * ============================================================================
 * ARCHITECTURAL PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE SYNTAX used to express constraints/bounds on
 * types and type parameters.
 *
 * It is deliberately narrower than:
 *
 *     grammar/core/constraints.g4
 *
 * which owns general source-level constraint expressions.
 *
 * This file therefore answers:
 *
 *     "How does a type declaration/type parameter express its type-level
 *      bounds?"
 *
 * It does NOT answer:
 *
 *     "How are arbitrary program constraints expressed?"
 *
 * That distinction prevents the type system from becoming coupled to:
 *
 *     resources
 *     hardware
 *     targets
 *     scheduling
 *     routing
 *     runtime policy
 *     quantum backend selection
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - type-constraint clauses;
 *   - ordered type bounds;
 *   - type-bound lists;
 *   - type-bound expressions;
 *   - trait/interface/type capability bounds when expressed as types;
 *   - compound type bounds;
 *   - optional type-constraint clauses;
 *   - source-level syntax needed to preserve bound structure.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - identifiers;
 *   - qualified names;
 *   - lexical tokens;
 *   - general expressions;
 *   - type expressions;
 *   - generic parameter declarations;
 *   - generic applications;
 *   - trait/interface declarations;
 *   - constraint solving;
 *   - type inference;
 *   - specialization;
 *   - monomorphization;
 *   - capability discovery;
 *   - hardware discovery;
 *   - resource allocation;
 *   - target selection;
 *   - placement;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - quantum::ir;
 *   - classical IR;
 *   - HDL IR;
 *   - runtime execution.
 *
 * ============================================================================
 * IMPORTANT ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * General constraints:
 *
 *     grammar/core/constraints.g4
 *
 * Type constraints:
 *
 *     grammar/types/type-constraints.g4
 *
 * Generic applications:
 *
 *     grammar/types/generic-types.g4
 *
 * Type expressions:
 *
 *     grammar/types/types.g4
 *
 * Generic declarations:
 *
 *     declarations/generics or the repository's canonical declaration owner
 *
 * Semantic resolution:
 *
 *     frontend/type system / compiler semantic analysis
 *
 * Therefore:
 *
 *     type-constraints.g4
 *
 * MUST NOT import the complete general constraint language merely to parse
 * type bounds.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 * canonical lexer
 *       |
 *       v
 * canonical names
 *       |
 *       v
 * canonical type expressions
 *       |
 *       +--------------------+
 *       |                    |
 *       v                    v
 * generic parameters   type-constraints.g4
 *                            |
 *                            v
 *                       frontend AST
 *                            |
 *                            v
 *                    semantic type system
 *                            |
 *             +--------------+---------------+
 *             |              |               |
 *             v              v               v
 *        type checking   inference       constraint solving
 *             |                              |
 *             +--------------+---------------+
 *                            |
 *                            v
 *                      canonical semantic IR
 *                            |
 *             +--------------+---------------+
 *             |              |               |
 *             v              v               v
 *       classical IR     quantum::ir      hardware/HDL IR
 *
 * The dependency MUST NOT be reversed.
 *
 * In particular:
 *
 *     grammar -> quantum::ir
 *
 * is forbidden.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Type constraints describe semantic requirements on types.
 *
 * They MUST NOT encode a particular machine.
 *
 * Forbidden examples:
 *
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_DEVICES
 *     MAX_NODES
 *     fixed memory sizes
 *     fixed topology
 *     fixed register counts
 *     fixed accelerator counts
 *
 * A type bound such as:
 *
 *     T: QuantumState
 *
 * remains meaningful across:
 *
 *     simulators
 *     quantum processors
 *     embedded systems
 *     distributed systems
 *     future machines
 *
 * The semantic layer determines whether a particular target can satisfy
 * the bound.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Type bounds are intentionally represented through type expressions rather
 * than an enumerated list such as:
 *
 *     numericConstraint
 *     quantumConstraint
 *     hardwareConstraint
 *     gpuConstraint
 *     fpgaConstraint
 *     tensorConstraint
 *
 * New domains therefore do not require modification of this grammar merely
 * because a new type-level abstraction is introduced.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer owns all token definitions.
 *
 * This parser delegate therefore MUST NOT declare lexer rules.
 *
 * The following token categories are expected from the canonical lexer:
 *
 *     IDENT
 *     DOUBLE_COLON
 *     COLON
 *     COMMA
 *     LESS_THAN
 *     GREATER_THAN
 *     QUESTION
 *     AMPERSAND
 *     PIPE
 *     LPAREN
 *     RPAREN
 *
 * The actual repository token names are authoritative.
 *
 * If the canonical lexer uses different names, the integration change must
 * update the composed grammar vocabulary rather than introducing duplicate
 * lexer definitions here.
 *
 * ============================================================================
 * TYPE EXPRESSION CONTRACT
 * ============================================================================
 *
 * This file consumes the canonical type expression rule:
 *
 *     typeExpression
 *
 * It MUST NOT redefine:
 *
 *     typeExpression
 *     typeAtom
 *     primitiveType
 *     referenceType
 *     tupleType
 *     functionType
 *     arrayType
 *     mapType
 *     optionType
 *     resultType
 *     quantumType
 *     hardwareType
 *
 * Those rules belong to their respective type grammar delegates.
 *
 * ============================================================================
 * GENERIC PARAMETER CONTRACT
 * ============================================================================
 *
 * This file does not declare generic parameters.
 *
 * A generic declaration may conceptually look like:
 *
 *     <T>
 *
 * or:
 *
 *     <T: Numeric>
 *
 * or:
 *
 *     <T: Numeric + Comparable>
 *
 * or:
 *
 *     <T: quantum::State>
 *
 * The generic declaration grammar owns:
 *
 *     parameter name
 *     parameter delimiter
 *     parameter list
 *
 * This file owns only the constraint portion:
 *
 *     Numeric
 *     Numeric + Comparable
 *     quantum::State
 *
 * ============================================================================
 * ORDER PRESERVATION
 * ============================================================================
 *
 * Constraint order MUST be preserved by the parse tree.
 *
 * Example:
 *
 *     T: A + B + C
 *
 * produces an ordered sequence:
 *
 *     A
 *     B
 *     C
 *
 * Semantic analysis may normalize or canonicalize that sequence later.
 *
 * The grammar must not sort, deduplicate, or otherwise transform source
 * information.
 *
 * ============================================================================
 * TYPE BOUND MODEL
 * ============================================================================
 *
 * A type constraint is syntactically:
 *
 *     typeConstraint
 *
 * consisting of:
 *
 *     typeBound
 *
 * optionally followed by additional bounds.
 *
 * The canonical semantic interpretation is expected to be equivalent to:
 *
 *     Bound(A)
 *     Bound(B)
 *     ...
 *
 * without the grammar deciding whether those bounds mean:
 *
 *     trait implementation
 *     interface implementation
 *     subtype relationship
 *     capability satisfaction
 *     associated-type availability
 *     semantic type property
 *
 * Such interpretation belongs to semantic analysis.
 *
 * ============================================================================
 * COMPOUND BOUNDS
 * ============================================================================
 *
 * Compound bounds are supported through:
 *
 *     +
 *
 * and, where the language requires explicit boolean/type-bound alternatives,
 *
 *     |
 *
 * The exact semantic meaning is determined by the type system.
 *
 * Example:
 *
 *     T: Numeric + Comparable
 *
 * means that T has both bounds.
 *
 * A future type-system implementation may also support:
 *
 *     T: A | B
 *
 * as an alternative bound.
 *
 * This grammar preserves the distinction between:
 *
 *     conjunction
 *
 * and:
 *
 *     alternative
 *
 * without deciding subtype/trait semantics.
 *
 * ============================================================================
 * NO MACHINE-LEVEL LIMITS
 * ============================================================================
 *
 * These rules intentionally use unbounded parser repetition:
 *
 *     *
 *     +
 *
 * There is no grammar-level maximum for:
 *
 *     number of type parameters
 *     number of bounds
 *     bound nesting
 *     qualified-name depth
 *     generic nesting
 *     number of declarations
 *     number of types
 *
 * Practical limits are compiler/resource/security policy.
 *
 * They must not be encoded as language semantics.
 *
 * ============================================================================
 * SYNTAX EXAMPLES
 * ============================================================================
 *
 * Simple bound:
 *
 *     T: Numeric
 *
 * Multiple bounds:
 *
 *     T: Numeric + Comparable
 *
 * Qualified bound:
 *
 *     T: quantum::State
 *
 * Nested type bound:
 *
 *     T: collections::Iterable<Value>
 *
 * Compound qualified bound:
 *
 *     T: quantum::State + quantum::Measurable
 *
 * Multiple generic parameters:
 *
 *     <T: Numeric, U: Comparable>
 *
 * The generic declaration itself is NOT owned by this file.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser output must permit the frontend AST to construct a semantic
 * representation conceptually equivalent to:
 *
 *     TypeConstraint {
 *         bounds: Vec<TypeBound>,
 *         source_span
 *     }
 *
 * and:
 *
 *     TypeBound {
 *         type_expression,
 *         source_span
 *     }
 *
 * Compound expressions must preserve their source structure where the AST
 * requires it.
 *
 * The grammar itself contains no Rust structures.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *   - name resolution;
 *   - type resolution;
 *   - trait/interface resolution;
 *   - bound compatibility;
 *   - subtype checking;
 *   - associated-type validation;
 *   - generic inference;
 *   - constraint solving;
 *   - ambiguity detection;
 *   - cycle detection;
 *   - satisfiability;
 *   - specialization;
 *   - monomorphization.
 *
 * The grammar only establishes syntactic structure.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum types may appear as type bounds:
 *
 *     T: quantum::State
 *
 *     T: quantum::Operation
 *
 *     T: quantum::Observable
 *
 *     T: quantum::LogicalQubit
 *
 * This file does not define any of those quantum types.
 *
 * It also does not define:
 *
 *     physical qubit allocation
 *     topology
 *     gate set
 *     calibration
 *     backend selection
 *     scheduling
 *     routing
 *     QEC
 *     ZQN
 *
 * The canonical semantic quantum boundary remains:
 *
 *     quantum::ir
 *
 * The grammar does not construct or depend directly on that IR.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical types may be used as bounds:
 *
 *     T: Numeric
 *     T: Comparable
 *     T: Iterable
 *     T: Serializable
 *
 * Their semantic definitions belong to the type system and standard/domain
 * libraries, not to this grammar.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware-related type bounds may be represented as normal qualified types:
 *
 *     T: hardware::Signal
 *     T: hardware::Memory
 *     T: hardware::Accelerator
 *     T: hdl::Module
 *
 * This grammar does not enumerate hardware classes.
 *
 * Therefore future hardware abstractions remain source-compatible.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource requirements are NOT type constraints merely because a type may
 * consume resources.
 *
 * For example:
 *
 *     T: quantum::State
 *
 * is a type-level bound.
 *
 * A statement such as:
 *
 *     requires resource::memory >= ...
 *
 * belongs to the requirement/resource constraint system.
 *
 * This separation prevents the type system from becoming a hidden hardware
 * allocation system.
 *
 * ============================================================================
 * GENERAL CONSTRAINT INTEGRATION
 * ============================================================================
 *
 * General semantic constraints belong to:
 *
 *     grammar/core/constraints.g4
 *
 * This file may be referenced by generic/type declaration grammar when a
 * type-level bound is required.
 *
 * It must not import the complete general constraint grammar merely to
 * implement ordinary type bounds.
 *
 * ============================================================================
 * ERROR HANDLING
 * ============================================================================
 *
 * The grammar must reject malformed type-bound syntax such as:
 *
 *     T:
 *
 *     T: +
 *
 *     T: A +
 *
 *     T: + A
 *
 *     T: A |
 *
 *     T: | A
 *
 *     T: A +
 *
 * Semantic invalidity is intentionally outside the parser.
 *
 * Examples that should parse and be rejected later if semantically invalid:
 *
 *     T: UnknownType
 *
 *     T: incompatible::Bound
 *
 *     T: ConcreteTypeThatCannotBeABound
 *
 * This distinction provides stable diagnostics and preserves forward
 * compatibility.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no actions
 *     no semantic predicates
 *     no external state
 *     no filesystem access
 *     no network access
 *     no random behavior
 *     no hardware discovery
 *     no runtime calls
 *
 * Identical token streams therefore produce identical parse structures.
 *
 * ============================================================================
 * SOURCE SPAN REQUIREMENT
 * ============================================================================
 *
 * The frontend must preserve source spans for:
 *
 *     entire constraint clause
 *     individual bounds
 *     compound-bound operators
 *     referenced type expressions
 *
 * This is necessary for production-quality diagnostics such as:
 *
 *     "type parameter T does not satisfy bound Numeric"
 *
 * and:
 *
 *     "conflicting bounds ..."
 *
 * The grammar does not manufacture spans; ANTLR token positions provide the
 * source information consumed by the frontend.
 *
 * ============================================================================
 * VERSIONING
 * ============================================================================
 *
 * Changes to the syntactic contract of these rules are language changes and
 * must be coordinated with:
 *
 *     grammar/specification/language-version.md
 *     grammar/compatibility/
 *     frontend AST
 *     type checker
 *     generic parameter parsing
 *     compiler diagnostics
 *     grammar tests
 *
 * Adding new domain-specific type names MUST NOT require a grammar change.
 *
 * ============================================================================
 * PRODUCTION COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *   [ ] It composes with the canonical Zamani lexer.
 *   [ ] It contains parser rules only.
 *   [ ] It contains no embedded Rust.
 *   [ ] It requires no unsafe Rust.
 *   [ ] It introduces no machine-size limits.
 *   [ ] It introduces no quantum-count limits.
 *   [ ] It introduces no hardware-count limits.
 *   [ ] It does not redefine identifiers.
 *   [ ] It does not redefine qualified names.
 *   [ ] It does not redefine type expressions.
 *   [ ] It does not redefine generic applications.
 *   [ ] It does not redefine general constraints.
 *   [ ] It preserves bound ordering.
 *   [ ] It supports qualified type bounds.
 *   [ ] It supports compound bounds.
 *   [ ] It supports arbitrary bound-list length.
 *   [ ] It supports nested type expressions through typeExpression.
 *   [ ] It remains domain neutral.
 *   [ ] It has positive parser tests.
 *   [ ] It has negative parser tests.
 *   [ ] It has scalability tests.
 *   [ ] It has quantum/classical/HDL cross-domain tests.
 *   [ ] It has AST integration tests.
 *   [ ] It has semantic type-checking integration tests.
 *   [ ] It has compatibility tests.
 *
 * ============================================================================
 */

parser grammar TypeConstraints;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. TYPE CONSTRAINT CLAUSE
 * ========================================================================== */

/**
 * A complete type-level constraint clause.
 *
 * The owning generic/type declaration grammar is responsible for introducing
 * the parameter and deciding where this clause occurs.
 *
 * Examples:
 *
 *     T: Numeric
 *
 *     T: Numeric + Comparable
 *
 *     T: quantum::State
 *
 * The colon belongs to the type-constraint syntax when this delegate is
 * composed into a generic parameter declaration.
 */
typeConstraintClause
    : COLON
      typeBoundExpression
    ;


/* ============================================================================
 * 2. TYPE BOUND EXPRESSION
 * ========================================================================== */

/**
 * Top-level type-bound expression.
 *
 * `|` represents alternative bounds where the semantic type system supports
 * them.
 *
 * Precedence:
 *
 *     conjunction (+) binds more tightly than alternative (|)
 */
typeBoundExpression
    : typeBoundAlternative
      (
          PIPE
          typeBoundAlternative
      )*
    ;


/* ============================================================================
 * 3. ALTERNATIVE TYPE BOUNDS
 * ========================================================================== */

/**
 * One alternative consists of one or more conjunctive bounds.
 *
 * Example:
 *
 *     T: A + B + C
 */
typeBoundAlternative
    : typeBoundConjunction
    ;


/* ============================================================================
 * 4. CONJUNCTIVE TYPE BOUNDS
 * ========================================================================== */

/**
 * Multiple simultaneously required bounds.
 *
 * Examples:
 *
 *     T: Numeric + Comparable
 *
 *     T: quantum::State + quantum::Measurable
 */
typeBoundConjunction
    : typeBoundPrimary
      (
          AMPERSAND
          typeBoundPrimary
      )*
    ;


/* ============================================================================
 * 5. PRIMARY TYPE BOUND
 * ========================================================================== */

/**
 * Primary bound.
 *
 * A bound is structurally a type expression.
 *
 * The canonical type system determines whether the resulting type expression
 * is legal as a bound.
 */
typeBoundPrimary
    : typeExpression
    | LPAREN
      typeBoundExpression
      RPAREN
    ;


/* ============================================================================
 * 6. ORDERED TYPE-BOUND LIST
 * ========================================================================== */

/**
 * Explicit ordered list form used by AST builders and generic declaration
 * grammar when a list rather than a single compound expression is desired.
 *
 * Example:
 *
 *     Numeric + Comparable + Serializable
 *
 * The parser preserves source order.
 */
typeBoundList
    : typeBoundPrimary
      (
          AMPERSAND
          typeBoundPrimary
      )*
    ;


/* ============================================================================
 * 7. OPTIONAL TYPE CONSTRAINT
 * ========================================================================== */

/**
 * Optional helper rule for generic parameter declarations.
 *
 * This rule consumes either:
 *
 *     no constraint
 *
 * or:
 *
 *     a complete type constraint clause.
 *
 * The generic parameter grammar may therefore use:
 *
 *     typeParameterConstraint?
 *
 * without reproducing the constraint syntax.
 */
typeParameterConstraint
    : typeConstraintClause
    ;


/* ============================================================================
 * 8. SINGLE TYPE BOUND
 * ========================================================================== */

/**
 * Explicit single-bound rule.
 *
 * Useful for generic declaration grammars and AST builders that distinguish
 * one bound from a compound bound.
 */
singleTypeBound
    : typeBoundPrimary
    ;


/* ============================================================================
 * 9. COMPOUND TYPE BOUND
 * ========================================================================== */

/**
 * Compound type bound.
 *
 * At least two bounds are required.
 *
 * Examples:
 *
 *     A & B
 *
 *     A & B & C
 */
compoundTypeBound
    : typeBoundPrimary
      AMPERSAND
      typeBoundPrimary
      (
          AMPERSAND
          typeBoundPrimary
      )*
    ;


/* ============================================================================
 * 10. ALTERNATIVE TYPE BOUND
 * ========================================================================== */

/**
 * Alternative type bound.
 *
 * At least two alternatives are required.
 *
 * Examples:
 *
 *     A | B
 *
 *     A | B | C
 */
alternativeTypeBound
    : typeBoundAlternative
      PIPE
      typeBoundAlternative
      (
          PIPE
          typeBoundAlternative
      )*
    ;


/* ============================================================================
 * 11. TYPE BOUND REFERENCE
 * ========================================================================== */

/**
 * A semantically resolved bound ultimately references a type.
 *
 * This rule intentionally delegates to the canonical type expression rather
 * than introducing a duplicate type-name grammar.
 *
 * It exists as an explicit integration point for frontend grammar composition.
 */
typeBoundReference
    : typeExpression
    ;


/* ============================================================================
 * 12. QUALIFIED TYPE BOUND
 * ========================================================================== */

/**
 * Qualified bounds are accepted through the canonical typeExpression rule.
 *
 * This named integration rule exists so consumers can identify the relevant
 * parse-tree node without redefining qualified-name syntax.
 *
 * Examples:
 *
 *     quantum::State
 *     hardware::Signal
 *     collections::Iterable<T>
 *     future::computing::Capability
 */
qualifiedTypeBound
    : typeExpression
    ;


/* ============================================================================
 * 13. GENERIC TYPE BOUND
 * ========================================================================== */

/**
 * Generic type bounds are also represented by the canonical type expression.
 *
 * Examples:
 *
 *     Iterable<T>
 *
 *     Container<Value>
 *
 *     quantum::Register<Qubit>
 *
 * Semantic analysis determines whether the resulting expression is a valid
 * bound.
 */
genericTypeBound
    : typeExpression
    ;


/* ============================================================================
 * 14. NESTED TYPE BOUND
 * ========================================================================== */

/**
 * Parenthesized nested bound.
 *
 * Example:
 *
 *     T: (A & B)
 *
 *     T: (A | B)
 *
 * Parentheses preserve explicit source grouping.
 */
nestedTypeBound
    : LPAREN
      typeBoundExpression
      RPAREN
    ;


/* ============================================================================
 * 15. TYPE BOUND SEQUENCE
 * ========================================================================== */

/**
 * Ordered sequence of constraints.
 *
 * This rule is useful when a declaration grammar needs to attach multiple
 * syntactically separate bounds while retaining their source order.
 *
 * Example conceptual representation:
 *
 *     T: A & B & C
 *
 * becomes:
 *
 *     A
 *     B
 *     C
 */
typeBoundSequence
    : typeBoundPrimary
      (
          AMPERSAND
          typeBoundPrimary
      )*
    ;


/* ============================================================================
 * 16. TYPE CONSTRAINT GROUP
 * ========================================================================== */

/**
 * Explicit grouping for a complete type constraint expression.
 */
typeConstraintGroup
    : LPAREN
      typeBoundExpression
      RPAREN
    ;


/* ============================================================================
 * 17. TYPE CONSTRAINT LIST
 * ========================================================================== */

/**
 * Comma-separated constraints for grammar owners that need to represent
 * multiple independent constraint clauses.
 *
 * Example:
 *
 *     : Numeric, Comparable
 *
 * The generic parameter grammar should normally prefer one canonical
 * constraint clause where the language specification defines a single
 * bound expression. This rule exists only as a composition point where the
 * declaration grammar explicitly permits multiple clauses.
 */
typeConstraintList
    : typeBoundExpression
      (
          COMMA
          typeBoundExpression
      )*
    ;


/* ============================================================================
 * 18. CONSTRAINT TERMINATOR
 * ========================================================================== */

/**
 * This delegate deliberately does NOT consume semicolons, commas belonging to
 * an enclosing generic parameter list, or declaration terminators unless the
 * owning grammar explicitly delegates them here.
 *
 * This keeps the rule reusable in:
 *
 *     generic parameters
 *     type declarations
 *     associated types
 *     function type parameters
 *     trait declarations
 *     interface declarations
 *     future type-system constructs
 *
 * The enclosing grammar owns statement/declaration termination.
 */


/* ============================================================================
 * 19. SEMANTIC EXTENSION POINT
 * ========================================================================== */

/**
 * Domain-specific semantic extensions must enter through typeExpression.
 *
 * The grammar therefore remains stable when new types are added:
 *
 *     quantum::...
 *     hardware::...
 *     hdl::...
 *     ai::...
 *     distributed::...
 *     future::...
 *
 * No domain-specific alternatives are intentionally listed here.
 */


/* ============================================================================
 * 20. RESOURCE-SCALABLE TYPE CONSTRAINTS
 * ========================================================================== */

/**
 * A type expression may contain symbolic parameters:
 *
 *     Tensor<T, N>
 *
 *     Matrix<T, Rows, Cols>
 *
 *     QuantumRegister<Qubit, Width>
 *
 * This grammar does not evaluate those parameters.
 *
 * Resource interpretation belongs downstream.
 *
 * Therefore the grammar never converts:
 *
 *     Width
 *
 * into a machine-specific maximum.
 */


/* ============================================================================
 * 21. QUANTUM SAFETY BOUNDARY
 * ========================================================================== */

/**
 * Quantum type bounds remain semantic type constraints.
 *
 * Valid syntactic examples include:
 *
 *     T: quantum::State
 *
 *     T: quantum::Operation
 *
 *     T: quantum::Observable
 *
 *     T: quantum::LogicalQubit
 *
 *     T: quantum::State & quantum::Measurable
 *
 * This grammar does not:
 *
 *     allocate qubits
 *     choose physical qubits
 *     select QPUs
 *     inspect calibration
 *     select topology
 *     schedule operations
 *     route circuits
 *     invoke QEC
 *     inspect ZQN
 *     construct quantum::ir
 */


/* ============================================================================
 * 22. HARDWARE SAFETY BOUNDARY
 * ========================================================================== */

/**
 * Hardware type bounds are symbolic.
 *
 * Examples:
 *
 *     T: hardware::Signal
 *
 *     T: hardware::Memory
 *
 *     T: hardware::Accelerator
 *
 *     T: hdl::Module
 *
 * No physical device identity is part of this grammar.
 */


/* ============================================================================
 * 23. DISTRIBUTED SAFETY BOUNDARY
 * ========================================================================== */

/**
 * Distributed type bounds may be represented symbolically:
 *
 *     T: distributed::Serializable
 *
 *     T: distributed::Replicable
 *
 *     T: distributed::Message
 *
 * The grammar does not choose:
 *
 *     node
 *     cluster
 *     cloud
 *     network
 *     provider
 *
 * Those belong to downstream realization.
 */


/* ============================================================================
 * 24. FUTURE-PROOFING
 * ========================================================================== */

/**
 * Future computational domains do not require grammar changes merely because
 * new type names are introduced.
 *
 * Example:
 *
 *     T: future::substrate::Computable
 *
 * is structurally handled through typeExpression.
 *
 * The semantic registry determines whether the type exists.
 */


/* ============================================================================
 * 25. PARSER COMPOSITION CONTRACT
 * ========================================================================== */

/**
 * The parent grammar must compose this delegate and provide the canonical
 * `typeExpression` rule.
 *
 * Conceptually:
 *
 *     genericParameter
 *         : identifier
 *           typeParameterConstraint?
 *         ;
 *
 * where:
 *
 *     typeParameterConstraint
 *         -> typeConstraintClause
 *
 * This file therefore does not define the parent generic-parameter rule.
 */


/* ============================================================================
 * 26. AST INTEGRATION CONTRACT
 * ========================================================================== */

/**
 * Frontend conversion should map:
 *
 *     typeConstraintClause
 *             |
 *             v
 *     TypeConstraint
 *
 * and:
 *
 *     typeBoundExpression
 *             |
 *             v
 *     ConstraintExpression / TypeBoundExpression
 *
 * according to the repository's canonical AST types.
 *
 * The grammar must not introduce a competing representation.
 *
 * Existing frontend structures already distinguish generic parameters and
 * ordered type constraints; this grammar is designed to feed that model.
 */


/* ============================================================================
 * 27. TYPE-CHECKER INTEGRATION
 * ========================================================================== */

/**
 * Semantic type checking consumes the AST generated from this grammar.
 *
 * It is responsible for:
 *
 *     resolve bound
 *     validate bound
 *     check compatibility
 *     solve bounds
 *     report conflicts
 *     infer missing information
 *
 * Parser acceptance MUST NOT imply semantic validity.
 */


/* ============================================================================
 * 28. COMPILER INTEGRATION
 * ========================================================================== */

/**
 * Compiler stages may consume resolved type constraints for:
 *
 *     type checking
 *     generic specialization
 *     lowering
 *     optimization legality
 *     target capability checking
 *
 * A compiler backend MUST NOT modify this grammar.
 *
 * Target-specific failure belongs to target analysis/diagnostics.
 */


/* ============================================================================
 * 29. IR INTEGRATION
 * ========================================================================== */

/**
 * This grammar has no direct IR dependency.
 *
 * After semantic analysis:
 *
 *     TypeConstraint
 *          |
 *          v
 *     canonical semantic type model
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware IR
 *
 * The grammar MUST NOT construct:
 *
 *     quantum::ir
 *     scheduling IR
 *     routing IR
 *     hardware allocation IR
 */


/* ============================================================================
 * 30. RUNTIME INTEGRATION
 * ========================================================================== */

/**
 * Runtime execution does not parse this grammar.
 *
 * Runtime receives compiled semantic representations.
 *
 * Runtime capability mismatch is a downstream concern.
 *
 * The source-level type constraint remains stable across execution targets.
 */


/* ============================================================================
 * 31. DIAGNOSTIC CONTRACT
 * ========================================================================== */

/**
 * Diagnostics should identify:
 *
 *     generic parameter
 *     offending bound
 *     source span
 *     expected type/bound category
 *
 * Examples:
 *
 *     type parameter `T` has an invalid bound
 *
 *     type `T` does not satisfy bound `Numeric`
 *
 *     conflicting bounds on type parameter `T`
 *
 * The grammar itself only provides the structural information required to
 * generate these diagnostics.
 */


/* ============================================================================
 * 32. NEGATIVE-SYNTAX CONTRACT
 * ========================================================================== */

/**
 * These forms must be rejected:
 *
 *     T:
 *
 *     T: +
 *
 *     T: A +
 *
 *     T: + A
 *
 *     T: A &
 *
 *     T: & A
 *
 *     T: A |
 *
 *     T: | A
 *
 *     T: A B
 *
 *     T: , A
 *
 *     T: A,
 *
 * when those forms occur inside a type-bound expression and the enclosing
 * grammar does not explicitly permit the separator.
 */


/* ============================================================================
 * 33. SEMANTICALLY UNKNOWN BUT SYNTACTICALLY VALID
 * ========================================================================== */

/**
 * These should parse:
 *
 *     T: future::UnknownCapability
 *
 *     T: quantum::FutureState
 *
 *     T: hardware::FutureAccelerator
 *
 * even when semantic registries do not yet know those names.
 *
 * Semantic analysis may subsequently produce:
 *
 *     unresolved
 *     unknown
 *     unsupported
 *
 * diagnostics according to repository policy.
 *
 * This is essential for POCO-REAF and language evolution.
 */


/* ============================================================================
 * 34. SCALABILITY CONTRACT
 * ========================================================================== */

/**
 * The grammar imposes no semantic maximum on:
 *
 *     type parameters
 *     bounds
 *     nesting
 *     domain names
 *     generic type nesting
 *     source file size
 *     program size
 *
 * Any parser/compiler stack or memory limitation is an implementation
 * constraint and must be represented through explicit compiler/resource policy,
 * not source-language constants.
 */


/* ============================================================================
 * 35. DETERMINISM CONTRACT
 * ========================================================================== */

/**
 * Parsing must be deterministic for a fixed token stream.
 *
 * There are:
 *
 *     no actions
 *     no semantic predicates
 *     no random choices
 *     no environment reads
 *     no filesystem access
 *     no network access
 *     no hardware access
 */


/* ============================================================================
 * 36. SECURITY CONTRACT
 * ========================================================================== */

/**
 * This grammar must not:
 *
 *     execute expressions
 *     evaluate compile-time code
 *     access files
 *     access networks
 *     invoke processes
 *     inspect hardware
 *     allocate runtime resources
 *
 * Compile-time evaluation belongs to a separately controlled compiler phase.
 */


/* ============================================================================
 * 37. COMPATIBILITY CONTRACT
 * ========================================================================== */

/**
 * Adding a new type:
 *
 *     quantum::NewType
 *     hardware::NewType
 *     future::NewType
 *
 * must not require this grammar to change if it conforms to the canonical
 * type-expression grammar.
 *
 * Syntax changes to this file require coordinated changes to:
 *
 *     generic parameter grammar
 *     frontend AST conversion
 *     type checker
 *     compiler diagnostics
 *     grammar tests
 *     language specification
 *
 * Existing valid source must remain valid unless an explicit language-version
 * migration says otherwise.
 */


/* ============================================================================
 * 38. HARD-CODING AUDIT
 * ========================================================================== */

/**
 * Forbidden in this file:
 *
 *     numeric machine limits
 *     fixed generic arity
 *     fixed type-bound count
 *     fixed nesting depth
 *     fixed qubit count
 *     fixed core count
 *     fixed memory size
 *     fixed accelerator count
 *     fixed device count
 *     vendor-specific type enumeration
 *     backend-specific identifiers
 *
 * No such values are encoded by this grammar.
 */


/* ============================================================================
 * 39. TEST CONTRACT
 * ========================================================================== */

/**
 * Positive tests MUST cover:
 *
 *     T: Numeric
 *     T: Numeric & Comparable
 *     T: quantum::State
 *     T: hardware::Signal
 *     T: hdl::Module
 *     T: future::substrate::Type
 *     T: A & B & C
 *     T: A | B
 *     T: (A & B)
 *     T: Generic<T>
 *     T: Generic<Generic<T>>
 *
 * Negative tests MUST cover:
 *
 *     missing bound
 *     dangling operator
 *     leading operator
 *     malformed grouping
 *     malformed qualified type
 *     malformed generic bound
 *
 * Cross-domain tests MUST cover:
 *
 *     classical + quantum
 *     quantum + hardware
 *     classical + HDL
 *     quantum + HDL
 *     distributed + hardware
 *     AI + quantum
 *
 * Scalability tests MUST generate:
 *
 *     many bounds
 *     deeply nested generic types
 *     deeply qualified type names
 *
 * without any grammar-defined maximum.
 */


/* ============================================================================
 * 40. COMPLETION CRITERIA
 * ========================================================================== */

/**
 * This file is COMPLETE only when all of the following hold:
 *
 *   [ ] Canonical Zamani lexer vocabulary is used.
 *   [ ] No lexer rules are duplicated.
 *   [ ] `typeExpression` is delegated to the canonical type grammar.
 *   [ ] Generic parameter declarations remain outside this file.
 *   [ ] Generic applications remain outside this file.
 *   [ ] General constraints remain outside this file.
 *   [ ] Type-bound syntax is complete.
 *   [ ] Bound order is preserved.
 *   [ ] Compound bounds are represented.
 *   [ ] Alternative bounds are represented where supported.
 *   [ ] Parenthesized bounds are represented.
 *   [ ] Qualified types are delegated.
 *   [ ] Generic types are delegated.
 *   [ ] No machine-specific limits exist.
 *   [ ] No quantum-specific limits exist.
 *   [ ] No hardware-specific limits exist.
 *   [ ] No unsafe code exists.
 *   [ ] No embedded Rust actions exist.
 *   [ ] No runtime dependency exists.
 *   [ ] AST conversion has a defined contract.
 *   [ ] Type-checking integration has a defined contract.
 *   [ ] Compiler integration has a defined contract.
 *   [ ] IR integration remains downstream.
 *   [ ] `quantum::ir` is not imported or duplicated.
 *   [ ] QEC is not implemented here.
 *   [ ] ZQN is not implemented here.
 *   [ ] Routing is not implemented here.
 *   [ ] Scheduling is not implemented here.
 *   [ ] Hardware discovery is not implemented here.
 *   [ ] Positive tests exist.
 *   [ ] Negative tests exist.
 *   [ ] Boundary tests exist.
 *   [ ] Cross-domain tests exist.
 *   [ ] Scalability tests exist.
 *   [ ] Determinism tests exist.
 *   [ ] Compatibility tests exist.
 */