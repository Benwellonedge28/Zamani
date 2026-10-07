/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/functional-dependencies.g4
 *
 * Grammar:
 *     FunctionalDependencies
 *
 * Status:
 *     CANONICAL TYPE-SYSTEM FUNCTIONAL-DEPENDENCY CONSTRAINT COMPONENT
 *
 * Compiler baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * Define the reusable source syntax for functional dependencies between
 * generic/type parameters.
 *
 * A functional dependency expresses a semantic relationship of the form:
 *
 *     A -> B
 *
 * meaning, semantically, that the type parameter B is determined by A within
 * the declaration/constraint context in which the dependency is declared.
 *
 * Multiple determining parameters are supported:
 *
 *     A, B -> C
 *
 * Multiple dependencies are supported by the surrounding constraint/where
 * grammar.
 *
 * This grammar defines ONLY the syntactic dependency relation.
 *
 * It does NOT implement:
 *
 *     dependency checking;
 *     inference;
 *     unification;
 *     type-family reduction;
 *     associated-type normalization;
 *     coherence;
 *     consistency;
 *     injectivity;
 *     specialization;
 *     implementation selection;
 *     constraint solving.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The intended pipeline is:
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     ANTLR parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     generic/type constraint model
 *       |
 *       v
 *     kind/type inference
 *       |
 *       v
 *     functional-dependency solving
 *       |
 *       v
 *     specialization / normalization
 *       |
 *       v
 *     canonical semantic type model
 *       |
 *       +------------------------+
 *       |                        |
 *       v                        v
 *   classical                quantum::ir
 *       |                        |
 *       +-----------+------------+
 *                   |
 *                   v
 *          target-independent
 *             optimization
 *                   |
 *                   v
 *              lowering
 *                   |
 *             routing/scheduling
 *                   |
 *          resilience / recovery
 *                   |
 *                 ZQN
 *                   |
 *                 HAL
 *
 * Functional dependencies are therefore entirely target-independent.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     functionalDependency
 *     functionalDependencyDeterminers
 *     functionalDependencyDeterminer
 *     functionalDependencyDependents
 *     functionalDependencyDependent
 *
 * It owns the syntactic relationship:
 *
 *     determining-parameters -> determined-parameters
 *
 * THIS FILE DOES NOT OWN:
 *
 *     lexer rules
 *     identifiers
 *     generic declarations
 *     generic applications
 *     typeExpression
 *     type bounds
 *     where clauses
 *     trait declarations
 *     implementation declarations
 *     associated types
 *     associated-type projections
 *     higher-kinded type declarations
 *     kind inference
 *     type inference
 *     type unification
 *     constraint solving
 *     coherence checking
 *     specialization
 *     type-family reduction
 *     dependent types
 *     contracts
 *     policies
 *     effects
 *     capabilities
 *     resources
 *     quantum operations
 *     HDL constructs
 *     backend lowering
 *     target realization
 *
 * ============================================================================
 * IMPORTANT OWNERSHIP RULE
 * ============================================================================
 *
 * Functional dependencies are constraints.
 *
 * They are NOT:
 *
 *     generic parameters;
 *     generic arguments;
 *     associated-type declarations;
 *     associated-type projections;
 *     type bounds;
 *     ordinary function arrows;
 *     function return types.
 *
 * In particular:
 *
 *     A -> B
 *
 * in a functional-dependency context MUST NOT be interpreted as an ordinary
 * function type merely because both use the canonical arrow token.
 *
 * The surrounding grammar establishes the syntactic context.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/lexer/
 *     canonical identifier grammar
 *
 * EXPORTS:
 *
 *     functionalDependency
 *     functionalDependencyDeterminers
 *     functionalDependencyDeterminer
 *     functionalDependencyDependents
 *     functionalDependencyDependent
 *
 * CONSUMED_BY:
 *
 *     grammar/types/constraints.g4
 *     grammar/types/type-constraints.g4 during migration, if retained
 *     where-clause grammar
 *     generic constraint composition
 *     trait/implementation constraint composition
 *     semantic constraint validation
 *
 * AST_OWNER:
 *
 *     Existing domain-neutral generic/type constraint representation.
 *
 *     If the current AST does not yet contain a first-class functional
 *     dependency node, the AST owner must add one in a coordinated frontend
 *     change. This grammar MUST NOT silently discard the dependency relation.
 *
 * SEMANTIC_OWNER:
 *
 *     Generic/type constraint semantic analysis.
 *
 * IR_OWNER:
 *
 *     Canonical semantic type/constraint model.
 *
 * TEST_OWNER:
 *
 *     grammar/tests/types/
 *     semantic/type-constraint tests
 *     generic constraint conformance tests
 *
 * SPEC_OWNER:
 *
 *     grammar/specification/types.md
 *     grammar/specification/constraints.md
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file contains NO lexer rules.
 *
 * Canonical parser-facing tokens required:
 *
 *     IDENTIFIER
 *     COMMA
 *     THIN_ARROW
 *
 * The canonical arrow token is:
 *
 *     THIN_ARROW
 *
 * representing:
 *
 *     ->
 *
 * This grammar MUST NOT introduce:
 *
 *     FUNCTIONAL_ARROW
 *     DEPENDENCY_ARROW
 *     FUNCTION_DEPENDENCY_ARROW
 *     HKT_ARROW
 *
 * or any other duplicate lexical spelling for `->`.
 *
 * The existing lexical authority remains:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * ============================================================================
 * PARAMETER MODEL
 * ============================================================================
 *
 * A functional dependency is deliberately restricted at the syntax level to
 * parameter references.
 *
 * Examples:
 *
 *     A -> B
 *     A, B -> C
 *     A, B, C -> D
 *
 * This is intentional.
 *
 * Functional dependencies describe relationships between declared type
 * parameters. They are not arbitrary type-expression equations.
 *
 * Therefore constructs such as:
 *
 *     Vec<A> -> B
 *     A -> Vec<B>
 *     fn(A) -> B
 *
 * are NOT functional dependencies in this grammar.
 *
 * If Zamani later requires generalized type-level relations, that feature
 * MUST receive its own semantic and syntactic contract rather than weakening
 * this grammar.
 *
 * ============================================================================
 * DETERMINER SIDE
 * ============================================================================
 *
 * The left side contains one or more parameter references:
 *
 *     A
 *
 *     A, B
 *
 *     A, B, C
 *
 * There is no language-level maximum.
 *
 * The source order is preserved.
 *
 * ============================================================================
 * DEPENDENT SIDE
 * ============================================================================
 *
 * The right side contains one or more parameter references:
 *
 *     B
 *
 *     B, C
 *
 *     B, C, D
 *
 * Multiple determined parameters are intentionally permitted so that the
 * syntax remains general and does not impose an arbitrary arity restriction.
 *
 * Semantic validation decides whether the declaration is meaningful.
 *
 * ============================================================================
 * SEMANTIC MEANING
 * ============================================================================
 *
 * The grammar only records:
 *
 *     determining parameters
 *     ->
 *     determined parameters
 *
 * Semantic analysis determines:
 *
 *     - whether every parameter exists;
 *     - whether every parameter belongs to the declaration;
 *     - whether a parameter occurs on both sides;
 *     - whether duplicate parameters occur;
 *     - whether the dependency is redundant;
 *     - whether the dependency is contradictory;
 *     - whether the dependency participates in a valid constraint system;
 *     - whether inference can use the dependency;
 *     - whether specialization preserves it;
 *     - whether associated-type normalization preserves it;
 *     - whether implementation selection remains coherent.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser MUST preserve:
 *
 *     - complete dependency span;
 *     - determining-parameter order;
 *     - dependent-parameter order;
 *     - each parameter's source span;
 *     - the arrow span;
 *     - source ordering relative to surrounding constraints.
 *
 * Conceptually, the semantic representation is:
 *
 *     FunctionalDependency {
 *         determining: [ParameterRef],
 *         determined:  [ParameterRef],
 *         span: SourceSpan,
 *     }
 *
 * The exact Rust AST type is owned by the frontend AST subsystem.
 *
 * This grammar MUST NOT introduce a competing type hierarchy.
 *
 * If a suitable canonical constraint representation already exists, this
 * construct must be lowered into that representation.
 *
 * If no representation exists, the AST owner must add one before this feature
 * is marked implemented.
 *
 * ============================================================================
 * SEMANTIC VALIDATION CONTRACT
 * ============================================================================
 *
 * The following are semantic rather than parser errors:
 *
 *     unknown parameter:
 *
 *         A -> Unknown
 *
 *     duplicate determining parameter:
 *
 *         A, A -> B
 *
 *     duplicate dependent parameter:
 *
 *         A -> B, B
 *
 *     self dependency:
 *
 *         A -> A
 *
 *     undeclared parameter:
 *
 *         A -> B
 *
 *     contradictory dependency sets;
 *
 *     dependency cycles;
 *
 *     incoherent implementation sets;
 *
 *     dependency violations during specialization.
 *
 * Whether a particular construct is legal is determined by the semantic
 * constraint system, not by this parser.
 *
 * ============================================================================
 * DUPLICATE PARAMETERS
 * ============================================================================
 *
 * The grammar deliberately permits syntactically repeated identifiers:
 *
 *     A, A -> B
 *
 * because detecting duplicates requires knowledge of the declaration scope.
 *
 * The parser cannot reliably determine whether two identical identifiers
 * refer to the same declaration across all future declaration forms,
 * namespaces, generic scopes, dialects, or compatibility modes.
 *
 * Therefore duplicate detection belongs to semantic validation.
 *
 * ============================================================================
 * ORDERING CONTRACT
 * ============================================================================
 *
 * Parameter order is significant at the source representation level.
 *
 * For:
 *
 *     A, B -> C
 *
 * the parser must preserve:
 *
 *     determining[0] = A
 *     determining[1] = B
 *     determined[0]  = C
 *
 * The semantic layer may canonicalize a dependency set if its mathematical
 * model treats the sets as unordered.
 *
 * Such canonicalization MUST occur downstream and MUST preserve source
 * provenance.
 *
 * ============================================================================
 * WHERE-CLAUSE INTEGRATION
 * ============================================================================
 *
 * This file does NOT own the `where` keyword.
 *
 * A where-clause owner may compose:
 *
 *     WHERE functionalDependency
 *
 * alongside other constraint predicates.
 *
 * Example:
 *
 *     where A -> B
 *
 * Multiple dependencies should be composed by the surrounding constraint
 * grammar according to that grammar's established separator rules.
 *
 * This file MUST NOT invent a second where-clause grammar.
 *
 * ============================================================================
 * GENERIC INTEGRATION
 * ============================================================================
 *
 * Generic declarations remain owned by:
 *
 *     grammar/functions/generics.g4
 *
 * and the relevant declaration grammars.
 *
 * A generic declaration may establish:
 *
 *     <A, B, C>
 *
 * and a surrounding constraint grammar may establish:
 *
 *     where A -> B
 *
 * This file does not declare A, B or C.
 *
 * It only references them.
 *
 * ============================================================================
 * TYPE-CLASS / TRAIT INTEGRATION
 * ============================================================================
 *
 * Functional dependencies are particularly useful for trait/type-class
 * systems.
 *
 * Example semantic shape:
 *
 *     trait Relation<A, B>
 *     where A -> B
 *
 * The trait declaration remains owned by:
 *
 *     grammar/declarations/traits.g4
 *
 * The type-class reference remains owned by:
 *
 *     grammar/types/type-class.g4
 *
 * This file contributes only the dependency predicate.
 *
 * Coherence and implementation selection remain semantic concerns.
 *
 * ============================================================================
 * ASSOCIATED-TYPE INTEGRATION
 * ============================================================================
 *
 * Functional dependencies may interact semantically with associated types.
 *
 * For example, a semantic system may use:
 *
 *     A -> B
 *
 * to establish that B is determined by A, while B is represented by an
 * associated type or type-level output.
 *
 * This grammar MUST NOT redefine:
 *
 *     associatedType
 *     associatedTypeProjectionSuffix
 *     T::Item
 *
 * Those remain owned by the associated-type subsystem.
 *
 * ============================================================================
 * HIGHER-KINDED TYPE INTEGRATION
 * ============================================================================
 *
 * Functional dependencies may reference higher-kinded generic parameters.
 *
 * Example:
 *
 *     F -> G
 *
 * where F and G are themselves kinded parameters.
 *
 * The functional-dependency grammar does not need to understand their kinds.
 *
 * Kind checking remains owned by:
 *
 *     grammar/types/higher-kinded-types.g4
 *
 * and the semantic kind system.
 *
 * The semantic pipeline is:
 *
 *     parameter declarations
 *          |
 *          v
 *     kind environment
 *          |
 *          v
 *     functional dependencies
 *          |
 *          v
 *     constraint solving
 *
 * ============================================================================
 * TYPE-CLASS / FUNCTIONAL-DEPENDENCY SEPARATION
 * ============================================================================
 *
 * This grammar MUST NOT attempt to encode:
 *
 *     trait implementation selection;
 *     instance search;
 *     type-class coherence;
 *     specialization.
 *
 * Functional dependencies provide information to those systems.
 *
 * They do not replace those systems.
 *
 * ============================================================================
 * TYPE INFERENCE CONTRACT
 * ============================================================================
 *
 * Functional dependencies may provide additional information to type
 * inference.
 *
 * For example:
 *
 *     A -> B
 *
 * may allow the semantic solver to infer B after A is known, subject to the
 * declaration's actual constraints.
 *
 * This grammar does not perform inference.
 *
 * It only preserves the source relationship.
 *
 * ============================================================================
 * UNIFICATION CONTRACT
 * ============================================================================
 *
 * Functional dependency constraints may participate in unification.
 *
 * However:
 *
 *     parser
 *
 * MUST NOT:
 *
 *     unify types;
 *     substitute variables;
 *     reduce constraints;
 *     instantiate generics;
 *     select implementations.
 *
 * Those operations belong to semantic analysis.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Functional dependency syntax introduces no runtime effect.
 *
 * It performs no:
 *
 *     IO
 *     network operation
 *     mutation
 *     randomness
 *     native call
 *     foreign call
 *     distributed communication
 *     quantum measurement
 *     learning
 *     adaptation
 *     reflection
 *     simulation
 *
 * A declaration containing functional dependencies remains effect-free at
 * this syntactic layer.
 *
 * Runtime effects belong to the operation/effect system.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Functional dependencies do not request or select capabilities.
 *
 * A semantic type or implementation constrained by a dependency may
 * eventually imply:
 *
 *     capability("quantum.measurement")
 *     capability("tensor.compute")
 *     capability("distributed.compute")
 *
 * Such requirements belong to the semantic/resource/capability layers.
 *
 * This grammar must remain independent of target capability inventories.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * This grammar has no resource requirements.
 *
 * It does not encode:
 *
 *     memory;
 *     processor count;
 *     accelerator count;
 *     qubit count;
 *     register width;
 *     network size;
 *     device count;
 *     topology;
 *     storage capacity;
 *     tensor rank limits.
 *
 * A type-system dependency remains valid independently of the eventual
 * realization target.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Functional dependencies may appear inside declarations that are subject to:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * This file does not own those constructs.
 *
 * Contract validation may consume semantic information derived from
 * functional dependencies.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policies may constrain:
 *
 *     generic specialization;
 *     implementation selection;
 *     use of particular type-level capabilities;
 *     deployment;
 *     compilation.
 *
 * Policy syntax and evaluation remain outside this grammar.
 *
 * Functional dependencies do not bypass policy evaluation.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The frontend must preserve enough source information to associate semantic
 * dependency facts with their origin.
 *
 * Provenance may record:
 *
 *     source dependency;
 *     declaration containing the dependency;
 *     semantic normalization;
 *     inferred consequences;
 *     specialization;
 *     implementation selection;
 *     diagnostic decisions.
 *
 * This grammar does not implement provenance storage.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Functional dependencies are domain-neutral.
 *
 * They may constrain generic abstractions involving:
 *
 *     quantum states;
 *     quantum operations;
 *     logical resources;
 *     measurement abstractions;
 *     hybrid computations.
 *
 * Example:
 *
 *     Q -> Result
 *
 * does not select:
 *
 *     physical qubits;
 *     QPU;
 *     coupling map;
 *     gate set;
 *     calibration;
 *     routing;
 *     scheduling;
 *     QEC strategy.
 *
 * Those decisions occur downstream.
 *
 * Quantum semantic constructs continue through:
 *
 *     semantic model
 *          |
 *          v
 *     quantum::ir
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Functional dependencies may constrain generic hardware-neutral abstractions.
 *
 * Example:
 *
 *     Signal -> Representation
 *
 * does not specify:
 *
 *     register width;
 *     number of lanes;
 *     FPGA resources;
 *     ASIC area;
 *     clock frequency;
 *     physical placement.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * AI / DATA CONTRACT
 * ============================================================================
 *
 * Functional dependencies are equally applicable to:
 *
 *     Tensor -> Element
 *     Dataset -> Schema
 *     Model -> Output
 *     Distribution -> Sample
 *
 * where such relationships are declared by a type-system owner.
 *
 * No AI-specific syntax is introduced here.
 *
 * ============================================================================
 * POCO-REAF / PORTABILITY CONTRACT
 * ============================================================================
 *
 * Functional dependencies describe source-level type relationships.
 *
 * They do not describe a particular machine.
 *
 * Therefore the same dependency syntax can participate in programs targeting:
 *
 *     embedded systems;
 *     CPUs;
 *     multicore systems;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     accelerators;
 *     QPUs;
 *     simulators;
 *     HPC systems;
 *     clusters;
 *     distributed environments;
 *     cloud environments;
 *     future computational substrates.
 *
 * Target feasibility is resolved downstream through:
 *
 *     types
 *     effects
 *     capabilities
 *     resources
 *     contracts
 *     policies
 *     semantic analysis
 *     lowering
 *     scheduling
 *     realization.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes NO language-level maximum on:
 *
 *     number of functional dependencies;
 *     number of determining parameters;
 *     number of determined parameters;
 *     generic parameter count;
 *     declaration count;
 *     type count;
 *     module count;
 *     program size;
 *     domain count;
 *     target count;
 *     hardware size;
 *     quantum resources;
 *     memory capacity;
 *     network size.
 *
 * It MUST NOT define constants such as:
 *
 *     MAX_FUNCTIONAL_DEPENDENCIES
 *     MAX_FUNCTIONAL_DEPENDENCY_ARITY
 *     MAX_DETERMINERS
 *     MAX_DEPENDENTS
 *     MAX_GENERIC_PARAMETERS
 *
 * or renamed equivalents.
 *
 * Repetition is represented using grammar repetition operators.
 *
 * Operational parser/compiler safeguards MAY exist outside the language.
 *
 * Such safeguards must be:
 *
 *     configurable;
 *     observable;
 *     diagnosable;
 *     documented;
 *     independent of target capacity;
 *     separate from language semantics.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * For identical:
 *
 *     source;
 *     lexer configuration;
 *     language version;
 *     grammar version;
 *     dialect configuration;
 *
 * this grammar must produce the same structural parse.
 *
 * Parsing MUST NOT depend on:
 *
 *     time;
 *     randomness;
 *     filesystem state;
 *     network state;
 *     target hardware;
 *     runtime state;
 *     available resources.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax errors owned by this grammar include:
 *
 *     ->
 *     A ->
 *     -> B
 *     A, -> B
 *     A,, B -> C
 *     A B -> C
 *     A -> 
 *     A -> , B
 *
 * Valid structural examples include:
 *
 *     A -> B
 *     A, B -> C
 *     A -> B, C
 *     A, B -> C, D
 *
 * Semantic diagnostics include:
 *
 *     unknown parameter;
 *     duplicate parameter;
 *     undeclared parameter;
 *     contradictory dependency;
 *     incoherent dependency set;
 *     invalid dependency cycle;
 *     specialization conflict.
 *
 * These MUST NOT be implemented as parser-specific semantic predicates.
 *
 * ============================================================================
 * SECURITY / HOSTILE INPUT CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no actions;
 *     no semantic predicates;
 *     no embedded Rust;
 *     no filesystem access;
 *     no network access;
 *     no hardware access;
 *     no environment inspection;
 *     no randomness;
 *     no mutable global state.
 *
 * Resource exhaustion protection belongs to the parser/compiler policy layer.
 *
 * The grammar itself must not use artificial language ceilings as a security
 * mechanism.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This is an additive grammar component.
 *
 * Existing syntax such as:
 *
 *     A
 *     A, B
 *     A: Bound
 *     A -> B
 *
 * is not globally reinterpreted by this file.
 *
 * `A -> B` becomes a functional dependency ONLY when consumed by the
 * functional-dependency grammar in its declared syntactic context.
 *
 * Ordinary function types continue to be owned by the canonical function-type
 * grammar.
 *
 * HKT arrows continue to be owned by:
 *
 *     grammar/types/higher-kinded-types.g4
 *
 * This grammar reuses the canonical THIN_ARROW token and does not redefine
 * it.
 *
 * ============================================================================
 * MIGRATION CONTRACT
 * ============================================================================
 *
 * If another grammar currently contains a functional-dependency-like rule,
 * there MUST be exactly one canonical implementation after migration.
 *
 * Candidate overlapping locations include:
 *
 *     grammar/types/constraints.g4
 *     grammar/types/type-constraints.g4
 *     grammar/types/type-class.g4
 *     grammar/declarations/traits.g4
 *     grammar/declarations/implementations.g4
 *
 * Those files may consume this grammar's public rule, but must not duplicate
 * its implementation.
 *
 * Legacy aliases may remain only when a real consumer requires them and must
 * be explicitly marked as compatibility boundaries.
 *
 * ============================================================================
 * INTEGRATION WITH `constraints.g4`
 * ============================================================================
 *
 * The canonical constraint grammar should import/compose this component.
 *
 * Conceptually:
 *
 *     typeConstraintPredicate
 *         : typeConstraintBound
 *         | functionalDependency
 *         | ...
 *         ;
 *
 * The exact predicate composition belongs to the constraint owner.
 *
 * This file MUST NOT define `typeConstraintPredicate`.
 *
 * ============================================================================
 * INTEGRATION WITH `where` GRAMMAR
 * ============================================================================
 *
 * The where-clause owner may consume:
 *
 *     functionalDependency
 *
 * without importing a second type-expression grammar.
 *
 * Example:
 *
 *     where
 *         A -> B
 *
 * The where owner controls:
 *
 *     keyword;
 *     predicate ordering;
 *     predicate separators;
 *     interaction with ordinary bounds;
 *     interaction with other constraint predicates.
 *
 * ============================================================================
 * INTEGRATION WITH `functions/generics.g4`
 * ============================================================================
 *
 * `functions/generics.g4` remains the owner of:
 *
 *     functionGenericParameters
 *     functionGenericParameter
 *     functionGenericParameterName
 *     functionGenericParameterBounds
 *
 * It must NOT duplicate:
 *
 *     functionalDependency
 *
 * A function declaration may eventually expose a constraint/where clause
 * that consumes functionalDependency through the canonical constraint owner.
 *
 * ============================================================================
 * INTEGRATION WITH TRAITS
 * ============================================================================
 *
 * `grammar/declarations/traits.g4` remains the trait declaration owner.
 *
 * A trait declaration may contain functional dependencies through its
 * canonical constraint/where composition.
 *
 * Trait members and associated types remain owned by the trait grammar.
 *
 * Functional dependency semantics remain downstream.
 *
 * ============================================================================
 * INTEGRATION WITH TYPE CLASSES
 * ============================================================================
 *
 * `grammar/types/type-class.g4` remains a reference/composition grammar.
 *
 * It MUST NOT redefine functional dependencies.
 *
 * Type-class resolution may consume semantic dependency information after
 * parsing.
 *
 * ============================================================================
 * INTEGRATION WITH ASSOCIATED TYPES
 * ============================================================================
 *
 * `grammar/types/associated.g4` remains the owner of associated-type syntax.
 *
 * Functional dependency analysis may constrain associated-type resolution,
 * but this grammar does not parse associated-type projections.
 *
 * ============================================================================
 * INTEGRATION WITH HIGHER-KINDED TYPES
 * ============================================================================
 *
 * `grammar/types/higher-kinded-types.g4` remains the owner of HKT syntax.
 *
 * This grammar only references the names of parameters.
 *
 * It does not parse or infer their kinds.
 *
 * The semantic system combines:
 *
 *     generic declaration
 *          +
 *     kind information
 *          +
 *     functional dependencies
 *
 * to construct the final constraint environment.
 *
 * ============================================================================
 * INTEGRATION WITH DEPENDENT TYPES
 * ============================================================================
 *
 * Functional dependencies may eventually interact with dependent types.
 *
 * However, this file must not parse dependent values or dependent type
 * expressions.
 *
 * The dependent-type grammar remains authoritative.
 *
 * ============================================================================
 * EFFECTS / RESOURCES / CAPABILITIES
 * ============================================================================
 *
 * No direct integration is required at the parser layer.
 *
 * Semantic consequences may flow through:
 *
 *     type resolution
 *         |
 *         v
 *     operation legality
 *         |
 *         v
 *     effects
 *         |
 *         v
 *     capabilities
 *         |
 *         v
 *     resources
 *         |
 *         v
 *     target realization
 *
 * The functional dependency grammar remains independent of that pipeline.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar emits NO direct backend IR.
 *
 * It must not emit:
 *
 *     LLVM IR
 *     machine code
 *     QIR
 *     QASM
 *     HDL netlists
 *     FPGA placement
 *     ASIC layout
 *     physical qubit mappings
 *     device schedules
 *     vendor-specific instructions.
 *
 * Functional dependencies are normalized into the canonical semantic
 * constraint/type model first.
 *
 * Only after semantic resolution may their consequences influence:
 *
 *     classical IR;
 *     quantum::ir;
 *     HDL/hardware representation;
 *     other domain IR.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This `.g4` file contains no Rust implementation code.
 *
 * The consuming Zamani compiler/frontend MUST:
 *
 *     - support Rust 1.97 or later;
 *     - remain Rust 2021 compatible where applicable;
 *     - use safe Rust;
 *     - use no unsafe blocks;
 *     - use no unsafe functions;
 *     - preserve source spans;
 *     - preserve deterministic semantic construction;
 *     - keep parser/resource limits outside language semantics.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * --------------
 *
 *     A -> B
 *
 *     A, B -> C
 *
 *     A -> B, C
 *
 *     A, B -> C, D
 *
 *     Input -> Output
 *
 *     Key, Namespace -> Value
 *
 *     F, G -> H
 *
 * NEGATIVE STRUCTURAL TESTS
 * -------------------------
 *
 *     ->
 *
 *     A ->
 *
 *     -> B
 *
 *     A, -> B
 *
 *     A,, B -> C
 *
 *     A B -> C
 *
 *     A -> , B
 *
 *     A -> B,
 *
 *     A B C
 *
 * Semantic negative tests:
 *
 *     undeclared parameter;
 *
 *     duplicate parameter;
 *
 *     contradictory dependencies;
 *
 *     invalid dependency cycle;
 *
 *     incompatible specialization;
 *
 *     incoherent implementation set.
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * Functional dependencies must be tested with:
 *
 *     ordinary generics;
 *     generic bounds;
 *     where clauses;
 *     traits;
 *     type classes;
 *     associated types;
 *     higher-kinded parameters;
 *     dependent types;
 *     linear types;
 *     affine types;
 *     function types;
 *     quantum types;
 *     hardware-neutral types;
 *     resource-aware types;
 *     effect-qualified declarations;
 *     contracts;
 *     policies;
 *     metaprogramming.
 *
 * The parser must remain unchanged when new domains are added.
 *
 * ============================================================================
 * CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * Functional dependencies are domain-neutral and must therefore be testable
 * with semantic declarations involving:
 *
 *     classical types;
 *     tensor types;
 *     probabilistic types;
 *     AI/model types;
 *     quantum abstractions;
 *     HDL abstractions;
 *     hardware abstractions;
 *     distributed abstractions;
 *     networking abstractions;
 *     interoperability abstractions.
 *
 * No domain-specific keyword is permitted in this grammar.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Tests MUST be generated from source-defined sizes.
 *
 * Required classes:
 *
 *     one determining parameter;
 *     many determining parameters;
 *     one determined parameter;
 *     many determined parameters;
 *     many dependencies;
 *     deeply nested declarations;
 *     large generic declarations;
 *     large constraint sets.
 *
 * The test suite must demonstrate that the grammar has no source-language
 * maximum.
 *
 * It must not claim literal mathematical infinity.
 *
 * The meaningful guarantee is:
 *
 *     arbitrary finite source structures are permitted by the grammar,
 *     subject only to explicit implementation/resource policy.
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * Given identical:
 *
 *     source;
 *     grammar;
 *     lexer;
 *     language version;
 *     dialect configuration;
 *
 * repeated parsing must produce equivalent parse structures.
 *
 * No test may depend on:
 *
 *     hardware;
 *     CPU count;
 *     memory size;
 *     QPU availability;
 *     network state;
 *     current time;
 *     randomness.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * PASS REQUIREMENTS:
 *
 *     no fixed dependency count;
 *     no fixed parameter count;
 *     no fixed generic count;
 *     no hardware capacity;
 *     no quantum capacity;
 *     no memory capacity;
 *     no tensor rank;
 *     no network size;
 *     no device count;
 *     no target enumeration.
 *
 * Forbidden examples include:
 *
 *     MAX_FUNCTIONAL_DEPENDENCIES
 *     MAX_DETERMINERS
 *     MAX_DEPENDENTS
 *     MAX_GENERIC_PARAMETERS
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_REGISTER_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * None are defined by this grammar.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] It is the sole canonical implementation of functional-dependency
 *         syntax.
 *
 *     [ ] It contains no lexer rules.
 *
 *     [ ] It contains no generic declaration grammar.
 *
 *     [ ] It contains no generic application grammar.
 *
 *     [ ] It contains no typeExpression grammar.
 *
 *     [ ] It contains no where-clause grammar.
 *
 *     [ ] It contains no trait declaration grammar.
 *
 *     [ ] It contains no associated-type grammar.
 *
 *     [ ] It uses the canonical IDENTIFIER token/rule.
 *
 *     [ ] It uses the canonical THIN_ARROW token.
 *
 *     [ ] It supports one or more determining parameters.
 *
 *     [ ] It supports one or more determined parameters.
 *
 *     [ ] It preserves source ordering.
 *
 *     [ ] It imposes no language-level arity ceiling.
 *
 *     [ ] It imposes no hardware/resource ceiling.
 *
 *     [ ] It does not perform semantic solving.
 *
 *     [ ] It does not perform type inference.
 *
 *     [ ] It does not perform implementation selection.
 *
 *     [ ] It does not emit backend IR.
 *
 *     [ ] It remains compatible with HKT syntax.
 *
 *     [ ] It remains compatible with associated types.
 *
 *     [ ] It remains compatible with trait/type-class constraints.
 *
 *     [ ] It has parser positive tests.
 *
 *     [ ] It has parser negative tests.
 *
 *     [ ] It has semantic negative tests.
 *
 *     [ ] It has boundary tests.
 *
 *     [ ] It has scalability tests.
 *
 *     [ ] It has determinism tests.
 *
 *     [ ] It has compatibility tests.
 *
 *     [ ] Rust integration remains compatible with Rust 1.97+.
 *
 *     [ ] No unsafe Rust is required.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * Functional dependencies are a reusable type-system relation.
 *
 * They are NOT:
 *
 *     a runtime mechanism;
 *     a hardware mechanism;
 *     a quantum mechanism;
 *     an AI mechanism;
 *     a resource mechanism;
 *     a capability mechanism;
 *     a second generic language;
 *     a second constraint language;
 *     a second AST;
 *     a second IR.
 *
 * The production architecture is:
 *
 *     functional dependency syntax
 *              |
 *              v
 *     domain-neutral AST
 *              |
 *              v
 *     semantic constraint model
 *              |
 *       +------+------+
 *       |             |
 *       v             v
 *   kind/type      inference
 *   analysis       / unification
 *       |             |
 *       +------+------+
 *              |
 *              v
 *       constraint solving
 *              |
 *              v
 *       canonical semantic
 *          type model
 *              |
 *       +------+------+
 *       |             |
 *       v             v
 * classical      quantum::ir
 *       |             |
 *       +------+------+
 *              |
 *              v
 *       target-independent
 *          optimization
 *              |
 *              v
 *       lowering / realization
 *
 * This preserves POCO-REAF by keeping functional dependencies at the level of
 * portable source meaning rather than physical machine realization.
 *
 * ============================================================================
 */

