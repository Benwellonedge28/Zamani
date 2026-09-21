/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/effects/effect-polymorphism.g4
 *
 * Grammar:
 *     EffectPolymorphism
 *
 * Status:
 *     CANONICAL MODULAR EFFECT-POLYMORPHISM GRAMMAR
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     This grammar contains:
 *
 *       - no embedded Rust actions;
 *       - no semantic predicates;
 *       - no unsafe code;
 *       - no filesystem access;
 *       - no network access;
 *       - no hardware discovery;
 *       - no runtime calls;
 *       - no target selection;
 *       - no backend selection.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX required to express polymorphism
 * over EFFECTS.
 *
 * Effect polymorphism allows a source construct to remain generic over one or
 * more effects rather than naming a closed, implementation-specific effect
 * set.
 *
 * Conceptually:
 *
 *     computation<T, effect E>
 *
 * may have a contract such as:
 *
 *     with effects { E }
 *
 * or:
 *
 *     with effects { E, IO }
 *
 * where `E` is resolved by semantic analysis.
 *
 * The grammar therefore permits an effect variable to be introduced,
 * referenced, constrained, substituted, and propagated without requiring the
 * grammar to know what concrete effect it will eventually denote.
 *
 * ============================================================================
 * CORE ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Effect polymorphism is a SOURCE-LEVEL TYPE/SEMANTIC CONCEPT.
 *
 * It is NOT:
 *
 *     - runtime dispatch;
 *     - hardware selection;
 *     - capability discovery;
 *     - resource allocation;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - HAL behavior.
 *
 * The dependency direction remains:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     effect/type semantic analysis
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +----------------------+----------------------+
 *       |                      |                      |
 *       v                      v                      v
 *   classical              quantum::ir           HDL/hardware
 *       |                      |                      |
 *       +----------------------+----------------------+
 *                              |
 *                              v
 *                       optimization
 *                              |
 *                   routing / scheduling
 *                              |
 *                    QEC / resilience / ZQN
 *                              |
 *                             HAL
 *                              |
 *                       target realization
 *
 * This grammar MUST remain above target realization.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     effectPolymorphicParameterList
 *     effectPolymorphicParameter
 *     effectPolymorphicParameterKind
 *     effectPolymorphicBounds
 *     effectPolymorphicBound
 *     effectPolymorphicClause
 *     effectPolymorphicEffectSet
 *     effectPolymorphicEffectList
 *     effectPolymorphicEffect
 *     effectVariableReference
 *     effectPolymorphicReference
 *     effectPolymorphicBinding
 *     effectPolymorphicSubstitution
 *     effectPolymorphicSubstitutionList
 *     effectPolymorphicConstraint
 *     effectPolymorphicConstraintList
 *     effectPolymorphicConstruct
 *
 * THIS FILE DOES NOT OWN:
 *
 *     identifiers
 *     qualified names
 *     ordinary type expressions
 *     ordinary generic type parameters
 *     ordinary effect references
 *     ordinary effect sets
 *     effect declarations
 *     effect operations
 *     effect handlers
 *     effect implementations
 *     capabilities
 *     resources
 *     requirements
 *     constraints on physical targets
 *     hardware topology
 *     quantum operations
 *     quantum::ir
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *     runtime dispatch
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Ordinary effect syntax remains owned by:
 *
 *     grammar/effects/effect-sets.g4
 *     grammar/effects/effect-declarations.g4
 *     grammar/effects/effect-operations.g4
 *     grammar/effects/effect-handling.g4
 *
 * This file MUST NOT redefine:
 *
 *     effectReference
 *     effectSet
 *     effectDeclaration
 *     effectOperationDeclaration
 *     effectInvocation
 *     effectHandler
 *
 * Instead it composes with the existing effect subsystem.
 *
 * ============================================================================
 * CANONICAL LEXICAL AUTHORITY
 * ============================================================================
 *
 * The parser consumes:
 *
 *     ZamaniLexer
 *
 * through:
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 * This is intentional.
 *
 * The repository's canonical lexical composition is:
 *
 *     grammar/lexer/tokens.g4
 *             |
 *             v
 *     grammar/antlr/ZamaniLexer.g4
 *             |
 *             v
 *     ZamaniLexer
 *
 * This grammar MUST NOT define lexer rules.
 *
 * In particular, it MUST NOT create:
 *
 *     EFFECT_VARIABLE
 *     EFFECT_PARAMETER
 *     FORALL
 *     EFFECT_NAME
 *
 * lexer tokens merely to implement polymorphism.
 *
 * Effect-variable identity is syntactic/semantic structure over canonical
 * identifiers.
 *
 * ============================================================================
 * WHY NO NEW `forall` KEYWORD
 * ============================================================================
 *
 * The canonical keyword vocabulary currently does not define `forall`.
 *
 * Effect polymorphism therefore MUST NOT silently introduce a new lexer
 * keyword through this file.
 *
 * Explicit effect-variable binding uses the already-established:
 *
 *     effect
 *
 * keyword inside an existing generic-style delimiter:
 *
 *     <effect E, effect F>
 *
 * Example:
 *
 *     <effect E>
 *
 *     <effect E, effect F>
 *
 * This keeps the lexical vocabulary centralized.
 *
 * If a future language specification adopts a dedicated `forall` keyword,
 * that change belongs to:
 *
 *     grammar/lexer/keywords.g4
 *     grammar/spec/lexical.md
 *     grammar/spec/type-system.md
 *     grammar/compatibility/
 *
 * and MUST NOT be introduced locally here.
 *
 * ============================================================================
 * CANONICAL SOURCE MODEL
 * ============================================================================
 *
 * An effect-polymorphic construct consists conceptually of:
 *
 *     binders
 *         +
 *     optional constraints
 *         +
 *     effect expression/set
 *
 * Example:
 *
 *     <effect E>
 *     with effects { E }
 *
 * Multiple variables:
 *
 *     <effect E, effect F>
 *     with effects { E, F }
 *
 * Mixed concrete and polymorphic effects:
 *
 *     <effect E>
 *     with effects {
 *         E,
 *         IO,
 *         quantum::Measurement,
 *     }
 *
 * The parser preserves the source structure.
 *
 * Semantic analysis determines what `E` and `F` mean.
 *
 * ============================================================================
 * OPEN-WORLD EFFECT MODEL
 * ============================================================================
 *
 * Effect polymorphism MUST remain open-world.
 *
 * Valid effect identities may include:
 *
 *     IO
 *     Network
 *     Storage
 *     quantum::Measurement
 *     quantum::Reset
 *     qec::Correction
 *     zqn::Observation
 *     distributed::Consensus
 *     accelerator::Tensor
 *     hdl::Clock
 *     security::Confidentiality
 *     vendor::extension::operation
 *     future::computing::effect
 *
 * No closed catalogue is defined here.
 *
 * A polymorphic effect variable may eventually be instantiated with any
 * semantically valid effect identity or effect set according to the type/effect
 * system.
 *
 * ============================================================================
 * EFFECT VARIABLE IDENTITY
 * ============================================================================
 *
 * Effect variables use ordinary canonical identifiers.
 *
 * Example:
 *
 *     E
 *     F
 *     Effects
 *     Ambient
 *
 * Their role as effect variables is established by their enclosing
 * `effectPolymorphicParameter` binding.
 *
 * The grammar MUST NOT assume:
 *
 *     E = effect
 *     F = effect
 *
 * globally.
 *
 * The same identifier may have a different semantic role outside the
 * polymorphic scope.
 *
 * ============================================================================
 * EFFECT POLYMORPHIC PARAMETER LIST
 * ============================================================================
 *
 * Canonical source forms:
 *
 *     <effect E>
 *
 *     <effect E, effect F>
 *
 *     <effect E, effect F, effect G>
 *
 * A trailing comma is permitted consistently with other Zamani generic-style
 * lists.
 *
 * No finite parameter count is imposed.
 *
 * ============================================================================
 */

