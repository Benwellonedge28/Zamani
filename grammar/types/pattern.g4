/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/types/pattern.g4
 *
 * GRAMMAR
 * -------
 * PatternTypes
 *
 * STATUS
 * ------
 * PRODUCTION TYPE-SYSTEM DELEGATE
 *
 * PURPOSE
 * -------
 * This grammar owns the type-level boundary for pattern-constrained types.
 *
 * It does NOT own the universal value-pattern language.
 *
 * The universal pattern language is owned exclusively by:
 *
 *     grammar/expressions/patterns.g4
 *
 * This file therefore provides the reusable type-system construct:
 *
 *     patternTypeConstraint
 *
 * whose semantic meaning is:
 *
 *     an already parsed type expression constrained by a canonical
 *     source-level pattern.
 *
 * Conceptually:
 *
 *     T where P
 *
 * means:
 *
 *     a value inhabiting T and satisfying the semantic constraint P
 *
 * where P is interpreted by the type/refinement semantic subsystem.
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * Rust:
 *
 *     Rust 1.97 or later
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * Parser:
 *
 *     ANTLR4 parser grammar
 *
 * This file contains:
 *
 *     no embedded Rust;
 *     no semantic actions;
 *     no semantic predicates;
 *     no I/O;
 *     no runtime execution;
 *     no target discovery;
 *     no hardware discovery.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *        |
 *        v
 *     canonical lexer
 *        |
 *        v
 *     parser
 *        |
 *        +-----------------------------+
 *        |                             |
 *        v                             v
 *   typeExpression              value pattern
 *        |                             |
 *        |                             |
 *        +-------------+---------------+
 *                      |
 *                      v
 *              type-pattern constraint
 *                      |
 *                      v
 *              domain-neutral AST
 *                      |
 *                      v
 *              structural validation
 *                      |
 *                      v
 *              semantic type analysis
 *                      |
 *          +-----------+-----------+
 *          |           |           |
 *          v           v           v
 *       classical   quantum::ir   HDL/hardware
 *          |           |           |
 *          +-----------+-----------+
 *                      |
 *                      v
 *              canonical semantic model
 *                      |
 *                      v
 *                target-independent
 *                    compilation
 *                      |
 *               optimization
 *                      |
 *                  lowering
 *                      |
 *             routing/scheduling
 *                      |
 *              resilience/QEC
 *                      |
 *                     ZQN
 *                      |
 *                     HAL
 *                      |
 *              target realization
 *
 * This grammar exists entirely before target realization.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * EXACTLY ONE grammar owns the universal source-level `pattern` rule:
 *
 *     grammar/expressions/patterns.g4
 *
 * This file MUST NOT define:
 *
 *     pattern
 *     patternAtom
 *     wildcardPattern
 *     bindingPattern
 *     literalPattern
 *     tuplePattern
 *     sequencePattern
 *     sequencePatternElement
 *     restPattern
 *     structPattern
 *     structPatternField
 *     variantPattern
 *     rangePattern
 *     rangePatternEndpoint
 *     orPattern
 *     patternAlternative
 *     referencePattern
 *     typePattern
 *     parenthesizedPattern
 *     patternList
 *
 * Those rules belong to the canonical Patterns grammar.
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * A value pattern and a type-constrained type are related but are not the
 * same language construct.
 *
 * VALUE PATTERN
 * ------------
 *
 *     x
 *     _
 *     0
 *     (a, b)
 *     Some(value)
 *     0 .. limit
 *     A | B
 *
 * A value pattern describes a structural/value-level matching condition.
 *
 * TYPE PATTERN CONSTRAINT
 * -----------------------
 *
 *     Integer where 0 .. limit
 *     Point where Point { x: _, y: _ }
 *
 * A type pattern constraint attaches a canonical value pattern to an already
 * established type position.
 *
 * The semantic layer determines whether the pattern is a valid constraint
 * for that type.
 *
 * This separation prevents:
 *
 *     value pattern syntax
 *
 * from becoming:
 *
 *     type-system syntax
 *
 * and prevents two independent pattern languages from developing.
 *
 * ============================================================================
 * OWNERSHIP CONTRACT
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     patternTypeConstraint
 *
 * THIS FILE DOES NOT OWN:
 *
 *     typeExpression
 *     typeCore
 *     typePostfix
 *     pattern
 *     patternAtom
 *     guards
 *     match expressions
 *     match statements
 *     refinement proving
 *     constraint solving
 *     type inference
 *     type unification
 *     ownership
 *     borrowing
 *     lifetimes
 *     effects
 *     capabilities
 *     resources
 *     contracts
 *     policies
 *     provenance
 *     IR
 *     optimization
 *     lowering
 *     routing
 *     scheduling
 *     quantum realization
 *     HDL realization
 *     hardware realization
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/expressions/patterns.g4
 *
 * Consumed canonical rule:
 *
 *     pattern
 *
 * Required lexical token:
 *
 *     WHERE
 *
 * EXPORTS:
 *
 *     patternTypeConstraint
 *
 * CONSUMED_BY:
 *
 *     grammar/types/types.g4
 *     future type-system composition grammars
 *
 * AST_OWNER:
 *
 *     existing canonical frontend type-expression / refinement AST
 *
 * SEMANTIC_OWNER:
 *
 *     semantic type checker
 *     refinement checker
 *     constraint solver
 *
 * TYPE_OWNER:
 *
 *     canonical type system
 *
 * EFFECT_OWNER:
 *
 *     grammar/effects/ and semantic effect analysis
 *
 * RESOURCE_OWNER:
 *
 *     grammar/resources/ and semantic resource analysis
 *
 * POLICY_OWNER:
 *
 *     grammar/policies/ and semantic policy analysis
 *
 * PROVENANCE_OWNER:
 *
 *     canonical provenance subsystem
 *
 * IR_OWNER:
 *
 *     canonical semantic model
 *     applicable domain IR
 *     quantum::ir for quantum-derived semantics
 *
 * SPEC_OWNER:
 *
 *     grammar/specification/types.md
 *     applicable refinement/type specifications
 *
 * TEST_OWNER:
 *
 *     grammar/tests/types/
 *     grammar/tests/semantic/
 *     frontend conformance tests
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * No lexer rules are declared here.
 *
 * The `where` keyword MUST come from the canonical lexer vocabulary.
 *
 * This grammar consumes:
 *
 *     WHERE
 *
 * It MUST NOT introduce a second spelling or parser-local keyword.
 *
 * The canonical lexical source remains:
 *
 *     grammar/lexer/tokens.g4
 *     grammar/antlr/ZamaniLexer.g4
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a parser delegate.
 *
 * It imports the canonical Patterns grammar.
 *
 * It deliberately does NOT import the complete Types grammar.
 *
 * That direction is mandatory.
 *
 * Correct dependency:
 *
 *     PatternTypes
 *          |
 *          +--> Patterns
 *
 * and:
 *
 *     Type
 *          |
 *          +--> PatternTypes
 *
 * Incorrect dependency:
 *
 *     Type
 *       |
 *       v
 *     PatternTypes
 *       |
 *       v
 *     Type
 *
 * The latter creates a circular grammar dependency.
 *
 * Therefore this file never references `typeExpression`.
 *
 * The enclosing type orchestrator attaches `patternTypeConstraint` to its
 * type-expression composition.
 *
 * ============================================================================
 * PUBLIC RULE
 * ============================================================================
 *
 * There is exactly one public rule:
 *
 *     patternTypeConstraint
 *
 * It represents:
 *
 *     WHERE pattern
 *
 * The preceding type expression is owned by the caller.
 *
 * ============================================================================
 * SYNTAX
 * ============================================================================
 *
 * Canonical conceptual form:
 *
 *     typeExpression where pattern
 *
 * Examples:
 *
 *     Integer where 0
 *
 *     Integer where 0 | 1 | 2
 *
 *     Integer where 0 .. limit
 *
 *     Point where Point { x: _, y: _ }
 *
 *     Result<T, E> where Ok(value)
 *
 *     Tensor<T> where _
 *
 *     QuantumState<T> where _
 *
 * The final examples are intentionally semantically permissive at the
 * grammar level. Semantic analysis determines whether the pattern is
 * meaningful for the type.
 *
 * ============================================================================
 * IMPORTANT: OPEN-WORLD TYPE DESIGN
 * ============================================================================
 *
 * This grammar deliberately does not enumerate:
 *
 *     integer widths
 *     tensor dimensions
 *     tensor ranks
 *     quantum sizes
 *     hardware types
 *     accelerator types
 *     CPU models
 *     GPU models
 *     FPGA families
 *     QPU models
 *     node counts
 *     memory capacities
 *     network sizes
 *     vendor identifiers
 *     AI model families
 *
 * Such information belongs to source types, generic parameters, semantic
 * capabilities, resources, dialects or target descriptions.
 *
 * A pattern constraint is therefore independent of physical realization.
 *
 * ============================================================================
 * PATTERN DELEGATION
 * ============================================================================
 *
 * The complete pattern after `where` is delegated to:
 *
 *     grammar/expressions/patterns.g4
 *
 * This gives type constraints access to the same pattern language used by:
 *
 *     match expressions
 *     match statements
 *     guards where applicable
 *     future pattern-consuming constructs
 *
 * Consequently:
 *
 *     pattern syntax
 *
 * remains one language-wide facility.
 *
 * ============================================================================
 * PATTERN PRECEDENCE
 * ============================================================================
 *
 * The canonical `pattern` rule already owns pattern precedence and
 * alternative composition.
 *
 * This file therefore does not add:
 *
 *     |
 *     &
 *     ..
 *     ..=
 *     parentheses
 *     tuple syntax
 *     structure syntax
 *     variant syntax
 *
 * around the delegated pattern.
 *
 * The complete pattern is consumed exactly once:
 *
 *     WHERE pattern
 *
 * ============================================================================
 * DELIMITER CONTRACT
 * ============================================================================
 *
 * `where` is the explicit boundary between:
 *
 *     type syntax
 *
 * and:
 *
 *     pattern syntax
 *
 * This is preferable to a bare:
 *
 *     Type { ... }
 *
 * boundary because braces already participate in:
 *
 *     struct patterns
 *     blocks
 *     record syntax
 *     other domain constructs
 *
 * The explicit `WHERE` boundary is therefore both readable and structurally
 * unambiguous.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates no AST.
 *
 * The parser tree represents:
 *
 *     patternTypeConstraint
 *         |
 *         +--> WHERE
 *         |
 *         +--> pattern
 *
 * The enclosing type AST associates the constraint with the preceding
 * TypeExpr.
 *
 * The canonical semantic shape is conceptually:
 *
 *     TypeConstraint
 *       baseType
 *       pattern
 *       sourceSpan
 *
 * This is a semantic description, not an instruction to introduce a second
 * AST hierarchy.
 *
 * Existing frontend AST infrastructure remains authoritative.
 *
 * The source span of the `where` clause and delegated pattern MUST remain
 * available to diagnostics and provenance.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing does not establish that a pattern is a valid refinement.
 *
 * Semantic analysis MUST determine:
 *
 *     1. the type of the preceding type expression;
 *
 *     2. the semantic identity of the pattern;
 *
 *     3. whether the pattern can constrain that type;
 *
 *     4. whether all pattern literals have compatible types;
 *
 *     5. whether bindings are legal;
 *
 *     6. whether repeated bindings satisfy the language's binding rules;
 *
 *     7. whether tuple/sequence/record/variant structure is compatible;
 *
 *     8. whether range endpoints are valid for the constrained type;
 *
 *     9. whether a reference pattern is legal;
 *
 *     10. whether an OR-pattern has compatible alternatives;
 *
 *     11. whether the refinement is satisfiable where proof is required;
 *
 *     12. whether compile-time evaluation is required and permitted;
 *
 *     13. whether runtime checking is required;
 *
 *     14. whether the constraint is decidable under the selected policy;
 *
 *     15. whether the constraint interacts with ownership/linearity;
 *
 *     16. whether the constraint introduces effects;
 *
 *     17. whether the constraint requires capabilities;
 *
 *     18. whether the constraint imposes resource requirements;
 *
 *     19. whether contracts apply;
 *
 *     20. whether policies permit its evaluation;
 *
 *     21. whether provenance must be recorded.
 *
 * A failed semantic proof MUST NOT silently become acceptance.
 *
 * An unavailable target capability MUST NOT be converted into a type error
 * unless the language specification explicitly defines that dependency as
 * part of the type's semantic validity.
 *
 * ============================================================================
 * REFINEMENT CONTRACT
 * ============================================================================
 *
 * A pattern-constrained type is a refinement-like type relationship.
 *
 * It must remain distinguishable from:
 *
 *     ordinary type constraints;
 *     generic bounds;
 *     resource requirements;
 *     capability requirements;
 *     contracts;
 *     policies.
 *
 * These systems may interact but are not interchangeable.
 *
 * Example conceptual distinction:
 *
 *     T: Numeric
 *
 * is a type bound.
 *
 *     T where 0 .. N
 *
 * is a value/pattern refinement.
 *
 *     requires capability("tensor.compute")
 *
 * is a capability requirement.
 *
 *     requires memory >= required_memory
 *
 * is a resource requirement.
 *
 *     requires condition
 *
 * is a contract/requirement construct.
 *
 * The grammar must preserve these distinctions.
 *
 * ============================================================================
 * TYPE SYSTEM INTEGRATION
 * ============================================================================
 *
 * `patternTypeConstraint` is attached to a complete type expression by the
 * canonical type orchestrator.
 *
 * Conceptually:
 *
 *     typeExpression
 *         : typePrefix*
 *           typeCore
 *           typePostfix*
 *           ;
 *
 * and the type postfix composition includes:
 *
 *     patternTypeConstraint
 *
 * Therefore the type system can represent:
 *
 *     BaseType where Pattern
 *
 * without this file needing to know how BaseType itself is constructed.
 *
 * ============================================================================
 * GENERICS
 * ============================================================================
 *
 * Generic types remain owned by the type system.
 *
 * Examples:
 *
 *     Vector<T> where _
 *
 *     Tensor<T, Shape> where _
 *
 *     Result<T, E> where Ok(value)
 *
 * Semantic analysis determines:
 *
 *     generic substitution;
 *     bound satisfaction;
 *     pattern/type compatibility;
 *     dependent-value relationships.
 *
 * This grammar imposes no generic arity ceiling.
 *
 * ============================================================================
 * DEPENDENT TYPES
 * ============================================================================
 *
 * Pattern constraints may refer to symbolic values already represented by
 * the source program.
 *
 * Example:
 *
 *     Array<T, N> where ...
 *
 * The grammar does not evaluate N.
 *
 * The semantic layer determines:
 *
 *     type;
 *     const-ness;
 *     value domain;
 *     satisfiability;
 *     representability;
 *     specialization requirements.
 *
 * No host-sized integer is required by this grammar.
 *
 * ============================================================================
 * LINEAR / AFFINE TYPES
 * ============================================================================
 *
 * A pattern constraint MUST NOT alter ownership semantics.
 *
 * In particular, pattern matching does not automatically imply:
 *
 *     copy;
 *     clone;
 *     duplication;
 *     aliasing;
 *     movement;
 *     borrowing.
 *
 * Ownership, affine and linear semantics remain owned by the type/semantic
 * systems.
 *
 * A pattern cannot make a non-copyable value copyable merely by matching it.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * The presence of this grammar construct introduces no effect by itself.
 *
 * However, semantic evaluation of a pattern constraint may involve an effect
 * depending on the expression/pattern and language policy.
 *
 * Potential semantic effects include:
 *
 *     io
 *     network
 *     randomness
 *     native
 *     foreign
 *     distributed
 *     measurement
 *     quantum
 *     learning
 *     adaptation
 *     reflection
 *     code_generation
 *     simulation
 *
 * Effects MUST be determined downstream.
 *
 * This grammar must never encode an effect by implication.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Pattern constraints do not directly select or inspect a target capability.
 *
 * A semantic constraint may depend on a capability-aware operation.
 *
 * Capability resolution remains downstream.
 *
 * Examples of capabilities that might participate semantically include:
 *
 *     tensor.compute
 *     quantum.measurement
 *     distributed.compute
 *     native.execute
 *
 * The grammar never probes whether such a capability exists.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * No resource limits are defined here.
 *
 * In particular, this grammar contains no universal limits on:
 *
 *     pattern count
 *     pattern depth
 *     alternatives
 *     tuple arity
 *     sequence length
 *     type nesting
 *     generic arity
 *     tensor rank
 *     tensor dimensions
 *     qubits
 *     CPUs
 *     GPUs
 *     FPGAs
 *     accelerators
 *     nodes
 *     memory
 *     storage
 *     devices
 *     network topology
 *
 * There are no parser-level constants representing those limits.
 *
 * Practical compiler limits, if necessary, belong to resource policy and
 * must produce explicit diagnostics rather than changing language semantics.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Pattern-constrained types may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Contract syntax is not defined here.
 *
 * Contract semantics may reason about the constrained type.
 *
 * A contract MUST NOT cause a target-specific reinterpretation of the type.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Policies may govern:
 *
 *     refinement checking;
 *     runtime validation;
 *     proof requirements;
 *     reflection;
 *     dynamic evaluation;
 *     resource consumption.
 *
 * Policy syntax remains owned by the policy subsystem.
 *
 * This file provides no policy bypass.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The parser must preserve enough structure for the frontend to associate:
 *
 *     type source span
 *     where source span
 *     pattern source span
 *
 * with the resulting semantic type.
 *
 * Provenance may subsequently record:
 *
 *     source;
 *     derived_from;
 *     generated_by;
 *     transformed_by;
 *     verified_by;
 *     reason;
 *     evidence;
 *     decision;
 *     version.
 *
 * Provenance storage and semantics remain downstream.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum types may be constrained by patterns when their semantic value model
 * permits such a constraint.
 *
 * This grammar does NOT introduce:
 *
 *     physical qubit patterns;
 *     physical qubit identifiers;
 *     coupling maps;
 *     calibration;
 *     pulse schedules;
 *     routing;
 *     QEC selection;
 *     hardware topology.
 *
 * The downstream quantum path remains:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic quantum model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     decomposition
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     resilience / QEC
 *       |
 *       v
 *     ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target
 *
 * This grammar participates only before the `quantum::ir` boundary.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * HDL and hardware semantic values may use the same type-pattern mechanism.
 *
 * The grammar does not encode:
 *
 *     fixed bus widths;
 *     fixed register widths;
 *     fixed device counts;
 *     fixed topology sizes;
 *     physical addresses;
 *     vendor-specific hardware identifiers.
 *
 * Those are target or semantic properties, not universal grammar limits.
 *
 * ============================================================================
 * CLASSICAL / DATA / AI INTEGRATION
 * ============================================================================
 *
 * The same construct can constrain:
 *
 *     scalar values;
 *     tuples;
 *     records;
 *     variants;
 *     collections;
 *     tensors;
 *     datasets;
 *     probabilistic values;
 *     knowledge values;
 *     inference results;
 *     learning results;
 *     distributed values;
 *     resource descriptions;
 *     capability descriptions.
 *
 * No domain-specific pattern-type grammar is required.
 *
 * ============================================================================
 * MATCH INTEGRATION
 * ============================================================================
 *
 * Match constructs consume the same canonical `pattern` rule.
 *
 * This file does not define:
 *
 *     matchStatement
 *     matchExpression
 *     matchArm
 *     guardClause
 *
 * A pattern appearing after `where` is therefore structurally identical to a
 * pattern appearing in a match arm.
 *
 * The semantic context differs:
 *
 *     match context
 *         -> scrutinee matching
 *
 *     type context
 *         -> type refinement
 *
 * Semantic analysis owns that distinction.
 *
 * ============================================================================
 * GUARD INTEGRATION
 * ============================================================================
 *
 * A type-level pattern constraint does not own guards.
 *
 * If a future refinement feature requires predicates in addition to patterns,
 * that predicate syntax must be owned by the canonical refinement/constraint
 * subsystem.
 *
 * Do not add a second guard language here.
 *
 * ============================================================================
 * INTEROPERABILITY
 * ============================================================================
 *
 * Foreign values may be constrained only after their semantic type has crossed
 * the FFI/ABI boundary.
 *
 * This file does not define:
 *
 *     ABI;
 *     layout;
 *     calling convention;
 *     marshaling;
 *     pointer representation.
 *
 * Those belong to interoperability.
 *
 * ============================================================================
 * METAPROGRAMMING
 * ============================================================================
 *
 * Generated type constraints must re-enter the ordinary parser and semantic
 * pipeline.
 *
 * Reflection and code generation MUST NOT bypass:
 *
 *     type checking;
 *     refinement checking;
 *     ownership;
 *     effects;
 *     capabilities;
 *     resources;
 *     contracts;
 *     policies;
 *     provenance.
 *
 * ============================================================================
 * DIALECT INTEGRATION
 * ============================================================================
 *
 * Domain dialects may define types whose values can be constrained by the
 * universal pattern system.
 *
 * A dialect does not obtain permission to redefine:
 *
 *     pattern
 *
 * or:
 *
 *     patternTypeConstraint
 *
 * as a competing universal syntax.
 *
 * Dialect-specific semantics remain outside this grammar.
 *
 * ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax errors associated with this rule include:
 *
 *     missing pattern after `where`;
 *     malformed pattern;
 *     unexpected end of input;
 *     invalid delimiter structure within the delegated pattern.
 *
 * Semantic diagnostics include:
 *
 *     pattern incompatible with base type;
 *     impossible refinement;
 *     invalid binding;
 *     invalid range endpoint;
 *     invalid structural decomposition;
 *     unsupported semantic refinement;
 *     forbidden effect;
 *     unavailable required capability;
 *     unavailable required resource;
 *     policy violation.
 *
 * The parser MUST distinguish syntax failure from semantic failure.
 *
 * A target feasibility failure MUST NOT be reported as a parser error.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar is open-ended with respect to:
 *
 *     pattern alternatives;
 *     nesting;
 *     type complexity;
 *     generic structure;
 *     symbolic values;
 *     source-program size.
 *
 * No finite language ceiling is introduced here.
 *
 * "Scale to infinity" means that the language does not artificially cap the
 * semantic domain.
 *
 * It does not claim that finite implementations possess infinite resources.
 *
 * A compiler may enforce configurable resource budgets for:
 *
 *     parsing;
 *     memory;
 *     semantic analysis;
 *     proof;
 *     compilation;
 *     execution.
 *
 * Such budgets are implementation policy, not language semantics.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Given identical:
 *
 *     source;
 *     grammar version;
 *     lexer vocabulary;
 *     parser configuration;
 *
 * this grammar must produce the same parse structure and source ordering.
 *
 * The grammar introduces no:
 *
 *     randomness;
 *     time dependence;
 *     environment inspection;
 *     target inspection;
 *     mutable parser state.
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains no executable code.
 *
 * Rust integration MUST remain:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust only
 *
 * No `unsafe` Rust is required.
 *
 * The grammar itself cannot perform:
 *
 *     filesystem access;
 *     network access;
 *     process execution;
 *     hardware discovery;
 *     device discovery.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The new construct is additive:
 *
 *     existing type syntax remains unchanged;
 *     existing value-pattern syntax remains unchanged;
 *     existing match syntax remains unchanged.
 *
 * The new boundary is introduced only by the canonical `WHERE` token.
 *
 * Existing programs containing:
 *
 *     where
 *
 * in an identifier position must follow the repository's existing reserved
 * keyword policy.
 *
 * No parser-local alias is permitted.
 *
 * ============================================================================
 * INTEGRATION WITH THE TYPE ORCHESTRATOR
 * ============================================================================
 *
 * `grammar/types/types.g4` remains the owner of:
 *
 *     typeExpression
 *     typeCore
 *     typePostfix
 *
 * It must import this grammar:
 *
 *     import PatternTypes;
 *
 * and its `typePostfix` rule must consume:
 *
 *     patternTypeConstraint
 *
 * The resulting composition is:
 *
 *     typeExpression
 *         : typePrefix*
 *           typeCore
 *           typePostfix*
 *         ;
 *
 *     typePostfix
 *         : QUESTION_MARK
 *         | patternTypeConstraint
 *         ;
 *
 * This keeps the dependency direction acyclic:
 *
 *     Type
 *       |
 *       v
 *     PatternTypes
 *       |
 *       v
 *     Patterns
 *
 * `PatternTypes` does NOT import `Type`.
 *
 * ============================================================================
 * INTEGRATION WITH VALUE PATTERNS
 * ============================================================================
 *
 * `grammar/expressions/patterns.g4` remains unchanged by this file.
 *
 * It continues to own:
 *
 *     pattern
 *     patternAtom
 *     wildcardPattern
 *     bindingPattern
 *     literalPattern
 *     tuplePattern
 *     sequencePattern
 *     structPattern
 *     variantPattern
 *     rangePattern
 *     referencePattern
 *     typePattern
 *     orPattern
 *     parenthesizedPattern
 *
 * No rule is copied here.
 *
 * ============================================================================
 * INTEGRATION WITH MATCH
 * ============================================================================
 *
 * Match grammars continue to consume:
 *
 *     pattern
 *
 * from:
 *
 *     grammar/expressions/patterns.g4
 *
 * They do not consume `patternTypeConstraint` unless a future type grammar
 * explicitly requires it.
 *
 * ============================================================================
 * INTEGRATION WITH REFINEMENT
 * ============================================================================
 *
 * The existing validation/refinement subsystem remains the semantic owner of
 * proof and validation.
 *
 * This grammar does not create:
 *
 *     refinementStatement
 *     refinementExpression
 *     refinementPattern
 *
 * solely for this feature.
 *
 * The type checker can transform the parsed structure into the repository's
 * existing refinement representation.
 *
 * ============================================================================
 * INTEGRATION WITH CONTRACTS
 * ============================================================================
 *
 * Contracts remain independent:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * may reference values whose types contain pattern constraints.
 *
 * Contract syntax is never duplicated here.
 *
 * ============================================================================
 * INTEGRATION WITH RESOURCES / CAPABILITIES
 * ============================================================================
 *
 * Pattern constraints may appear on types that have semantic relationships
 * with:
 *
 *     resources;
 *     capabilities;
 *     requirements;
 *     constraints;
 *     policies.
 *
 * Resolution occurs downstream.
 *
 * This grammar never inspects the available machine.
 *
 * ============================================================================
 * INTEGRATION WITH EFFECTS
 * ============================================================================
 *
 * Pattern syntax remains effect-neutral.
 *
 * If semantic checking of the pattern requires an effect, the effect system
 * records it.
 *
 * This keeps:
 *
 *     syntax
 *
 * separate from:
 *
 *     execution behavior.
 *
 * ============================================================================
 * INTEGRATION WITH PROVENANCE
 * ============================================================================
 *
 * The parser structure must allow the semantic layer to associate provenance
 * with:
 *
 *     the constrained type;
 *     the `where` clause;
 *     the delegated pattern;
 *     any generated/refined semantic representation.
 *
 * No provenance storage is implemented here.
 *
 * ============================================================================
 * INTEGRATION WITH CANONICAL IR
 * ============================================================================
 *
 * This grammar creates no IR.
 *
 * Semantic lowering may represent the constraint in the canonical semantic
 * model.
 *
 * Domain-specific lowering then determines whether the constraint:
 *
 *     can be statically proven;
 *     becomes a runtime check;
 *     is discharged during specialization;
 *     is represented as metadata;
 *     is rejected as unsatisfiable.
 *
 * Quantum-derived semantics continue toward:
 *
 *     quantum::ir
 *
 * No second quantum IR is introduced.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no universal numerical capacity.
 *
 * Forbidden concepts include any universal maximum for:
 *
 *     qubits;
 *     CPUs;
 *     GPUs;
 *     FPGAs;
 *     accelerators;
 *     nodes;
 *     memory;
 *     threads;
 *     register width;
 *     tensor rank;
 *     network size;
 *     device count;
 *     pattern count;
 *     pattern depth;
 *     generic arity;
 *     tuple arity.
 *
 * Numeric literals inside source patterns remain program data.
 *
 * For example:
 *
 *     Integer where 42
 *
 * contains a program value.
 *
 * The grammar MUST NOT interpret `42` as a language-wide capacity.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Required lexical tests:
 *
 *     where
 *
 * Required parser-positive tests:
 *
 *     Integer where 0
 *     Integer where 0 | 1
 *     Integer where 0 .. limit
 *     Point where Point { x: _, y: _ }
 *     Result<T, E> where Ok(value)
 *     Tensor<T> where _
 *
 * Required parser-negative tests:
 *
 *     Integer where
 *     Integer where |
 *     Integer where ,
 *     Integer where (
 *     Integer where )
 *
 * Required semantic tests:
 *
 *     compatible pattern/type;
 *     incompatible pattern/type;
 *     invalid binding;
 *     invalid range;
 *     impossible refinement;
 *     valid generic refinement;
 *     valid dependent-value refinement;
 *     ownership-sensitive refinement.
 *
 * Required cross-domain tests:
 *
 *     classical type + pattern;
 *     tensor type + pattern;
 *     data type + pattern;
 *     quantum semantic type + pattern;
 *     HDL semantic type + pattern;
 *     distributed value type + pattern.
 *
 * Required scalability tests:
 *
 *     large pattern alternatives;
 *     deeply nested patterns;
 *     large generic type structures;
 *     symbolic dependent values;
 *     large source files;
 *     no artificial language-level ceiling.
 *
 * Required determinism tests:
 *
 *     identical input produces identical parse structure;
 *     source ordering remains unchanged;
 *     delegated pattern structure is preserved.
 *
 * ============================================================================
 * TEST OWNERSHIP
 * ============================================================================
 *
 * grammar/types/pattern.g4
 *     owns parser syntax for patternTypeConstraint.
 *
 * grammar/expressions/patterns.g4
 *     owns pattern syntax tests.
 *
 * semantic type tests
 *     own compatibility/refinement semantics.
 *
 * frontend AST tests
 *     own AST representation.
 *
 * compiler/IR tests
 *     own semantic-to-IR behavior.
 *
 * backend tests
 *     own target realization.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] grammar name is PatternTypes;
 *
 *     [ ] tokenVocab is ZamaniLexer;
 *
 *     [ ] Patterns is imported;
 *
 *     [ ] `patternTypeConstraint` is the only public rule;
 *
 *     [ ] `pattern` is not redefined;
 *
 *     [ ] `typeExpression` is not redefined;
 *
 *     [ ] `typeCore` is not redefined;
 *
 *     [ ] no circular import exists;
 *
 *     [ ] WHERE is consumed from the canonical lexer;
 *
 *     [ ] the delegated pattern is the canonical pattern rule;
 *
 *     [ ] no semantic actions exist;
 *
 *     [ ] no semantic predicates exist;
 *
 *     [ ] no Rust is embedded;
 *
 *     [ ] no unsafe implementation is required;
 *
 *     [ ] no hardware assumptions exist;
 *
 *     [ ] no resource ceilings exist;
 *
 *     [ ] no domain-specific keyword catalogue is introduced;
 *
 *     [ ] AST ownership is downstream;
 *
 *     [ ] semantic ownership is downstream;
 *
 *     [ ] refinement ownership is downstream;
 *
 *     [ ] effects are downstream;
 *
 *     [ ] capabilities are downstream;
 *
 *     [ ] resources are downstream;
 *
 *     [ ] contracts are downstream;
 *
 *     [ ] policies are downstream;
 *
 *     [ ] provenance is preserved;
 *
 *     [ ] canonical IR ownership is preserved;
 *
 *     [ ] quantum semantics continue toward quantum::ir;
 *
 *     [ ] positive parser tests pass;
 *
 *     [ ] negative parser tests pass;
 *
 *     [ ] semantic tests pass;
 *
 *     [ ] boundary tests pass;
 *
 *     [ ] scalability tests pass;
 *
 *     [ ] determinism tests pass;
 *
 *     [ ] compatibility tests pass;
 *
 *     [ ] Rust 1.97+ frontend integration passes.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * ONE TYPE SYSTEM
 * ONE VALUE-PATTERN LANGUAGE
 * ONE PATTERN AUTHORITY
 * ONE DOMAIN-NEUTRAL AST
 * ONE SEMANTIC TYPE MODEL
 * MANY DOMAINS
 * MANY TARGETS
 * NO ARTIFICIAL CAPACITY CEILING
 *
 * ============================================================================
 */

parser grammar PatternTypes;

options {
    tokenVocab = ZamaniLexer;
}

import Patterns;


/*
 * ============================================================================
 * TYPE-LEVEL PATTERN CONSTRAINT
 * ============================================================================
 *
 * This is intentionally a narrow delegate.
 *
 * The preceding type expression is owned by the enclosing type grammar.
 *
 * This rule owns only:
 *
 *     where pattern
 *
 * Therefore there is no recursive dependency from this grammar back into
 * Type/types.g4.
 *
 * Examples:
 *
 *     Integer where 0
 *
 *     Integer where 0 | 1
 *
 *     Point where Point { x: _, y: _ }
 *
 *     Result<T, E> where Ok(value)
 */
patternTypeConstraint
    : WHERE pattern
    ;