/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/type-constraints.g4
 *
 * Grammar:
 *     TypeConstraints
 *
 * Status:
 *     Production parser delegate.
 *
 * Purpose:
 *     Canonical grammar delegate for type-level constraints/bounds.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Edition:
 *     Rust 2021
 *
 * Safety:
 *     Parser grammar only.
 *     No embedded Rust.
 *     No actions.
 *     No semantic predicates.
 *     No filesystem/network access.
 *     No unsafe code.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * Zamani source
 *      |
 *      v
 * canonical lexer
 *      |
 *      v
 * ZamaniTokens
 *      |
 *      v
 * parser grammar
 *      |
 *      +--> grammar/types/types.g4
 *      |        |
 *      |        +--> canonical typeExpression
 *      |
 *      +--> this file
 *      |        |
 *      |        +--> type-level constraint clause
 *      |        +--> ordered type-bound list
 *      |
 *      v
 * domain-neutral frontend AST
 *      |
 *      v
 * structural validation
 *      |
 *      v
 * semantic type/constraint system
 *      |
 *      +--> type checking
 *      +--> inference
 *      +--> constraint solving
 *      +--> capability interpretation
 *      +--> generic satisfiability
 *      |
 *      v
 * canonical semantic model / ZUIR
 *      |
 *      +--> classical IR
 *      +--> quantum::ir
 *      +--> HDL/hardware IR
 *      +--> resource/capability semantics
 *
 * This grammar file MUST remain upstream of all semantic and target systems.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - the colon introducing type-level bounds;
 *   - the ordered list of type bounds;
 *   - the individual type-constraint bound wrapper;
 *   - the syntax contract consumed by generic declarations;
 *   - the syntax contract consumed by where predicates when they represent
 *     type bounds;
 *   - source-order preservation of bounds.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - identifiers;
 *   - identifier Unicode rules;
 *   - keywords;
 *   - punctuation token definitions;
 *   - type expressions;
 *   - named types;
 *   - generic type applications;
 *   - tuples;
 *   - arrays;
 *   - slices;
 *   - function types;
 *   - dependent types;
 *   - quantum types;
 *   - hardware types;
 *   - resource types;
 *   - capability type definitions;
 *   - trait declarations;
 *   - interface declarations;
 *   - generic parameter declarations;
 *   - arbitrary expressions;
 *   - general constraints;
 *   - constraint solving;
 *   - type inference;
 *   - type substitution;
 *   - specialization;
 *   - monomorphization;
 *   - resource discovery;
 *   - hardware discovery;
 *   - placement;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - calibration;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - HAL;
 *   - runtime execution.
 *
 * ============================================================================
 * SINGLE TYPE-EXPRESSION AUTHORITY
 * ============================================================================
 *
 * The canonical type-expression owner is:
 *
 *     grammar/types/types.g4
 *
 * This file MUST consume:
 *
 *     typeExpression
 *
 * and MUST NOT redefine:
 *
 *     typeExpression
 *     typeAtom
 *     namedType
 *     genericType
 *     tupleType
 *     arrayType
 *     sliceType
 *     functionType
 *     dependentType
 *     quantumType
 *     hardwareType
 *     resourceType
 *
 * This prevents a second type system from being introduced accidentally.
 *
 * ============================================================================
 * SINGLE LEXER AUTHORITY
 * ============================================================================
 *
 * This parser delegate consumes the canonical token vocabulary:
 *
 *     ZamaniTokens
 *
 * from:
 *
 *     grammar/lexer/tokens.g4
 *
 * The following existing canonical tokens are required:
 *
 *     IDENTIFIER
 *     COLON
 *     PLUS
 *
 * Additional tokens such as:
 *
 *     WHERE
 *     LESS_THAN
 *     GREATER_THAN
 *     COMMA
 *     DOUBLE_COLON
 *
 * are owned by the canonical lexer and are consumed indirectly by
 * typeExpression or by other parser delegates.
 *
 * This file MUST NOT declare lexer rules.
 *
 * ============================================================================
 * NO LEXER DUPLICATION
 * ============================================================================
 *
 * Do NOT add rules such as:
 *
 *     COLON : ':' ;
 *     PLUS  : '+' ;
 *     IDENTIFIER : ... ;
 *
 * here.
 *
 * Doing so would create a competing lexical authority.
 *
 * ============================================================================
 * GENERIC BOUND SYNTAX
 * ============================================================================
 *
 * The established Zamani syntax is:
 *
 *     <T: Numeric>
 *
 *     <T: Numeric + Comparable>
 *
 *     <T: quantum::State>
 *
 *     <T: quantum::State + quantum::Measurable>
 *
 * The generic parameter itself belongs to:
 *
 *     grammar/functions/generics.g4
 *     grammar/declarations/types.g4
 *     other generic-declaration owners
 *
 * This file owns the reusable:
 *
 *     :
 *     bound
 *     +
 *     bound
 *
 * portion.
 *
 * ============================================================================
 * WHERE-BOUND SYNTAX
 * ============================================================================
 *
 * Type constraints may also be attached through a where-style predicate:
 *
 *     where T: Numeric
 *
 *     where T: Numeric + Comparable
 *
 *     where T: quantum::State
 *
 * This file does not own the complete where-clause.
 *
 * The surrounding declaration/function grammar owns:
 *
 *     WHERE
 *
 * and the subject:
 *
 *     typeExpression
 *
 * This file owns the reusable constraint suffix:
 *
 *     COLON typeConstraintBoundList
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The canonical frontend representation is conceptually:
 *
 *     TypeBounds
 *         |
 *         +--> Vec<TypeExpr>
 *
 * Each parsed:
 *
 *     typeConstraintBound
 *
 * therefore corresponds to exactly one canonical:
 *
 *     typeExpression
 *
 * in source order.
 *
 * No closed enum such as:
 *
 *     TypeBound::Trait
 *     TypeBound::Quantum
 *     TypeBound::Hardware
 *
 * is introduced by this grammar.
 *
 * This preserves the open-world architecture required for:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     AI
 *     data
 *     networking
 *     security
 *     future domains
 *
 * ============================================================================
 * SOURCE-ORDER CONTRACT
 * ============================================================================
 *
 * For:
 *
 *     T: A + B + C
 *
 * the parser must preserve:
 *
 *     A
 *     B
 *     C
 *
 * in exactly that order.
 *
 * This grammar does not:
 *
 *     sort
 *     deduplicate
 *     normalize
 *     resolve
 *     simplify
 *
 * bounds.
 *
 * Such transformations belong to semantic analysis.
 *
 * ============================================================================
 * TYPE-LEVEL SEMANTIC NEUTRALITY
 * ============================================================================
 *
 * A type bound is syntactically a type expression.
 *
 * Its semantic interpretation may later mean:
 *
 *     trait satisfaction
 *     interface satisfaction
 *     subtype relation
 *     capability satisfaction
 *     associated-type requirement
 *     type property
 *     domain capability
 *     formal type constraint
 *
 * The parser MUST NOT choose among those meanings.
 *
 * ============================================================================
 * OPEN-WORLD DOMAIN MODEL
 * ============================================================================
 *
 * This grammar intentionally does not enumerate:
 *
 *     Numeric
 *     Comparable
 *     Iterable
 *     QuantumState
 *     LogicalQubit
 *     GPU
 *     FPGA
 *     Accelerator
 *     Tensor
 *     Dataset
 *     NetworkEndpoint
 *     HardwareModule
 *
 * These are type expressions.
 *
 * Consequently, adding a future domain does not require modifying this file.
 *
 * Example:
 *
 *     T: future::computing::Capability
 *
 * is syntactically handled through canonical typeExpression.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Type constraints must describe portable program/type intent.
 *
 * They MUST NOT encode machine-specific limits.
 *
 * Forbidden language-level constraints include constructs whose meaning is:
 *
 *     maximum qubits supported by the language
 *     maximum CPUs supported by the language
 *     maximum GPUs supported by the language
 *     maximum FPGAs supported by the language
 *     maximum nodes supported by the language
 *     maximum memory supported by the language
 *     maximum accelerator count supported by the language
 *     maximum tensor dimensions supported by the language
 *
 * A programmer may use a type whose semantic definition requires a resource.
 *
 * For example:
 *
 *     T: quantum::LogicalState
 *
 * is portable.
 *
 * Whether a target can satisfy that type requirement is determined downstream.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY / RESOURCE SEPARATION
 * ============================================================================
 *
 * These concepts must remain distinct.
 *
 * TYPE BOUND:
 *
 *     T: quantum::State
 *
 * CAPABILITY:
 *
 *     capability::quantum::measurement
 *
 * RESOURCE REQUIREMENT:
 *
 *     resource::memory >= required
 *
 * TARGET:
 *
 *     target::quantum
 *
 * PREFERENCE:
 *
 *     prefer target::quantum
 *
 * IMPLEMENTATION DECISION:
 *
 *     physical qubit mapping
 *
 * None of the latter target/resource decisions belong in this grammar.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum types are consumed through typeExpression.
 *
 * Examples:
 *
 *     T: quantum::State
 *
 *     T: quantum::Operation
 *
 *     T: quantum::Observable
 *
 *     T: quantum::LogicalQubit
 *
 *     T: quantum::Circuit
 *
 * This file does not enumerate quantum types.
 *
 * It does not encode:
 *
 *     qubit counts
 *     physical qubit IDs
 *     topology
 *     gate sets
 *     calibration
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * The grammar has no dependency on quantum::ir.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Examples:
 *
 *     T: Numeric
 *
 *     T: Comparable
 *
 *     T: Iterable
 *
 *     T: Serializable
 *
 * These are ordinary type expressions.
 *
 * No classical domain-specific grammar is duplicated here.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Examples:
 *
 *     T: hdl::Module
 *
 *     T: hardware::Signal
 *
 *     T: hardware::Memory
 *
 *     T: hardware::Accelerator
 *
 * No hardware enumeration is performed.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * A resource-backed type may be used as a bound:
 *
 *     T: resource::MemoryBacked
 *
 * but resource requirements themselves remain outside this file.
 *
 * This distinction is necessary for POCO-REAF:
 *
 *     type semantics
 *          !=
 *     physical allocation
 *
 * ============================================================================
 * GENERAL-CONSTRAINT BOUNDARY
 * ============================================================================
 *
 * General source-level constraints belong to:
 *
 *     grammar/core/constraints.g4
 *
 * This file must not import the complete general constraint language simply
 * to implement generic type bounds.
 *
 * A generic type bound is intentionally narrow:
 *
 *     COLON typeConstraintBoundList
 *
 * General arithmetic, logical, resource, temporal, deployment, or hardware
 * predicates are handled by their proper constraint systems.
 *
 * ============================================================================
 * RECURSION AND NESTING
 * ============================================================================
 *
 * This file introduces no artificial depth or arity limit.
 *
 * Examples such as:
 *
 *     T: collections::Iterable<collections::Iterable<U>>
 *
 * are handled by the canonical typeExpression.
 *
 * Likewise:
 *
 *     T: a::b::c::d::E
 *
 * is handled by the canonical type-expression/name system.
 *
 * No:
 *
 *     MAX_BOUND_DEPTH
 *     MAX_QUALIFIER_DEPTH
 *     MAX_GENERIC_ARITY
 *     MAX_BOUND_COUNT
 *
 * exists here.
 *
 * Practical parser/compiler resource limits remain implementation policy,
 * not language semantics.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * The grammar must reject structurally incomplete constraints:
 *
 *     T:
 *
 *     T: +
 *
 *     T: A +
 *
 *     T: + A
 *
 *     T: A + +
 *
 *     T: A + B +
 *
 * The grammar must accept syntactically valid but semantically unresolved
 * references:
 *
 *     T: UnknownType
 *
 *     T: future::UnknownCapability
 *
 * because name/type resolution is a semantic responsibility.
 *
 * ============================================================================
 * EMPTY CONSTRAINTS
 * ============================================================================
 *
 * A generic parameter with no bound:
 *
 *     T
 *
 * is represented by the absence of:
 *
 *     typeConstraintClause
 *
 * and is therefore owned by the generic-parameter grammar.
 *
 * This file does not make the colon optional inside its own constraint clause.
 *
 * Once the clause begins:
 *
 *     :
 *
 * at least one type constraint bound is required.
 *
 * ============================================================================
 * DUPLICATION PREVENTION
 * ============================================================================
 *
 * Do not define:
 *
 *     typeExpression
 *
 * here.
 *
 * Do not define:
 *
 *     identifier
 *
 * here.
 *
 * Do not define:
 *
 *     genericParameter
 *
 * here.
 *
 * Do not define:
 *
 *     genericParameterList
 *
 * here.
 *
 * Do not define:
 *
 *     whereClause
 *
 * here.
 *
 * Do not define:
 *
 *     constraintExpression
 *
 * here.
 *
 * This keeps the grammar acyclic and composable.
 *
 * ============================================================================
 * LEGACY COMPATIBILITY
 * ============================================================================
 *
 * Existing monolithic grammar material contains older rules such as:
 *
 *     genericBoundList
 *     typeBound
 *
 * and older parser grammars also contain competing type-bound definitions.
 *
 * This file deliberately does not reproduce those names where doing so would
 * create duplicate parser rules during composition.
 *
 * Integration migration:
 *
 *     legacy genericBoundList
 *              |
 *              v
 *     typeConstraintClause
 *
 *     legacy typeBound
 *              |
 *              v
 *     typeConstraintBound
 *
 * The canonical semantic payload remains:
 *
 *     typeExpression
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Consumers that currently have:
 *
 *     genericBoundList
 *
 * should migrate their bound suffix to:
 *
 *     typeConstraintClause
 *
 * Consumers that currently have:
 *
 *     typeBound
 *
 * should use:
 *
 *     typeConstraintBound
 *
 * where a distinct rule name is required to avoid conflicts with legacy
 * imported grammars.
 *
 * The root composition grammar should import this parser delegate exactly once.
 *
 * ============================================================================
 * EXPECTED COMPOSITION
 * ============================================================================
 *
 * Conceptual parser composition:
 *
 *     parser grammar ZamaniParser;
 *
 *     options {
 *         tokenVocab = ZamaniTokens;
 *     }
 *
 *     // imports TypeConstraints and the other parser delegates.
 *
 * The exact import topology remains owned by the canonical parser composition
 * root. This file remains independently testable as a parser delegate.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     T: Numeric
 *     T: Numeric + Comparable
 *     T: quantum::State
 *     T: quantum::State + quantum::Observable
 *     T: collections::Iterable<Value>
 *     T: future::computing::Capability
 *
 * Where-style integration:
 *
 *     where T: Numeric
 *     where T: Numeric + Comparable
 *     where T: quantum::State
 *
 * Nested generic integration:
 *
 *     T: collections::Iterable<collections::Iterable<U>>
 *
 * Negative:
 *
 *     T:
 *     T: +
 *     T: + A
 *     T: A +
 *     T: A + +
 *     T: A + B +
 *
 * Boundary:
 *
 *     one bound
 *     many bounds
 *     deeply qualified types
 *     deeply nested generic types
 *     empty surrounding generic list
 *     adjacent generic parameters
 *
 * Scalability:
 *
 *     arbitrary source-level number of bounds
 *     arbitrary type-expression nesting permitted by typeExpression
 *     arbitrary qualified-name depth permitted by the name grammar
 *     arbitrary generic arity permitted by generic declaration grammar
 *
 * No test may encode an artificial machine/resource limit.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no actions
 *     no semantic predicates
 *     no mutable global state
 *     no runtime calls
 *     no I/O
 *     no randomness
 *     no hardware discovery
 *
 * The same token sequence therefore has the same parse structure.
 *
 * ============================================================================
 * SOURCE SPANS
 * ============================================================================
 *
 * The frontend must preserve source locations for:
 *
 *     the colon
 *     the complete bound list
 *     every individual type expression
 *     every '+' separator
 *
 * This enables precise diagnostics without embedding diagnostics into the
 * grammar.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * The grammar must remain declarative.
 *
 * No:
 *
 *     embedded Rust
 *     unsafe code
 *     filesystem access
 *     network access
 *     process execution
 *     environment access
 *     hardware probing
 *
 * is permitted.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This file must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *     ANTLR4
 *
 * No nightly Rust feature is relevant to this grammar.
 *
 * ============================================================================
 * PRODUCTION COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *   [x] It is a parser grammar delegate.
 *   [x] It uses the canonical ZamaniTokens vocabulary.
 *   [x] It does not declare lexer rules.
 *   [x] It consumes canonical typeExpression.
 *   [x] It preserves bound order.
 *   [x] It supports arbitrarily many bounds at the grammar level.
 *   [x] It introduces no machine-size limit.
 *   [x] It introduces no quantum-size limit.
 *   [x] It introduces no hardware-size limit.
 *   [x] It introduces no resource-size limit.
 *   [x] It introduces no domain enumeration.
 *   [x] It does not duplicate the type system.
 *   [x] It does not duplicate general constraints.
 *   [x] It does not depend on quantum::ir.
 *   [x] It does not depend on QEC/ZQN/HAL.
 *   [x] It does not depend on runtime implementation.
 *   [x] It has no embedded Rust.
 *   [x] It requires no unsafe Rust.
 *   [x] It defines explicit AST integration.
 *   [x] It defines explicit semantic integration.
 *   [x] It defines explicit IR boundaries.
 *   [x] It defines diagnostics/source-span expectations.
 *   [x] It defines positive/negative/boundary/scalability tests.
 *
 * ============================================================================
 */