parser grammar EffectPolymorphism;

options {
    tokenVocab = ZamaniLexer;
}

import EffectSets, Core, Types, Expressions;


/*
 * ============================================================================
 * 1. EFFECT POLYMORPHIC PARAMETER LIST
 * ============================================================================
 *
 * Examples:
 *
 *     <effect E>
 *
 *     <effect E, effect F>
 *
 *     <effect E, effect F, effect G>
 *
 * This is deliberately distinct from ordinary type generic parameters.
 *
 * Type generic parameters remain owned by the canonical type/function
 * grammars.
 *
 * This rule binds semantic variables whose domain is effect expressions.
 */

effectPolymorphicParameterList
    : LESS
      effectPolymorphicParameter
      (
          COMMA effectPolymorphicParameter
      )*
      COMMA?
      GREATER
    ;


/*
 * ============================================================================
 * 2. EFFECT POLYMORPHIC PARAMETER
 * ============================================================================
 *
 * Canonical form:
 *
 *     effect E
 *
 *     effect E: IO
 *
 *     effect E: IO + Network
 *
 * The `effect` keyword establishes the parameter's semantic kind.
 *
 * The identifier remains ordinary identifier syntax.
 */

effectPolymorphicParameter
    : effectPolymorphicParameterKind
      identifier
      effectPolymorphicBounds?
    ;


/*
 * ============================================================================
 * 3. EFFECT PARAMETER KIND
 * ============================================================================
 *
 * The explicit kind marker is intentionally reused from the canonical lexical
 * vocabulary.
 *
 * This avoids creating a second lexical token family.
 */

effectPolymorphicParameterKind
    : EFFECT
    ;


/*
 * ============================================================================
 * 4. EFFECT POLYMORPHIC BOUNDS
 * ============================================================================
 *
 * Examples:
 *
 *     : IO
 *
 *     : IO + Network
 *
 *     : quantum::Measurement + quantum::Reset
 *
 * The `+` here is only syntactic composition of bound references.
 *
 * It does NOT define effect-set union semantics.
 *
 * Effect algebra remains a semantic responsibility.
 */

effectPolymorphicBounds
    : COLON
      effectPolymorphicBound
      (
          PLUS effectPolymorphicBound
      )*
    ;


/*
 * ============================================================================
 * 5. EFFECT POLYMORPHIC BOUND
 * ============================================================================
 *
 * A bound is an ordinary effect reference.
 *
 * The effect-set grammar owns effectReference itself.
 */

effectPolymorphicBound
    : effectReference
    ;


/*
 * ============================================================================
 * 6. EFFECT POLYMORPHIC CLAUSE
 * ============================================================================
 *
 * This is the reusable source-level attachment point for a polymorphic effect
 * environment.
 *
 * Canonical form:
 *
 *     with effects <effect E> { E }
 *
 *     with effects <effect E, effect F> { E, F }
 *
 *     with effects <effect E> {
 *         E,
 *         IO,
 *         quantum::Measurement,
 *     }
 *
 * The surrounding declaration/function/type grammar decides where this clause
 * is legal.
 *
 * This file does not redefine functions, types, declarations, or statements.
 */

