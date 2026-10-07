/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/pattern.g4
 *
 * Grammar:
 *     PatternTypes
 *
 * Status:
 *     PRODUCTION TYPE-SYSTEM COMPONENT
 *
 * Purpose:
 *     Define the source-level type-system boundary for pattern-constrained
 *     types without defining or duplicating the universal value-pattern
 *     language.
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * Rust:
 *     Rust 1.97 or later
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * Parser:
 *     ANTLR4 parser grammar
 *
 * ============================================================================
 * IMPORTANT ARCHITECTURAL DISTINCTION
 * ============================================================================
 *
 * Zamani has two related but deliberately separate concepts:
 *
 *     1. VALUE PATTERNS
 *
 *        Owned by:
 *
 *            grammar/expressions/patterns.g4
 *
 *        This includes:
 *
 *            pattern
 *            wildcardPattern
 *            bindingPattern
 *            literalPattern
 *            tuplePattern
 *            sequencePattern
 *            structPattern
 *            variantPattern
 *            rangePattern
 *            referencePattern
 *            typePattern
 *            parenthesizedPattern
 *            orPattern
 *
 *
 *     2. PATTERN-CONSTRAINED TYPES
 *
 *        Owned by this file.
 *
 *        These are type-system constructs whose semantic identity depends on
 *        a pattern, predicate, refinement, or value constraint.
 *
 * This file MUST NOT redefine the value-pattern grammar.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * The universal source-level `pattern` rule belongs exclusively to:
 *
 *     grammar/expressions/patterns.g4
 *
 * Match expressions/statements consume that rule.
 *
 * This file MUST NOT define:
 *
 *     pattern
 *     patternAtom
 *     patternList
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
 *     parenthesizedPattern
 *     orPattern
 *
 * Defining any of those here would create two pattern authorities.
 *
 * ============================================================================
 * PURPOSE OF THIS COMPONENT
 * ============================================================================
 *
 * Pattern-constrained types are a type-system feature.
 *
 * Their purpose is to express a relationship between:
 *
 *     a type
 *     and
 *     a source-level value/pattern constraint
 *
 * without embedding target-specific representation.
 *
 * Conceptually:
 *
 *     base type
 *          +
 *     pattern/refinement
 *          |
 *          v
 *     constrained semantic type
 *
 * The exact semantic interpretation is owned by the type checker and
 * refinement/constraint system.
 *
 * This grammar therefore provides only the smallest stable composition
 * boundary needed by the type orchestrator.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     patternType
 *     patternTypeBody
 *
 * These rules establish the type-level wrapper and its internal structural
 * boundary.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     universal value-pattern syntax
 *     match expressions
 *     match statements
 *     guards
 *     pattern exhaustiveness
 *     pattern reachability
 *     pattern overlap
 *     binding analysis
 *     type inference
 *     type unification
 *     refinement proving
 *     theorem proving
 *     constant evaluation
 *     ownership
 *     borrowing
 *     lifetimes
 *     effects
 *     capabilities
 *     resources
 *     policies
 *     provenance
 *     IR
 *     quantum routing
 *     quantum scheduling
 *     QEC
 *     HDL synthesis
 *     hardware placement
 *     target selection
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 *
 * Public:
 *
 *     patternType
 *
 * Internal:
 *
 *     patternTypeBody
 *
 * The type orchestrator is the only intended consumer of `patternType`.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     canonical type-expression composition
 *     canonical expression/value-pattern composition
 *
 * EXPORTS:
 *
 *     patternType
 *
 * CONSUMED_BY:
 *
 *     grammar/types/types.g4
 *     future canonical type-system composition where explicitly required
 *
 * AST_OWNER:
 *
 *     existing frontend type-expression / refinement AST infrastructure
 *
 * SEMANTIC_OWNER:
 *
 *     semantic type checker
 *     refinement/constraint checker
 *
 * IR_OWNER:
 *
 *     canonical semantic model
 *     downstream canonical IR
 *
 * TEST_OWNER:
 *
 *     grammar/tests/types/
 *     grammar/tests/semantic/
 *     repository frontend conformance tests
 *
 * SPEC_OWNER:
 *
 *     grammar/specification/types.md
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file declares no lexer rules.
 *
 * All lexical tokens MUST originate from:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * and its canonical lexical hierarchy.
 *
 * This file MUST NOT introduce:
 *
 *     PATTERN_TYPE
 *     PATTERN_BEGIN
 *     PATTERN_END
 *     RANGE-specific duplicate tokens
 *     duplicate punctuation tokens
 *
 * merely to support this type component.
 *
 * ============================================================================
 * GRAMMAR DEPENDENCY DIRECTION
 * ============================================================================
 *
 * The intended dependency direction is:
 *
 *     ZamaniLexer
 *          |
 *          v
 *     expression/value-pattern grammar
 *          |
 *          v
 *     type-system composition
 *          |
 *          v
 *     semantic type model
 *
 * The type-pattern component MUST NOT import:
 *
 *     grammar/statements/*
 *
 * The type-pattern component MUST NOT import:
 *
 *     grammar/expressions/expressions.g4
 *
 * merely to obtain the complete expression language.
 *
 * The component must remain a narrow type-system boundary.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * IMPORTANT:
 *
 * This file is intentionally a component grammar.
 *
 * It does not define:
 *
 *     typeExpression
 *
 * and it does not define:
 *
 *     program
 *
 * The canonical type orchestrator remains responsible for composing all
 * source-level type constructors.
 *
 * Therefore this file must never become a second type-expression root.
 *
 * ============================================================================
 * PATTERN-TYPE SURFACE
 * ============================================================================
 *
 * The production representation is intentionally minimal:
 *
 *     patternType
 *         : typeExpression patternTypeBody
 *         ;
 *
 * However, this component does not recursively import the complete type
 * orchestrator to implement that rule because doing so can create:
 *
 *     Type
 *       ->
 *     PatternTypes
 *       ->
 *     Type
 *
 * circular grammar composition.
 *
 * Consequently, the canonical production integration should expose the
 * `patternType` rule from the type orchestrator after its complete
 * composition has been established.
 *
 * This file therefore uses an explicit integration token boundary rather
 * than pretending that a standalone component can safely own the complete
 * recursive type language.
 *
 * ============================================================================
 * IMPORTANT PRODUCTION DECISION
 * ============================================================================
 *
 * A pattern-constrained type is NOT represented by inventing a lexer keyword.
 *
 * The language should remain open-world.
 *
 * Future type constraints should be expressible through:
 *
 *     generic parameters
 *     value parameters
 *     refinements
 *     predicates
 *     constraints
 *     dependent values
 *     registered semantic capabilities
 *
 * rather than requiring one new keyword for every possible domain.
 *
 * ============================================================================
 * REFINEMENT RELATION
 * ============================================================================
 *
 * Pattern-constrained types are closely related to refinement types.
 *
 * The architecture must distinguish:
 *
 *     type pattern
 *
 * from:
 *
 *     value pattern
 *
 * and:
 *
 *     refinement predicate
 *
 * A value pattern answers:
 *
 *     "Does this value have this structural form?"
 *
 * A refinement answers:
 *
 *     "Does this value satisfy this semantic predicate?"
 *
 * A type answers:
 *
 *     "What semantic type does this value inhabit?"
 *
 * These concepts may interact but MUST NOT collapse into one grammar authority.
 *
 * ============================================================================
 * TYPE-SYSTEM RELATION
 * ============================================================================
 *
 * Pattern-constrained types must compose with:
 *
 *     generics
 *     dependent values
 *     associated types
 *     type constraints
 *     refinements
 *     ownership
 *     affine types
 *     linear types
 *     references
 *     lifetimes
 *     effects
 *     capabilities
 *     resources
 *     temporal types
 *     classical types
 *     quantum types
 *     HDL/hardware semantic types
 *
 * Semantic validation determines whether a particular combination is legal.
 *
 * This grammar does not attempt to encode all combinations explicitly.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT create a new AST universe.
 *
 * The canonical frontend AST remains the only source-level AST.
 *
 * A successful pattern-constrained type must ultimately be represented using
 * the repository's canonical TypeExpr/refinement representation.
 *
 * If the existing AST does not yet have a dedicated pattern-constrained type
 * node, that is an AST implementation task and MUST NOT be solved by creating
 * a second grammar-local AST.
 *
 * Conceptually:
 *
 *     PatternType {
 *         base_type
 *         constraint
 *         source_span
 *     }
 *
 * is semantic structure, not an instruction to create a new incompatible AST
 * hierarchy.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     name resolution
 *     type resolution
 *     pattern resolution
 *     pattern/type compatibility
 *     constraint validation
 *     refinement validation
 *     constant evaluation where required
 *     exhaustiveness where relevant
 *     reachability where relevant
 *     binding validation
 *     ownership validation
 *     affine validation
 *     linear validation
 *     effect validation
 *     capability validation
 *     resource validation
 *     policy validation
 *     provenance
 *
 * The grammar performs none of these operations.
 *
 * ============================================================================
 * TYPE SAFETY
 * ============================================================================
 *
 * A pattern-constrained type MUST NOT silently weaken type safety.
 *
 * In particular:
 *
 *     failed inference
 *         !=
 *     dynamic/unknown escape
 *
 *     failed refinement proof
 *         !=
 *     automatic acceptance
 *
 *     unavailable capability
 *         !=
 *     type validity
 *
 *     unavailable resource
 *         !=
 *     type validity
 *
 * A semantically valid type must remain distinguishable from a valid type
 * whose realization is currently impossible on a particular target.
 *
 * ============================================================================
 * OWNERSHIP / LINEARITY / AFFINITY
 * ============================================================================
 *
 * Pattern matching can bind values with ownership-sensitive types.
 *
 * This file does not decide whether a binding:
 *
 *     moves
 *     borrows
 *     copies
 *     consumes
 *     aliases
 *
 * Those rules belong to the semantic ownership system.
 *
 * In particular, a pattern constraint MUST NOT implicitly make a linear value
 * copyable or an affine value reusable.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Pattern-constrained types do not introduce an effect merely because their
 * syntax exists.
 *
 * If a semantic constraint requires:
 *
 *     evaluation
 *     IO
 *     randomness
 *     measurement
 *     reflection
 *     foreign interaction
 *     network access
 *
 * the relevant effect must be determined by the semantic/effect system.
 *
 * This grammar does not define effects.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Pattern-constrained types do not directly require hardware capabilities.
 *
 * A semantic type may nevertheless participate in capability constraints.
 *
 * For example, a type representing a value that requires a particular
 * computational capability remains source-level semantic information.
 *
 * Capability satisfaction belongs to:
 *
 *     grammar/resources/
 *     semantic capability analysis
 *     compiler/runtime negotiation
 *
 * The grammar must never inspect the available machine.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * This file contains no resource limits.
 *
 * It MUST NOT encode limits for:
 *
 *     values
 *     pattern count
 *     pattern depth
 *     generic arguments
 *     tensor rank
 *     tensor dimensions
 *     qubits
 *     CPUs
 *     GPUs
 *     FPGAs
 *     accelerators
 *     nodes
 *     memory
 *     devices
 *     network topology
 *
 * There is no universal maximum pattern size.
 *
 * There is no universal maximum type size.
 *
 * Any implementation limit belongs to compiler/runtime resource policy and
 * must never silently become language semantics.
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
 * but this grammar does not own those constructs.
 *
 * Contract syntax remains owned by:
 *
 *     grammar/validation/
 *
 * A contract can constrain a value whose type is pattern-constrained.
 *
 * A contract MUST NOT change the source-level identity of the type merely
 * because a particular target cannot satisfy it.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Policies may restrict:
 *
 *     refinement evaluation
 *     proof obligations
 *     runtime checking
 *     reflection
 *     dynamic checks
 *     resource use
 *
 * Policy syntax is owned elsewhere.
 *
 * This grammar only provides the type-system structure consumed by policy
 * analysis.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Type-constrained information must preserve source provenance where the
 * frontend infrastructure supports it.
 *
 * Relevant provenance may include:
 *
 *     source span
 *     declaration origin
 *     generated-by information
 *     macro origin
 *     refinement origin
 *     semantic derivation
 *
 * This file does not define provenance storage.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum values may participate in pattern matching.
 *
 * However, the type grammar MUST NOT introduce physical quantum patterns.
 *
 * It must not encode:
 *
 *     physical qubit identifiers
 *     coupling maps
 *     calibration
 *     pulse topology
 *     physical QPU layout
 *     QEC code selection
 *     routing
 *     scheduling
 *
 * A semantic quantum type remains target-independent.
 *
 * The downstream quantum pipeline remains:
 *
 *     source
 *       |
 *       v
 *     AST
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
 * This type grammar participates only before that boundary.
 *
 * ============================================================================
 * CLASSICAL / AI / DATA / HDL INTEGRATION
 * ============================================================================
 *
 * The same pattern-constrained type machinery may apply to:
 *
 *     classical values
 *     numerical values
 *     tensor values
 *     model values
 *     datasets
 *     knowledge values
 *     probabilistic values
 *     distributed values
 *     HDL semantic values
 *     hardware abstraction values
 *
 * No domain-specific pattern type hierarchy is required.
 *
 * Domain-specific behavior is represented through normal types, generic
 * parameters, constraints, capabilities, and semantic registration.
 *
 * ============================================================================
 * INTEROPERABILITY
 * ============================================================================
 *
 * Foreign values may participate in pattern constraints only after their
 * foreign representation has been mapped into a valid semantic boundary.
 *
 * This file does not define:
 *
 *     ABI
 *     calling conventions
 *     foreign layout
 *     pointer representation
 *     data marshaling
 *
 * Those belong to:
 *
 *     grammar/interoperability/
 *
 * ============================================================================
 * DIALECT INTEGRATION
 * ============================================================================
 *
 * Dialects may provide additional semantic types and constraint forms.
 *
 * A dialect MUST NOT silently redefine the universal pattern language.
 *
 * Dialect extensions must declare:
 *
 *     syntax ownership
 *     semantic ownership
 *     compatibility
 *     AST representation
 *     lowering
 *     diagnostics
 *     provenance
 *
 * Core Zamani remains independent of any one dialect.
 *
 * ============================================================================
 * METAPROGRAMMING
 * ============================================================================
 *
 * Macros and compile-time facilities may generate pattern-constrained type
 * syntax.
 *
 * Generated syntax must pass through the same:
 *
 *     parsing
 *     structural validation
 *     name resolution
 *     type checking
 *     constraint checking
 *
 * as handwritten syntax.
 *
 * Metaprogramming must not bypass semantic validation.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing this component must be deterministic with respect to:
 *
 *     source text
 *     selected language version
 *     selected grammar configuration
 *
 * It must not depend on:
 *
 *     target hardware
 *     runtime state
 *     scheduler state
 *     randomness
 *     network state
 *     filesystem state
 *     resource availability
 *
 * Resource availability is evaluated downstream.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Pattern-constrained types describe semantic intent.
 *
 * They must remain independent of the machine on which they are realized.
 *
 * The same source-level type may ultimately participate in:
 *
 *     tiny embedded execution
 *     CPU execution
 *     multicore execution
 *     GPU execution
 *     FPGA execution
 *     ASIC execution
 *     accelerator execution
 *     QPU execution
 *     simulation
 *     HPC execution
 *     distributed execution
 *     cloud execution
 *     future execution targets
 *
 * Target realization may change:
 *
 *     representation
 *     layout
 *     optimization
 *     placement
 *     scheduling
 *     execution strategy
 *
 * but must not silently change the source type's semantic identity.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The grammar contains no artificial finite capacity.
 *
 * Arbitrarily large source structures remain representable subject only to
 * actual implementation resources.
 *
 * Compiler/resource limits must be expressed as implementation policy and
 * diagnostics, not as grammar constants.
 *
 * In particular, no source rule may assume:
 *
 *     fixed tuple size
 *     fixed generic arity
 *     fixed pattern depth
 *     fixed tensor rank
 *     fixed collection length
 *     fixed quantum size
 *     fixed machine size
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Syntax diagnostics are owned by the parser.
 *
 * Semantic diagnostics are owned by semantic analysis.
 *
 * The separation must remain explicit.
 *
 * Examples:
 *
 *     malformed type constraint
 *         -> parser diagnostic
 *
 *     unknown type
 *         -> name/type resolver diagnostic
 *
 *     invalid pattern/type relationship
 *         -> semantic type diagnostic
 *
 *     unsatisfied refinement
 *         -> refinement/constraint diagnostic
 *
 *     missing hardware capability
 *         -> capability/resource diagnostic
 *
 *     insufficient runtime resources
 *         -> execution/resource diagnostic
 *
 * These conditions must never be collapsed into one generic parser error.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This file introduces no replacement for the canonical value-pattern
 * grammar.
 *
 * Existing source using:
 *
 *     grammar/expressions/patterns.g4
 *
 * remains governed by that grammar.
 *
 * If a historical syntax is supported, compatibility handling belongs under:
 *
 *     grammar/compatibility/
 *
 * Historical aliases must not create duplicate canonical tokens or duplicate
 * pattern authorities.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Production completion requires tests at all relevant layers.
 *
 * STRUCTURAL TESTS:
 *
 *     - PatternTypes grammar generates successfully;
 *     - canonical lexer vocabulary resolves;
 *     - no lexer rules are declared here;
 *     - no duplicate pattern rule is declared;
 *     - no duplicate type-expression root is declared;
 *     - grammar imports remain acyclic.
 *
 * PARSER TESTS:
 *
 *     - valid pattern-type forms parse;
 *     - malformed pattern-type forms fail deterministically;
 *     - nested type structures remain bounded by implementation resources
 *       rather than language constants;
 *     - source spans are preserved.
 *
 * INTEGRATION TESTS:
 *
 *     - type orchestrator consumes patternType;
 *     - canonical expression pattern grammar remains the sole owner of
 *       `pattern`;
 *     - match expressions continue consuming expression patterns;
 *     - match statements continue consuming expression patterns;
 *     - type expressions remain the sole type composition authority.
 *
 * SEMANTIC TESTS:
 *
 *     - pattern/type compatibility;
 *     - invalid refinement;
 *     - valid refinement;
 *     - generic interaction;
 *     - dependent-value interaction;
 *     - ownership interaction;
 *     - affine interaction;
 *     - linear interaction;
 *     - reference interaction;
 *     - effect interaction;
 *     - capability interaction;
 *     - resource interaction.
 *
 * CROSS-DOMAIN TESTS:
 *
 *     - classical values;
 *     - tensor/data values;
 *     - probabilistic values;
 *     - knowledge values;
 *     - quantum semantic values;
 *     - hybrid values;
 *     - HDL semantic values;
 *     - hardware abstraction values;
 *     - distributed values.
 *
 * SCALABILITY TESTS:
 *
 *     - symbolic constraints;
 *     - large type graphs;
 *     - large nested structures;
 *     - large generic structures;
 *     - large pattern alternatives;
 *     - large source spans;
 *     - no artificial parser maximum is introduced.
 *
 * NEGATIVE TESTS:
 *
 *     - malformed pattern constraint;
 *     - missing type component;
 *     - malformed delimiter;
 *     - invalid nesting;
 *     - duplicate incompatible constraints where prohibited semantically;
 *     - unsatisfied semantic refinement;
 *     - invalid type/pattern relationship.
 *
 * DETERMINISM TESTS:
 *
 *     The same source and grammar configuration must produce the same parse
 *     structure and source spans.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST contain:
 *
 *     no universal hardware constants
 *     no maximum pattern count
 *     no maximum type count
 *     no maximum generic arity
 *     no maximum tuple arity
 *     no maximum tensor rank
 *     no maximum quantum count
 *     no maximum node count
 *     no maximum memory size
 *     no target-specific numeric assumptions
 *
 * It MUST NOT contain:
 *
 *     CPU-specific syntax
 *     GPU-specific syntax
 *     FPGA-specific syntax
 *     QPU-specific syntax
 *     vendor-specific physical identifiers
 *     physical topology
 *     runtime resource discovery
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust actions.
 *
 * Rust integration must use:
 *
 *     Rust 1.97 or later
 *     Rust 2021
 *     safe Rust
 *
 * No unsafe Rust is required or permitted by this grammar feature.
 *
 * Parser generation must not embed:
 *
 *     filesystem operations
 *     network operations
 *     hardware operations
 *     dynamic execution
 *     target probing
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] the repository explicitly confirms what semantic surface
 *         `patternType` represents;
 *
 *     [ ] the canonical value-pattern grammar remains
 *         grammar/expressions/patterns.g4;
 *
 *     [ ] no competing `pattern` rule exists here;
 *
 *     [ ] no competing `typePattern` rule exists here;
 *
 *     [ ] the canonical type orchestrator consumes this component;
 *
 *     [ ] AST representation is defined by the existing frontend AST;
 *
 *     [ ] semantic ownership is defined;
 *
 *     [ ] refinement/constraint ownership is defined;
 *
 *     [ ] effects are delegated to the effect system;
 *
 *     [ ] capabilities are delegated to capability analysis;
 *
 *     [ ] resources are delegated to resource analysis;
 *
 *     [ ] policies are delegated to policy analysis;
 *
 *     [ ] provenance is preserved;
 *
 *     [ ] quantum semantics terminate at the canonical quantum::ir boundary;
 *
 *     [ ] no target-specific realization is encoded;
 *
 *     [ ] no artificial scalability ceiling exists;
 *
 *     [ ] positive tests pass;
 *
 *     [ ] negative tests pass;
 *
 *     [ ] boundary tests pass;
 *
 *     [ ] cross-domain tests pass;
 *
 *     [ ] determinism tests pass;
 *
 *     [ ] safe Rust integration passes on Rust 1.97 or later.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * This file exists to give the type system a stable home for
 * pattern-constrained type semantics.
 *
 * It is NOT another pattern language.
 *
 * The authoritative architecture is:
 *
 *     grammar/expressions/patterns.g4
 *             |
 *             | value-pattern syntax
 *             v
 *        canonical pattern
 *             |
 *             v
 *     grammar/types/pattern.g4
 *             |
 *             | type-level integration
 *             v
 *        canonical type system
 *             |
 *             v
 *        semantic model
 *             |
 *       +-----+------+----------------+
 *       |            |                |
 *       v            v                v
 *   classical    quantum::ir      HDL/hardware
 *       |            |                |
 *       +------------+----------------+
 *                    |
 *                    v
 *              target-independent
 *                 optimization
 *                    |
 *               lowering/routing
 *                    |
 *                scheduling
 *                    |
 *             resilience/recovery
 *                    |
 *                  ZQN
 *                    |
 *                  HAL
 *                    |
 *              target realization
 *
 * The grammar therefore remains portable, open-ended, target-independent,
 * deterministic, and free of artificial hardware limits.
 *
 * ============================================================================
 */
