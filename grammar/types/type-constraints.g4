/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/type-constraints.g4
 *
 * Grammar:
 *     TypeConstraints
 *
 * Status:
 *     COMPATIBILITY / MIGRATION FACADE
 *
 * Purpose:
 *     Preserve the existing type-constraint rule names while delegating the
 *     actual type-bound syntax to the canonical TypeBounds grammar.
 *
 * Compiler baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * This file is NOT a second implementation of type-bound syntax.
 *
 * The canonical implementation is:
 *
 *     grammar/types/bounds.g4
 *
 * whose grammar name is:
 *
 *     TypeBounds
 *
 * This file exists only so existing grammar consumers that use:
 *
 *     typeConstraintClause
 *     typeConstraintBoundList
 *     typeConstraintBound
 *
 * can migrate without creating another constraint implementation.
 *
 * Canonical ownership:
 *
 *     TypeBounds
 *         |
 *         +--> typeBoundClause
 *         +--> typeBoundList
 *         +--> typeBound
 *
 * Compatibility names:
 *
 *     TypeConstraints
 *         |
 *         +--> typeConstraintClause
 *         +--> typeConstraintBoundList
 *         +--> typeConstraintBound
 *
 * Both representations describe the same semantic structure.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     compatibility aliases for historical/current type-constraint rule names
 *
 * THIS FILE DOES NOT OWN:
 *
 *     typeExpression
 *     typeCore
 *     typeBoundClause
 *     typeBoundList
 *     typeBound
 *     genericParameter
 *     genericParameterList
 *     generic type application
 *     whereClause
 *     general constraint expressions
 *     resource constraints
 *     capability negotiation
 *     contracts
 *     policies
 *     effects
 *     type inference
 *     unification
 *     substitution
 *     specialization
 *     monomorphization
 *     target selection
 *     hardware discovery
 *     quantum allocation
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *
 * ============================================================================
 * CANONICAL OWNERS
 * ============================================================================
 *
 * Type expressions:
 *
 *     grammar/types/types.g4
 *
 * Canonical type-bound syntax:
 *
 *     grammar/types/bounds.g4
 *
 * Generic type application:
 *
 *     grammar/types/generic.g4
 *
 * General source constraints:
 *
 *     grammar/core/constraints.g4
 *
 * Resource constraints:
 *
 *     grammar/resources/
 *
 * Capability semantics:
 *
 *     grammar/resources/
 *     grammar/core/
 *
 * Effects:
 *
 *     grammar/effects/
 *
 * Contracts:
 *
 *     grammar/validation/
 *
 * Policies:
 *
 *     grammar/policies/
 *
 * ============================================================================
 * SINGLE TYPE-EXPRESSION AUTHORITY
 * ============================================================================
 *
 * This file MUST NOT define:
 *
 *     typeExpression
 *
 * or any substitute such as:
 *
 *     constraintTypeExpression
 *     boundTypeExpression
 *     genericBoundType
 *     typeConstraintExpression
 *
 * Every actual bound is parsed by the canonical:
 *
 *     typeExpression
 *
 * through:
 *
 *     TypeBounds.typeBound
 *
 * This preserves one source-level type language across:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     AI
 *     data
 *     distributed
 *     networking
 *     security
 *     future domains
 *
 * ============================================================================
 * SINGLE BOUND AUTHORITY
 * ============================================================================
 *
 * The canonical syntax is:
 *
 *     T: A
 *
 *     T: A + B
 *
 *     T: A + B + C
 *
 * and is implemented only by:
 *
 *     grammar/types/bounds.g4
 *
 * This file merely exposes compatibility names for those rules.
 *
 * ============================================================================
 * SINGLE LEXER AUTHORITY
 * ============================================================================
 *
 * This file contains no lexer rules.
 *
 * Tokens ultimately come from:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through:
 *
 *     tokenVocab = ZamaniLexer
 *
 * This grammar MUST NOT declare:
 *
 *     COLON
 *     PLUS
 *     IDENTIFIER
 *     WHERE
 *     LESS_THAN
 *     GREATER_THAN
 *     COMMA
 *
 * or any other lexer token.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This grammar imports:
 *
 *     TypeBounds
 *
 * The resulting composition is:
 *
 *     TypeConstraints
 *          |
 *          +--> TypeBounds
 *                  |
 *                  +--> typeBoundClause
 *                  +--> typeBoundList
 *                  +--> typeBound
 *
 * The canonical parser composition root must import this compatibility
 * facade instead of separately importing both TypeConstraints and TypeBounds
 * for the same parser composition.
 *
 * This avoids duplicate rule ownership and duplicate delegate paths.
 *
 * ============================================================================
 * COMPATIBILITY RULES
 * ============================================================================
 *
 * The compatibility names intentionally map one-to-one:
 *
 *     typeConstraintClause
 *         -> typeBoundClause
 *
 *     typeConstraintBoundList
 *         -> typeBoundList
 *
 *     typeConstraintBound
 *         -> typeBound
 *
 * No semantic transformation occurs here.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar introduces no new AST type.
 *
 * The compatibility aliases must lower to the same semantic representation
 * as the canonical TypeBounds rules.
 *
 * Conceptually:
 *
 *     typeConstraintClause
 *          |
 *          v
 *     typeBoundClause
 *          |
 *          v
 *     TypeBounds
 *          |
 *          v
 *     ordered TypeExpr collection
 *
 * For:
 *
 *     T: A + B + C
 *
 * source order MUST remain:
 *
 *     A
 *     B
 *     C
 *
 * The compatibility layer must not:
 *
 *     sort
 *     deduplicate
 *     normalize
 *     resolve
 *     simplify
 *
 * bounds.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing only establishes syntax.
 *
 * Semantic analysis determines whether a bound means:
 *
 *     trait satisfaction
 *     interface satisfaction
 *     subtype relation
 *     type predicate
 *     associated-type requirement
 *     structural property
 *     nominal property
 *     other future type-system relation
 *
 * This file does not choose among those interpretations.
 *
 * ============================================================================
 * GENERIC INTEGRATION
 * ============================================================================
 *
 * Generic declarations own:
 *
 *     genericParameter
 *     genericParameterList
 *
 * They may consume:
 *
 *     typeConstraintClause
 *
 * for compatibility.
 *
 * Canonical new consumers should prefer:
 *
 *     typeBoundClause
 *
 * supplied by TypeBounds.
 *
 * Conceptual composition:
 *
 *     genericParameter
 *         : identifier
 *           typeConstraintClause?
 *         ;
 *
 * or, canonically:
 *
 *     genericParameter
 *         : identifier
 *           typeBoundClause?
 *         ;
 *
 * The generic declaration grammar remains responsible for the parameter
 * itself.
 *
 * ============================================================================
 * WHERE-CLAUSE BOUNDARY
 * ============================================================================
 *
 * This file deliberately DOES NOT own:
 *
 *     WHERE
 *     whereClause
 *     wherePredicate
 *
 * A declaration/function grammar that owns a where clause may compose:
 *
 *     typeExpression typeBoundClause
 *
 * or the compatibility:
 *
 *     typeExpression typeConstraintClause
 *
 * as appropriate.
 *
 * This prevents `WHERE` placement from becoming duplicated across:
 *
 *     functions
 *     declarations
 *     types
 *     core constraints
 *
 * ============================================================================
 * GENERAL CONSTRAINT BOUNDARY
 * ============================================================================
 *
 * Type bounds are not the general constraint language.
 *
 * These remain separate:
 *
 *     T: Numeric
 *
 * versus:
 *
 *     requires memory >= required_memory
 *
 * versus:
 *
 *     requires capability("tensor.compute")
 *
 * versus:
 *
 *     where resource::memory >= required_memory
 *
 * versus:
 *
 *     requires topology(required_topology)
 *
 * TypeBounds represents the type-level form.
 *
 * General constraint semantics belong to:
 *
 *     grammar/core/constraints.g4
 *
 * Resource semantics belong to:
 *
 *     grammar/resources/
 *
 * Capability semantics belong to:
 *
 *     grammar/resources/
 *     grammar/core/
 *
 * ============================================================================
 * TYPE-SYSTEM INTEGRATION
 * ============================================================================
 *
 * Type bounds can qualify types participating in:
 *
 *     generics
 *     associated types
 *     type classes
 *     linear types
 *     affine types
 *     dependent types
 *     effect-qualified types
 *     resource-aware types
 *     capability-aware types
 *     classical types
 *     quantum types
 *     HDL types
 *     hardware types
 *     data types
 *     model types
 *     distributed types
 *
 * None of those domains receives a separate bound grammar.
 *
 * ============================================================================
 * LINEAR / AFFINE INTEGRATION
 * ============================================================================
 *
 * A bound may reference a type whose semantic rules are owned by:
 *
 *     grammar/types/linear.g4
 *     grammar/types/affine.g4
 *
 * This compatibility grammar does not enforce ownership or usage rules.
 *
 * Those are semantic type-system responsibilities.
 *
 * ============================================================================
 * DEPENDENT-TYPE INTEGRATION
 * ============================================================================
 *
 * A bound may contain a canonical dependent/value-parameterized type whenever
 * the canonical type-expression grammar permits it.
 *
 * Examples:
 *
 *     T: Matrix<Element, Rows, Columns>
 *
 *     T: Tensor<Element, Shape>
 *
 *     T: Register<Qubit, Count>
 *
 * No machine-sized integer conversion is performed here.
 *
 * Symbolic values remain symbolic until semantic analysis.
 *
 * ============================================================================
 * ASSOCIATED-TYPE INTEGRATION
 * ============================================================================
 *
 * Associated types remain part of the canonical type-expression system.
 *
 * This grammar does not resolve:
 *
 *     associated type declarations
 *     projections
 *     equality constraints
 *     normalization
 *
 * Those belong to the semantic type system.
 *
 * ============================================================================
 * TYPE-CLASS / TRAIT INTEGRATION
 * ============================================================================
 *
 * A bound such as:
 *
 *     T: Numeric
 *
 * is represented as a canonical type expression.
 *
 * This grammar does not create a closed enumeration of:
 *
 *     Numeric
 *     Comparable
 *     Iterable
 *     Serializable
 *     QuantumState
 *     Accelerator
 *     Tensor
 *     Dataset
 *
 * New type-level concepts therefore do not require modifying this file.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum types are ordinary canonical type expressions.
 *
 * Examples:
 *
 *     T: quantum::State
 *
 *     T: quantum::Observable
 *
 *     T: quantum::Circuit
 *
 *     T: quantum::LogicalQubit
 *
 * This file does NOT define:
 *
 *     gates
 *     physical qubits
 *     topology
 *     coupling maps
 *     calibration
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *
 * The semantic pipeline remains:
 *
 *     source
 *       ->
 *     AST
 *       ->
 *     semantic type model
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     decomposition
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     resilience / QEC
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *
 * ============================================================================
 * CLASSICAL / DATA / AI INTEGRATION
 * ============================================================================
 *
 * Bounds may reference:
 *
 *     Numeric
 *     Tensor<T>
 *     Model<T>
 *     Dataset<T>
 *     Evidence<T>
 *     Distribution<T>
 *     Knowledge<T>
 *
 * without this grammar acquiring domain-specific alternatives.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Bounds may reference:
 *
 *     hdl::Module
 *     hardware::Signal
 *     hardware::Memory
 *     hardware::Accelerator
 *
 * without selecting a physical implementation.
 *
 * ============================================================================
 * DISTRIBUTED / NETWORKING INTEGRATION
 * ============================================================================
 *
 * Bounds may reference:
 *
 *     distributed::Message
 *     distributed::Actor
 *     networking::Endpoint
 *     networking::Stream
 *
 * No node count, topology size, or device count is encoded.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * This file does not define effects.
 *
 * If the canonical type system supports effect-qualified types, their syntax
 * remains owned by the effect/type grammar.
 *
 * Effect checking occurs downstream.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * A type bound does not allocate resources.
 *
 * For example:
 *
 *     T: quantum::State
 *
 * is not a physical qubit allocation.
 *
 * Resource intent remains separate:
 *
 *     requires qubits >= required_qubits;
 *
 * Capability intent remains separate:
 *
 *     requires capability("quantum.measurement");
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * These constructs are NOT defined here:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Contract syntax belongs to validation/contracts and its surrounding
 * declaration/expression owners.
 *
 * A semantic contract checker may consume type information, but it does not
 * change this grammar's ownership.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * These constructs are NOT type constraints:
 *
 *     allow
 *     forbid
 *     prefer
 *     fallback
 *     constrain
 *
 * Policy syntax and semantics remain under:
 *
 *     grammar/policies/
 *
 * ============================================================================
 * PROVENANCE
 * ============================================================================
 *
 * The parser must preserve source locations for compatibility aliases.
 *
 * Semantic provenance may record:
 *
 *     source constraint
 *     bound source span
 *     resolved type
 *     normalization
 *     diagnostic
 *     specialization decision
 *
 * Provenance recording itself is downstream.
 *
 * ============================================================================
 * SCALABILITY / POCO-REAF
 * ============================================================================
 *
 * This file introduces NO language-level finite capacity.
 *
 * In particular, it introduces no limits for:
 *
 *     qubits
 *     CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     nodes
 *     memory
 *     threads
 *     tensor rank
 *     register width
 *     network size
 *     devices
 *     generic arity
 *     bound count
 *     qualification depth
 *     type nesting depth
 *
 * The canonical grammar uses unbounded repetition where the language
 * structure requires it.
 *
 * Practical limits caused by:
 *
 *     memory
 *     compiler resources
 *     parser resources
 *     operating-system limits
 *     runtime resources
 *
 * are implementation/resource concerns rather than language semantics.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * These aliases are purely syntactic.
 *
 * They contain:
 *
 *     no actions
 *     no semantic predicates
 *     no I/O
 *     no randomness
 *     no hardware inspection
 *     no environment inspection
 *     no runtime execution
 *
 * Therefore the same token sequence under the same language configuration
 * produces the same parse structure.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust
 *     no unsafe Rust
 *     no filesystem access
 *     no network access
 *     no process execution
 *     no environment access
 *     no hardware probing
 *
 * Generated Rust must remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust
 *
 * ============================================================================
 * COMPATIBILITY POLICY
 * ============================================================================
 *
 * The compatibility aliases are retained so existing parser consumers can
 * migrate without requiring a second implementation of type bounds.
 *
 * New grammar code SHOULD use:
 *
 *     typeBoundClause
 *     typeBoundList
 *     typeBound
 *
 * directly when possible.
 *
 * Existing code MAY continue using:
 *
 *     typeConstraintClause
 *     typeConstraintBoundList
 *     typeConstraintBound
 *
 * during the migration period.
 *
 * These aliases must remain structurally equivalent.
 *
 * ============================================================================
 * DEPRECATION RULE
 * ============================================================================
 *
 * `TypeConstraints` is a compatibility surface, not the long-term canonical
 * owner.
 *
 * Once all consumers have migrated to TypeBounds, this file can be reduced to
 * a compatibility marker or retired according to the repository's normal
 * compatibility policy.
 *
 * It MUST NOT evolve into another independent constraint implementation.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive compatibility tests:
 *
 *     : Numeric
 *
 *     : Numeric + Comparable
 *
 *     : quantum::State
 *
 *     : quantum::State + quantum::Measurable
 *
 *     : collections::Iterable<Value>
 *
 *     : hardware::Accelerator<Model>
 *
 *     : future::domain::Capability
 *
 * Canonical generic integration:
 *
 *     T: Numeric
 *
 *     T: Numeric + Comparable
 *
 * Nested type expressions:
 *
 *     T: collections::Iterable<collections::Iterable<U>>
 *
 * Dependent/value-parameterized forms:
 *
 *     T: Matrix<Element, Rows, Columns>
 *
 * Semantic-unresolved forms MUST parse:
 *
 *     T: UnknownType
 *
 *     T: future::UnknownType
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The following must be rejected by the canonical TypeBounds implementation:
 *
 *     :
 *
 *     : +
 *
 *     : A +
 *
 *     : + A
 *
 *     : A + +
 *
 *     : A + B +
 *
 * This facade must not weaken those structural guarantees.
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test:
 *
 *     one bound
 *     many bounds
 *     deeply nested type expressions
 *     deeply qualified types
 *     nested generic applications
 *     dependent types
 *     associated types
 *     linear types
 *     affine types
 *     quantum types
 *     HDL types
 *     hardware types
 *     data/model types
 *     distributed types
 *     future qualified types
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Test increasing:
 *
 *     bound count
 *     type-expression depth
 *     generic arity
 *     qualification depth
 *     source size
 *
 * until practical implementation resources are reached.
 *
 * The test suite MUST NOT convert observed implementation limits into
 * language-level constants.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/types/bounds.g4
 *     grammar/antlr/ZamaniLexer.g4
 *
 * EXPORTS:
 *
 *     typeConstraintClause
 *     typeConstraintBoundList
 *     typeConstraintBound
 *
 * CONSUMED_BY:
 *
 *     legacy generic/declaration grammars
 *     compatibility parser compositions
 *
 * AST_OWNER:
 *
 *     existing domain-neutral TypeExpr / type-bound semantic model
 *
 * SEMANTIC_OWNER:
 *
 *     canonical semantic type/constraint system
 *
 * IR_OWNER:
 *
 *     canonical semantic model and downstream domain IRs
 *
 * TEST_OWNER:
 *
 *     grammar/tests/types/
 *     grammar/tests/compatibility/
 *
 * SPEC_OWNER:
 *
 *     grammar/specification/
 *     grammar/spec/
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * DONE means:
 *
 *     [ ] No duplicate type-bound implementation remains here.
 *
 *     [ ] TypeBounds is the sole syntax owner.
 *
 *     [ ] TypeConstraints is a compatibility facade.
 *
 *     [ ] Canonical ZamaniLexer vocabulary is used.
 *
 *     [ ] No lexer rules are declared.
 *
 *     [ ] No type-expression grammar is duplicated.
 *
 *     [ ] No WHERE grammar is duplicated.
 *
 *     [ ] No general constraint grammar is duplicated.
 *
 *     [ ] No resource grammar is duplicated.
 *
 *     [ ] No capability negotiation is implemented.
 *
 *     [ ] No contract/policy grammar is duplicated.
 *
 *     [ ] No domain-specific type catalogue exists here.
 *
 *     [ ] No physical hardware limits exist.
 *
 *     [ ] No quantum limits exist.
 *
 *     [ ] No generic-arity limits exist.
 *
 *     [ ] No bound-count limits exist.
 *
 *     [ ] Existing compatibility rule names remain available.
 *
 *     [ ] Canonical TypeBounds rules are used by new code.
 *
 *     [ ] Source ordering is preserved.
 *
 *     [ ] Unresolved names remain parseable.
 *
 *     [ ] Rust 1.97+ frontend integration remains safe.
 *
 *     [ ] ANTLR composition succeeds.
 *
 *     [ ] Positive tests pass.
 *
 *     [ ] Negative tests pass.
 *
 *     [ ] Boundary tests pass.
 *
 *     [ ] Scalability tests pass.
 *
 *     [ ] Determinism tests pass.
 *
 * ============================================================================
 * FINAL RULE
 * ============================================================================
 *
 * This file exists to prevent a breaking change from forcing the repository
 * to maintain two type-bound implementations.
 *
 * Canonical syntax:
 *
 *     TypeBounds
 *
 * Compatibility surface:
 *
 *     TypeConstraints
 *
 * Semantic meaning:
 *
 *     canonical type system
 *
 * Resource/capability meaning:
 *
 *     resource and capability systems
 *
 * Target realization:
 *
 *     compiler/backend/HAL
 *
 * This separation preserves the universal, target-independent type system
 * required for scalable Zamani programs.
 *
 * ============================================================================
 */