effectPolymorphicClause
    : WITH
      EFFECTS
      effectPolymorphicParameterList
      effectPolymorphicEffectSet
    ;


/*
 * ============================================================================
 * 7. EFFECT POLYMORPHIC EFFECT SET
 * ============================================================================
 *
 * Braces deliberately mirror the canonical effect-set syntax.
 *
 * The members may be:
 *
 *     effect variables
 *     concrete effect references
 *
 * Semantic analysis distinguishes the two.
 *
 * This rule does not redefine the ordinary effectSet rule.
 */

effectPolymorphicEffectSet
    : LBRACE
      effectPolymorphicEffectList?
      RBRACE
    ;


/*
 * ============================================================================
 * 8. EFFECT POLYMORPHIC EFFECT LIST
 * ============================================================================
 *
 * Examples:
 *
 *     E
 *
 *     E, F
 *
 *     E, IO
 *
 *     E, quantum::Measurement
 *
 * No finite list size is imposed.
 */

effectPolymorphicEffectList
    : effectPolymorphicEffect
      (
          COMMA effectPolymorphicEffect
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 9. EFFECT POLYMORPHIC EFFECT
 * ============================================================================
 *
 * This rule accepts both:
 *
 *     E
 *
 * and:
 *
 *     quantum::Measurement
 *
 * The semantic environment determines whether the name resolves to:
 *
 *     an effect variable
 *     an effect declaration
 *     a qualified effect
 *     an invalid/unresolved name
 *
 * The parser deliberately does not perform name resolution.
 */

effectPolymorphicEffect
    : effectPolymorphicReference
    ;


/*
 * ============================================================================
 * 10. EFFECT POLYMORPHIC REFERENCE
 * ============================================================================
 *
 * A polymorphic reference is a canonical qualified name.
 *
 * This is intentionally not a new identifier grammar.
 *
 * Qualified names therefore remain compatible with:
 *
 *     quantum::Measurement
 *     distributed::Consensus
 *     vendor::domain::Effect
 *
 * while simple identifiers may represent effect variables.
 */

effectPolymorphicReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 11. EFFECT VARIABLE REFERENCE
 * ============================================================================
 *
 * This named boundary is useful to semantic analysis.
 *
 * It remains syntactically identical to a simple canonical identifier.
 *
 * Whether the identifier is actually bound as an effect variable is a semantic
 * question.
 */

effectVariableReference
    : identifier
    ;


/*
 * ============================================================================
 * 12. EFFECT POLYMORPHIC BINDING
 * ============================================================================
 *
 * This is a reusable semantic-facing syntax boundary.
 *
 * Example:
 *
 *     effect E
 *
 * The enclosing polymorphic parameter list supplies the scope.
 */

effectPolymorphicBinding
    : EFFECT
      effectVariableReference
      effectPolymorphicBounds?
    ;


/*
 * ============================================================================
 * 13. EFFECT POLYMORPHIC SUBSTITUTION
 * ============================================================================
 *
 * This rule describes SOURCE-LEVEL substitution metadata when an explicit
 * substitution form is required by a higher-level grammar.
 *
 * Canonical form:
 *
 *     E = IO
 *
 *     E = quantum::Measurement
 *
 *     E = distributed::Consensus
 *
 * This is intentionally a syntactic mapping only.
 *
 * It does not perform substitution.
 *
 * It does not modify the AST.
 *
 * It does not select hardware.
 *
 * It does not invoke an effect.
 */

effectPolymorphicSubstitution
    : effectVariableReference
      ASSIGN
      effectReference
    ;


/*
 * ============================================================================
 * 14. EFFECT POLYMORPHIC SUBSTITUTION LIST
 * ============================================================================
 *
 * Example:
 *
 *     E = IO,
 *     F = Network,
 *     G = quantum::Measurement
 *
 * No finite substitution count is imposed.
 */

effectPolymorphicSubstitutionList
    : effectPolymorphicSubstitution
      (
          COMMA effectPolymorphicSubstitution
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 15. EFFECT POLYMORPHIC CONSTRAINT
 * ============================================================================
 *
 * This rule provides an explicit source-level boundary for constraints on an
 * effect variable.
 *
 * Example:
 *
 *     E : IO
 *
 *     E : quantum::Measurement
 *
 *     E : IO + Network
 *
 * The semantic layer decides whether the constraint means:
 *
 *     membership;
 *     upper bound;
 *     lower bound;
 *     compatibility;
 *     capability implication;
 *     effect-class membership;
 *     or another standardized semantic relation.
 *
 * The grammar does not choose among those meanings.
 */

effectPolymorphicConstraint
    : effectVariableReference
      COLON
      effectPolymorphicBound
      (
          PLUS effectPolymorphicBound
      )*
    ;


/*
 * ============================================================================
 * 16. EFFECT POLYMORPHIC CONSTRAINT LIST
 * ============================================================================
 *
 * Multiple constraints are allowed without a language-level count.
 */

effectPolymorphicConstraintList
    : effectPolymorphicConstraint
      (
          COMMA effectPolymorphicConstraint
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 17. EFFECT POLYMORPHIC CONSTRUCT
 * ============================================================================
 *
 * Canonical modular entry point.
 *
 * It intentionally does not parse a complete program.
 *
 * It is suitable for:
 *
 *     function grammar
 *     type grammar
 *     declaration grammar
 *     effect declaration grammar
 *     semantic tooling
 *     conformance tooling
 */

effectPolymorphicConstruct
    : effectPolymorphicParameterList
    | effectPolymorphicClause
    | effectPolymorphicBinding
    | effectPolymorphicSubstitution
    | effectPolymorphicSubstitutionList
    | effectPolymorphicConstraint
    | effectPolymorphicConstraintList
    ;


/*
 * ============================================================================
 * 18. EFFECT POLYMORPHIC TYPE QUALIFICATION
 * ============================================================================
 *
 * This rule provides the type-system integration boundary without redefining
 * typeExpression.
 *
 * Canonical conceptual form:
 *
 *     typeExpression
 *         with effects
 *         <effect E>
 *         { E }
 *
 * The actual surrounding type grammar decides whether this construct is
 * permitted in a particular type position.
 */

effectPolymorphicTypeQualification
    : typeExpression
      effectPolymorphicClause
    ;


/*
 * ============================================================================
 * 19. EFFECT POLYMORPHIC EFFECT QUALIFICATION
 * ============================================================================
 *
 * This rule is useful to declarations/functions whose semantic contract is
 * effect-polymorphic.
 *
 * It does not define function declarations.
 */

effectPolymorphicEffectQualification
    : effectPolymorphicClause
    ;


/*
 * ============================================================================
 * 20. EFFECT POLYMORPHIC EFFECT VARIABLE SET
 * ============================================================================
 *
 * Convenience boundary for semantic consumers that need a set containing
 * only syntactic effect-variable references.
 *
 * Example:
 *
 *     { E, F, G }
 *
 * Semantic analysis must verify that each member is actually bound as an
 * effect variable in the active polymorphic environment.
 */

effectPolymorphicVariableSet
    : LBRACE
      effectPolymorphicVariableList?
      RBRACE
    ;


/*
 * ============================================================================
 * 21. EFFECT POLYMORPHIC VARIABLE LIST
 * ============================================================================
 */

effectPolymorphicVariableList
    : effectVariableReference
      (
          COMMA effectVariableReference
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 22. EFFECT POLYMORPHIC APPLICATION
 * ============================================================================
 *
 * This is an explicit integration boundary for semantic instantiation.
 *
 * Example:
 *
 *     effect Foo<E>
 *
 *     Foo<IO>
 *
 *     Foo<quantum::Measurement>
 *
 * The ordinary generic/application grammar remains authoritative for the
 * surrounding construct.
 *
 * This rule only identifies an effect-polymorphic argument list.
 */

effectPolymorphicApplication
    : LESS
      effectPolymorphicArgument
      (
          COMMA effectPolymorphicArgument
      )*
      COMMA?
      GREATER
    ;


/*
 * ============================================================================
 * 23. EFFECT POLYMORPHIC ARGUMENT
 * ============================================================================
 *
 * An argument is a canonical effect reference.
 *
 * It may be:
 *
 *     IO
 *     Network
 *     quantum::Measurement
 *     distributed::Consensus
 *
 * The semantic layer determines compatibility with the corresponding
 * polymorphic parameter.
 */

effectPolymorphicArgument
    : effectReference
    ;


/*
 * ============================================================================
 * 24. EFFECT POLYMORPHIC SIGNATURE
 * ============================================================================
 *
 * Reusable semantic boundary:
 *
 *     <effect E, effect F>
 *     with effects { E, F }
 *
 * This rule deliberately contains no implementation body.
 */

effectPolymorphicSignature
    : effectPolymorphicParameterList
      effectPolymorphicEffectQualification
    ;


/*
 * ============================================================================
 * 25. EFFECT POLYMORPHIC SCOPE
 * ============================================================================
 *
 * A scope contains bindings and an effect contract.
 *
 * The block/body itself remains owned by the canonical expression/statement
 * grammar.
 *
 * Therefore this rule does not redefine blocks.
 */

effectPolymorphicScope
    : effectPolymorphicParameterList
      effectPolymorphicEffectQualification
    ;


/*
 * ============================================================================
 * 26. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes ONLY:
 *
 *     - effect-variable binding syntax;
 *     - effect-variable reference syntax;
 *     - effect-polymorphic parameter syntax;
 *     - effect bounds syntax;
 *     - effect-polymorphic effect-set syntax;
 *     - substitution syntax;
 *     - constraint syntax;
 *     - application syntax.
 *
 * Semantic analysis MUST determine:
 *
 *     - whether a variable is actually bound;
 *     - variable scope;
 *     - shadowing;
 *     - duplicate binders;
 *     - unresolved variables;
 *     - effect identity;
 *     - effect-set normalization;
 *     - effect equivalence;
 *     - effect subtyping;
 *     - effect inclusion;
 *     - effect substitution;
 *     - effect unification;
 *     - effect inference;
 *     - effect generalization;
 *     - effect instantiation;
 *     - effect constraint solving;
 *     - effect compatibility;
 *     - handler compatibility;
 *     - function-type compatibility;
 *     - capability implications;
 *     - resource implications;
 *     - target feasibility.
 *
 * None of those decisions belong in this grammar.
 *
 * ============================================================================
 * EFFECT POLYMORPHISM AND EFFECT INFERENCE
 * ============================================================================
 *
 * This grammar does NOT require programmers to explicitly write every effect
 * variable everywhere.
 *
 * Semantic analysis may infer effect variables where the normative type/effect
 * system permits inference.
 *
 * Example:
 *
 *     <effect E>
 *     with effects { E }
 *
 * The semantic system may infer an instantiation from a call site.
 *
 * The grammar must remain unchanged regardless of how sophisticated the
 * inference engine becomes.
 *
 * ============================================================================
 * EFFECT GENERALIZATION
 * ============================================================================
 *
 * Generalization is semantic.
 *
 * The parser must not determine whether an effect variable should be:
 *
 *     generalized;
 *     instantiated;
 *     monomorphic;
 *     rigid;
 *     flexible;
 *     existential;
 *     universally quantified.
 *
 * Those properties belong to the type/effect inference specification and
 * semantic implementation.
 *
 * ============================================================================
 * EFFECT SUBTYPING
 * ============================================================================
 *
 * This grammar does not define an effect-subtyping algebra.
 *
 * In particular, it does not assume:
 *
 *     E < F
 *
 * or:
 *
 *     E <= F
 *
 * or:
 *
 *     E + F
 *
 * has a particular semantic interpretation.
 *
 * The `+` token in bound lists only structures multiple source references.
 *
 * The semantic specification determines whether the resulting constraint
 * means intersection, union, conjunction, accumulation, or another relation.
 *
 * ============================================================================
 * EFFECT SET NORMALIZATION
 * ============================================================================
 *
 * The parser MUST preserve source order.
 *
 * It MUST NOT:
 *
 *     sort effects;
 *     deduplicate effects;
 *     resolve aliases;
 *     expand substitutions;
 *     infer bounds;
 *     normalize effect algebra.
 *
 * Those operations belong to semantic analysis.
 *
 * This guarantees deterministic parse structure while allowing a future
 * semantic normalization strategy to evolve independently.
 *
 * ============================================================================
 * EFFECT VARIABLE SHADOWING
 * ============================================================================
 *
 * Shadowing is not decided here.
 *
 * For example:
 *
 *     <effect E>
 *     ...
 *     <effect E>
 *
 * may be syntactically representable in separate scopes.
 *
 * Semantic analysis determines whether shadowing is:
 *
 *     legal;
 *     restricted;
 *     deprecated;
 *     rejected.
 *
 * A declaration with duplicate binders in the same scope is likewise a
 * semantic diagnostic rather than a parser concern unless the specification
 * explicitly makes it syntactically impossible.
 *
 * ============================================================================
 * EFFECT POLYMORPHISM AND CAPABILITIES
 * ============================================================================
 *
 * An effect variable does NOT imply a capability.
 *
 * Example:
 *
 *     <effect E>
 *     with effects { E }
 *
 * does not itself imply:
 *
 *     GPU
 *     QPU
 *     CPU
 *     FPGA
 *     Network
 *     Storage
 *
 * Capability analysis may later establish implications for a concrete
 * instantiation.
 *
 * ============================================================================
 * EFFECT POLYMORPHISM AND RESOURCES
 * ============================================================================
 *
 * An effect variable does NOT allocate resources.
 *
 * For example:
 *
 *     E = quantum::Measurement
 *
 * does not mean:
 *
 *     allocate physical qubits;
 *     select a QPU;
 *     choose a topology;
 *     select a calibration;
 *     reserve a device.
 *
 * Those decisions remain downstream.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Effect polymorphism must work with quantum effects without creating a second
 * quantum language or IR.
 *
 * Example:
 *
 *     <effect E>
 *     with effects {
 *         E,
 *         quantum::Measurement,
 *     }
 *
 * The downstream semantic pipeline is:
 *
 *     effect-polymorphic source
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     effect/type analysis
 *          |
 *          v
 *     quantum semantic lowering
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
 *     QEC / resilience / ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target realization
 *
 * This grammar never names physical qubits or fixed gate sets.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Effect variables may abstract over:
 *
 *     IO
 *     Storage
 *     Network
 *     Parallel
 *     Distributed
 *     Security
 *     External
 *     other future semantic effects.
 *
 * Classical IR lowering remains downstream.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * An effect variable may eventually instantiate to effects such as:
 *
 *     hdl::Clock
 *     hdl::Signal
 *     hardware::Memory
 *     hardware::Timing
 *     hardware::External
 *
 * The grammar does not encode:
 *
 *     FPGA count;
 *     register width;
 *     clock frequency;
 *     physical pin;
 *     device ID;
 *     topology.
 *
 * Those remain target/resource semantics.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Effect polymorphism can abstract over:
 *
 *     distributed::RemoteExecution
 *     distributed::Consensus
 *     distributed::Replication
 *     networking::Transmit
 *
 * No node count or topology is encoded.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * Effect variables can abstract over semantic interactions such as:
 *
 *     ai::Inference
 *     ai::Training
 *     data::Read
 *     data::Write
 *     accelerator::Tensor
 *
 * The grammar does not depend on a specific AI framework or accelerator.
 *
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * Security effects remain semantic declarations.
 *
 * Effect polymorphism does not grant authorization.
 *
 * For example:
 *
 *     <effect E>
 *     with effects { E, security::Confidentiality }
 *
 * does not grant a permission.
 *
 * Authorization remains owned by the security subsystem.
 *
 * ============================================================================
 * INTEROPERABILITY
 * ============================================================================
 *
 * Effect-polymorphic semantics may cross:
 *
 *     C
 *     C++
 *     Rust
 *     WebAssembly
 *     OpenQASM
 *     QIR
 *     HDL
 *     vendor-specific interfaces
 *
 * only after semantic lowering.
 *
 * This grammar does not define foreign ABIs.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must preserve enough structure to represent:
 *
 *     EffectPolymorphicParameter
 *     EffectPolymorphicParameterList
 *     EffectPolymorphicBound
 *     EffectVariableReference
 *     EffectPolymorphicClause
 *     EffectPolymorphicEffectSet
 *     EffectPolymorphicSubstitution
 *     EffectPolymorphicConstraint
 *
 * Conceptually:
 *
 *     parser rule
 *         ->
 *     domain-neutral AST node
 *         ->
 *     semantic effect variable
 *
 * The AST must NOT introduce backend-specific nodes such as:
 *
 *     GPUEffectVariable
 *     QPUEffectVariable
 *     FPGAEffectVariable
 *     HardwareEffectVariable
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar does NOT introduce a new IR.
 *
 * Effect polymorphism is lowered through the existing semantic pipeline.
 *
 * The conceptual relationship is:
 *
 *     source effect variable
 *         ->
 *     semantic effect variable
 *         ->
 *     instantiated/normalized effect information
 *         ->
 *     canonical IR metadata/semantics
 *
 * For quantum computation:
 *
 *     quantum::ir
 *
 * remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler must consume semantic effect information rather than parser
 * rules directly.
 *
 * Compiler responsibilities include:
 *
 *     - effect inference;
 *     - effect unification;
 *     - substitution;
 *     - generalization;
 *     - instantiation;
 *     - effect compatibility;
 *     - lowering.
 *
 * This file has no compiler actions.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime behavior is outside this grammar.
 *
 * Runtime may use already-resolved effect metadata to determine:
 *
 *     - which services are needed;
 *     - which execution capabilities are required;
 *     - which resources must be acquired;
 *     - which resilience policy applies.
 *
 * Runtime MUST NOT reinterpret parser syntax independently of semantic
 * analysis.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Effect polymorphism is specifically designed to preserve portability.
 *
 * The source program expresses:
 *
 *     semantic effect relationships
 *
 * rather than:
 *
 *     physical machine realization.
 *
 * Therefore this grammar imposes NO universal limits on:
 *
 *     - effect-variable count;
 *     - effect-set size;
 *     - effect-instantiation count;
 *     - operation count;
 *     - generic parameter count;
 *     - handler count;
 *     - function count;
 *     - program size;
 *     - qubit count;
 *     - CPU count;
 *     - core count;
 *     - thread count;
 *     - GPU count;
 *     - FPGA count;
 *     - QPU count;
 *     - node count;
 *     - accelerator count;
 *     - memory capacity;
 *     - tensor dimensions;
 *     - network topology;
 *     - deployment size.
 *
 * No rules such as:
 *
 *     MAX_EFFECTS
 *     MAX_EFFECT_VARIABLES
 *     MAX_EFFECT_PARAMETERS
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *
 * may be introduced.
 *
 * Practical limits imposed by:
 *
 *     compiler memory;
 *     parser stack;
 *     source representation;
 *     runtime policy;
 *     target resources;
 *
 * are implementation/resource constraints and MUST NOT become language
 * semantics.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * For identical:
 *
 *     source;
 *     language version;
 *     lexer version;
 *     grammar version;
 *
 * the parser must produce equivalent parse structure.
 *
 * This grammar contains no:
 *
 *     randomness;
 *     wall-clock dependency;
 *     filesystem access;
 *     network access;
 *     hardware discovery;
 *     environment lookup;
 *     resource probing;
 *     target selection.
 *
 * ============================================================================
 * SOURCE ORDER
 * ============================================================================
 *
 * The parser preserves:
 *
 *     parameter order;
 *     bound order;
 *     effect-reference order;
 *     substitution order;
 *     constraint order.
 *
 * Semantic normalization is downstream.
 *
 * ============================================================================
 * ERROR MODEL
 * ============================================================================
 *
 * The grammar should allow the parser to identify structural errors such as:
 *
 *     <effect>
 *     <effect E
 *     <effect E,>
 *     with effects <effect E>
 *     with effects <effect E> {
 *     E =
 *
 * while semantic analysis handles:
 *
 *     unknown effect variable;
 *     duplicate binder;
 *     unbound variable;
 *     invalid substitution;
 *     invalid bound;
 *     incompatible effect;
 *     cyclic constraint;
 *     conflicting instantiation;
 *     invalid effect variance;
 *     invalid effect escape.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar performs no external activity.
 *
 * Parsing effect polymorphism MUST NOT:
 *
 *     - execute effects;
 *     - execute commands;
 *     - access files;
 *     - access networks;
 *     - discover hardware;
 *     - inspect credentials;
 *     - select devices;
 *     - allocate resources.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST contain no:
 *
 *     MAX_EFFECTS
 *     MAX_EFFECT_VARIABLES
 *     MAX_EFFECT_SET_SIZE
 *     MAX_HANDLERS
 *     MAX_PARAMETERS
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *
 * Numeric values appearing in tests or source programs remain program data,
 * not language limits.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The production test suite must cover at minimum:
 *
 * POSITIVE
 *
 *     <effect E>
 *     <effect E, effect F>
 *     <effect E: IO>
 *     <effect E: IO + Network>
 *     with effects <effect E> { E }
 *     with effects <effect E, effect F> { E, F }
 *     with effects <effect E> { E, IO }
 *     E = IO
 *     E = quantum::Measurement
 *     E : IO
 *     E : IO + Network
 *
 * NEGATIVE
 *
 *     <effect>
 *     <E>
 *     <effect E
 *     <effect E,>
 *     with effects <effect E>
 *     with effects <effect E> {
 *     E =
 *     E :
 *
 * SEMANTIC NEGATIVE
 *
 *     duplicate effect binder;
 *     unbound effect variable;
 *     unresolved effect;
 *     invalid substitution;
 *     conflicting substitution;
 *     cyclic effect constraint;
 *     effect-variable escape;
 *     invalid instantiation.
 *
 * BOUNDARY
 *
 *     one effect variable;
 *     many effect variables;
 *     one bound;
 *     many bounds;
 *     one effect member;
 *     many effect members;
 *     empty effect set;
 *     trailing commas;
 *     nested generic contexts.
 *
 * CROSS-DOMAIN
 *
 *     classical + IO;
 *     quantum + measurement;
 *     quantum + QEC;
 *     quantum + ZQN;
 *     HDL + timing;
 *     hardware + memory;
 *     distributed + network;
 *     AI + accelerator;
 *     security + network;
 *     hybrid classical/quantum effects.
 *
 * SCALABILITY
 *
 *     generated source containing large numbers of effect variables;
 *     large effect sets;
 *     large substitution sets;
 *     large constraint sets;
 *     deeply nested generic source;
 *     large mixed-domain programs.
 *
 * Tests may impose finite bounds for test execution, but those bounds MUST NOT
 * become language-level limits.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Adding this grammar is an additive language capability.
 *
 * Existing programs containing no effect-polymorphic syntax remain unaffected.
 *
 * If effect-polymorphic syntax becomes stable, compatibility documentation
 * must record:
 *
 *     grammar/compatibility/versions.md
 *     grammar/compatibility/migrations.md
 *     grammar/spec/compatibility.md
 *
 * No existing token is renamed by this file.
 *
 * No new lexer token is introduced by this file.
 *
 * ============================================================================
 * RUST INTEGRATION
 * ============================================================================
 *
 * This grammar contains no Rust.
 *
 * Rust consumers must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * The Rust implementation must remain safe Rust.
 *
 * Repository production code must reject unsafe Rust according to the
 * project's safety policy.
 *
 * A suitable crate-level enforcement remains:
 *
 *     #![forbid(unsafe_code)]
 *
 * where compatible with the affected crate/module architecture.
 *
 * ============================================================================
 * INTEGRATION WITH EXISTING EFFECT FILES
 * ============================================================================
 *
 * Existing canonical effect ownership remains:
 *
 *     effect-sets.g4
 *         ->
 *     ordinary effectReference/effectSet
 *
 *     effect-declarations.g4
 *         ->
 *     effect declarations and effect generics
 *
 *     effect-operations.g4
 *         ->
 *     effect operation use
 *
 *     effect-handling.g4
 *         ->
 *     handlers
 *
 *     capabilities.g4
 *         ->
 *     capability-related effect semantics/syntax
 *
 *     hardware.g4
 *         ->
 *     hardware-domain effect semantics/syntax
 *
 *     quantum.g4
 *         ->
 *     quantum-domain effect semantics/syntax
 *
 *     distributed.g4
 *         ->
 *     distributed-domain effect semantics/syntax
 *
 *     security.g4
 *         ->
 *     security-domain effect semantics/syntax
 *
 *     network.g4
 *         ->
 *     networking-domain effect semantics/syntax
 *
 *     custom-effects.g4
 *         ->
 *     extensible user/dialect effect syntax
 *
 * This file adds the missing polymorphic layer without taking ownership from
 * those files.
 *
 * ============================================================================
 * IMPORTANT EXISTING-REPOSITORY INTEGRATION ISSUE
 * ============================================================================
 *
 * The inspected repository currently contains legacy token-name references in
 * portions of the effects grammar such as:
 *
 *     K_EFFECT
 *     K_FN
 *     K_MUT
 *     K_WHERE
 *
 * while the canonical lexer vocabulary currently defines the corresponding
 * tokens as:
 *
 *     EFFECT
 *     FN
 *     MUT
 *     WHERE
 *
 * This file intentionally uses the canonical token names:
 *
 *     EFFECT
 *     LESS
 *     GREATER
 *     WITH
 *     EFFECTS
 *     COLON
 *     PLUS
 *     COMMA
 *     LBRACE
 *     RBRACE
 *     ASSIGN
 *
 * Therefore this file does not perpetuate the K_* token inconsistency.
 *
 * The existing K_* references must be corrected as a separate effects-grammar
 * conformance task before the complete effects subsystem can be declared
 * generated-parser production-ready.
 *
 * That correction MUST NOT require changing this file.
 *
 * ============================================================================
 * ZAMANI.G4 INTEGRATION
 * ============================================================================
 *
 * The canonical root grammar:
 *
 *     grammar/Zamani.g4
 *
 * should import or otherwise compose this grammar through the repository's
 * established modular ANTLR composition mechanism.
 *
 * It MUST NOT copy these rules into Zamani.g4.
 *
 * The root grammar should expose only the appropriate universal integration
 * entry points.
 *
 * Recommended conceptual composition:
 *
 *     Zamani.g4
 *          |
 *          +--> effects composition
 *                    |
 *                    +--> EffectPolymorphism
 *                              |
 *                              +--> EffectSets
 *                              +--> Core
 *                              +--> Types
 *                              +--> Expressions
 *
 * The exact root rule selected by Zamani.g4 should remain a composition
 * decision, not a duplication of this grammar.
 *
 * ============================================================================
 * FUNCTION INTEGRATION
 * ============================================================================
 *
 * Function grammar should consume:
 *
 *     effectPolymorphicParameterList
 *
 * and/or:
 *
 *     effectPolymorphicClause
 *
 * at the function-signature boundary.
 *
 * It must not redefine the rules here.
 *
 * Conceptually:
 *
 *     function signature
 *         ->
 *     ordinary type/generic parameters
 *         +
 *     optional effect polymorphism
 *         +
 *     effect qualification
 *
 * ============================================================================
 * TYPE INTEGRATION
 * ============================================================================
 *
 * Type grammar may consume:
 *
 *     effectPolymorphicTypeQualification
 *
 * where the normative type specification permits effect-qualified types.
 *
 * It must continue to own:
 *
 *     typeExpression
 *
 * This file never becomes a second type grammar.
 *
 * ============================================================================
 * SEMANTIC INTEGRATION
 * ============================================================================
 *
 * The semantic layer must establish one canonical effect-variable model.
 *
 * It should conceptually track:
 *
 *     EffectVariableId
 *     EffectVariableScope
 *     EffectVariableBinding
 *     EffectBound
 *     EffectSubstitution
 *     EffectConstraint
 *
 * These are semantic concepts, not parser-level machine resources.
 *
 * ============================================================================
 * CANONICAL QUANTUM BOUNDARY
 * ============================================================================
 *
 * No rule here creates:
 *
 *     QuantumGate
 *     PhysicalQubit
 *     QubitId
 *     QuantumDevice
 *     QPU
 *     topology
 *
 * Quantum effects eventually lower through:
 *
 *     quantum::ir
 *
 * exactly once.
 *
 * ============================================================================
 * FINAL INVARIANTS
 * ============================================================================
 *
 * 1. Effect polymorphism is open-world.
 *
 * 2. Effect variables use canonical identifiers.
 *
 * 3. No new lexer authority is introduced.
 *
 * 4. No new `forall` keyword is introduced locally.
 *
 * 5. Ordinary effect references remain owned by EffectSets.
 *
 * 6. Ordinary effect declarations remain owned by EffectDeclarations.
 *
 * 7. Ordinary effect operations remain owned by EffectOperations.
 *
 * 8. Effect handlers remain owned by EffectHandling.
 *
 * 9. Type syntax remains owned by the canonical type grammar.
 *
 * 10. Expression syntax remains owned by the canonical expression grammar.
 *
 * 11. Semantic inference remains outside the grammar.
 *
 * 12. Effect normalization remains outside the grammar.
 *
 * 13. Effect substitution remains outside the grammar.
 *
 * 14. Capability resolution remains outside the grammar.
 *
 * 15. Resource allocation remains outside the grammar.
 *
 * 16. Hardware realization remains outside the grammar.
 *
 * 17. Quantum semantics remain downstream.
 *
 * 18. quantum::ir remains the canonical quantum semantic boundary.
 *
 * 19. QEC is not duplicated.
 *
 * 20. ZQN is not duplicated.
 *
 * 21. Routing is not duplicated.
 *
 * 22. Scheduling is not duplicated.
 *
 * 23. No hardware count is hard-coded.
 *
 * 24. No machine topology is hard-coded.
 *
 * 25. No artificial effect-count limit is hard-coded.
 *
 * 26. Parse order remains deterministic.
 *
 * 27. Source order remains observable to downstream tooling.
 *
 * 28. The grammar contains no embedded Rust.
 *
 * 29. The Rust implementation remains safe Rust 1.97/1.97.1.
 *
 * 30. The same source semantics can be lowered across classical, quantum,
 *     HDL, accelerator, distributed, AI, networking, security and future
 *     targets.
 *
 * ============================================================================
 * FINAL ARCHITECTURE
 * ============================================================================
 *
 *                 Zamani Source
 *                       |
 *                       v
 *             Effect Polymorphism
 *                       |
 *                       v
 *                Frontend AST
 *                       |
 *                       v
 *              Effect/Type Analysis
 *                       |
 *             +---------+---------+
 *             |                   |
 *             v                   v
 *       Effect Variables     Concrete Effects
 *             |                   |
 *             +---------+---------+
 *                       |
 *                       v
 *               Semantic Effects
 *                       |
 *             +---------+---------+
 *             |         |         |
 *             v         v         v
 *         Classical  quantum::ir  HDL
 *             |         |         |
 *             +---------+---------+
 *                       |
 *                       v
 *                  Optimization
 *                       |
 *               Routing/Scheduling
 *                       |
 *                QEC/Resilience/ZQN
 *                       |
 *                       v
 *                      HAL
 *                       |
 *                       v
 *                Target Realization
 *
 * Effect polymorphism therefore contributes to:
 *
 *     Program Once
 *       ->
 *     Compile Once
 *       ->
 *     Run Everywhere
 *       ->
 *     Run Anywhere
 *       ->
 *     Run Forever
 *
 * without turning the source grammar into a description of today's hardware.
 *
 * ============================================================================
 */