parser grammar PatternTypes;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * PUBLIC TYPE-PATTERN ENTRY
 * ============================================================================
 *
 * NOTE:
 *
 * The concrete type-expression implementation belongs to the canonical type
 * orchestrator. This component deliberately does not import that orchestrator
 * because doing so would create a circular dependency when the orchestrator
 * imports this component.
 *
 * The production composition layer should bind the semantic type-pattern
 * construct to the canonical type-expression rule.
 *
 * The leaf rule below is therefore intentionally a structural marker rather
 * than a second complete type grammar.
 *
 * If the repository's ANTLR composition is later normalized so that a shared
 * type-expression delegate can be imported without a cycle, this rule may be
 * widened through that delegate without changing the ownership contract.
 */
patternType
    : patternTypeBody
    ;


/*
 * ============================================================================
 * TYPE-PATTERN BODY
 * ============================================================================
 *
 * This rule is intentionally a narrow integration hook.
 *
 * The exact surface representation of a pattern-constrained type MUST be
 * finalized by grammar/types/types.g4 and grammar/specification/types.md
 * together.
 *
 * It MUST NOT consume arbitrary complete expressions here because doing so
 * would make this component capable of swallowing unrelated expression
 * syntax and would introduce parser ambiguity with the canonical expression
 * pattern grammar.
 *
 * Until the normative type syntax is established, the production-safe form
 * is an explicit parenthesized constraint boundary.
 *
 * The semantic type orchestrator owns the interpretation of the contents.
 *
 * IMPORTANT:
 *
 * The token sequence inside the boundary is intentionally represented using
 * the canonical expression/pattern entry only after the composition grammar
 * supplies that dependency. This leaf cannot safely import the full
 * expression grammar without potentially creating a dependency cycle.
 *
 * Therefore this rule is a placeholder integration boundary and MUST NOT be
 * promoted to a normative source syntax until the corresponding specification
 * defines its concrete delimiters.
 */
patternTypeBody
    : LBRACE
      RBRACE
    ;