parser grammar TypeConstraints;

options {
    tokenVocab = ZamaniLexer;
}

import TypeBounds;


/*
 * ============================================================================
 * COMPATIBILITY: TYPE CONSTRAINT CLAUSE
 * ============================================================================
 *
 * Historical/current consumers can continue using:
 *
 *     typeConstraintClause
 *
 * The actual implementation is:
 *
 *     TypeBounds.typeBoundClause
 *
 * No new syntax is introduced here.
 */
typeConstraintClause
    : typeBoundClause
    ;


/*
 * ============================================================================
 * COMPATIBILITY: ORDERED TYPE CONSTRAINT BOUNDS
 * ============================================================================
 *
 * Historical/current consumers can continue using:
 *
 *     typeConstraintBoundList
 *
 * The actual implementation is:
 *
 *     TypeBounds.typeBoundList
 */
typeConstraintBoundList
    : typeBoundList
    ;


/*
 * ============================================================================
 * COMPATIBILITY: SINGLE TYPE CONSTRAINT BOUND
 * ============================================================================
 *
 * Historical/current consumers can continue using:
 *
 *     typeConstraintBound
 *
 * The actual implementation is:
 *
 *     TypeBounds.typeBound
 *
 * The payload remains the canonical:
 *
 *     typeExpression
 */
typeConstraintBound
    : typeBound
    ;