parser grammar TypeConstraints;

options {
    tokenVocab = ZamaniTokens;
}

/*
 * ============================================================================
 * PUBLIC COMPOSITION RULE
 * ============================================================================
 *
 * Generic declarations and where predicates should consume this rule.
 *
 * Example:
 *
 *     T: Numeric + Comparable
 *
 * This rule owns the colon and the complete ordered bound list.
 */
typeConstraintClause
    : COLON typeConstraintBoundList
    ;

/*
 * ============================================================================
 * ORDERED TYPE-BOUND LIST
 * ============================================================================
 *
 * Existing Zamani syntax uses '+' for conjunction of type bounds:
 *
 *     T: A + B + C
 *
 * The grammar preserves source order.
 *
 * No maximum number of bounds is imposed.
 */
typeConstraintBoundList
    : typeConstraintBound
      (PLUS typeConstraintBound)*
    ;

/*
 * ============================================================================
 * SINGLE TYPE BOUND
 * ============================================================================
 *
 * The payload is exactly one canonical typeExpression.
 *
 * This is intentionally not:
 *
 *     qualifiedName
 *     identifier
 *     traitName
 *     quantumType
 *     hardwareType
 *
 * because all of those would create closed-world duplication.
 *
 * The canonical type grammar determines the complete type expression.
 */
typeConstraintBound
    : typeExpression
    ;

/*
 * ============================================================================
 * WHERE-STYLE TYPE PREDICATE SUFFIX
 * ============================================================================
 *
 * This is a reusable integration rule for grammars that already own:
 *
 *     WHERE
 *
 * and the subject:
 *
 *     typeExpression
 *
 * Example:
 *
 *     where T: Numeric + Comparable
 *
 * The complete where-clause remains owned by the surrounding grammar.
 *
 * This rule is intentionally named differently from `wherePredicate` so it
 * cannot collide with the general constraint grammar.
 */
whereTypeConstraint
    : typeExpression typeConstraintClause
    ;

/*
 * ============================================================================
 * SINGLE WHERE TYPE PREDICATE
 * ============================================================================
 *
 * This rule is useful to declaration/function generic grammar that needs a
 * complete type-bound predicate but does not own general constraint syntax.
 *
 * Example:
 *
 *     where T: Numeric
 *
 * The WHERE token remains owned by ZamaniTokens.
 */
whereTypeBoundPredicate
    : WHERE whereTypeConstraint
    ;