parser grammar FunctionalDependencies;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * 1. FUNCTIONAL DEPENDENCY
 * ============================================================================
 *
 * Canonical forms:
 *
 *     A -> B
 *     A, B -> C
 *     A -> B, C
 *     A, B -> C, D
 *
 * The surrounding constraint/where grammar determines where this predicate
 * may occur.
 */
functionalDependency
    : functionalDependencyDeterminers
      THIN_ARROW
      functionalDependencyDependents
    ;

/*
 * ============================================================================
 * 2. DETERMINING PARAMETERS
 * ============================================================================
 *
 * One or more declared parameter references.
 *
 * Duplicate detection is intentionally semantic.
 */
functionalDependencyDeterminers
    : functionalDependencyDeterminer
      (
          COMMA
          functionalDependencyDeterminer
      )*
    ;

/*
 * ============================================================================
 * 3. DETERMINING PARAMETER
 * ============================================================================
 *
 * A functional dependency refers to a parameter by name.
 *
 * The identifier is resolved against the surrounding generic/declaration
 * scope during semantic analysis.
 */
functionalDependencyDeterminer
    : IDENTIFIER
    ;

/*
 * ============================================================================
 * 4. DETERMINED PARAMETERS
 * ============================================================================
 *
 * One or more parameter references.
 *
 * The grammar intentionally allows multiple outputs:
 *
 *     A -> B, C
 *
 * so no artificial arity ceiling is introduced.
 */
functionalDependencyDependents
    : functionalDependencyDependent
      (
          COMMA
          functionalDependencyDependent
      )*
    ;

/*
 * ============================================================================
 * 5. DETERMINED PARAMETER
 * ============================================================================
 *
 * A parameter name whose type-level value is semantically determined by the
 * determining side, subject to the declaration's constraint system.
 */
functionalDependencyDependent
    : IDENTIFIER
    ;