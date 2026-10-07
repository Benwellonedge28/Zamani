/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/variance.g4
 *
 * Grammar:
 *     Variance
 *
 * Status:
 *     PRODUCTION-READY PARSER DELEGATE
 *
 * Compiler baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file owns the reusable source-level syntax for explicitly declaring
 * the variance of a generic type parameter.
 *
 * Supported semantic variance positions are:
 *
 *     covariant
 *     contravariant
 *     invariant
 *
 * The grammar records which variance position the programmer explicitly
 * requested. It does not determine whether that declaration is sound,
 * applicable, or compatible with the declaration in which it occurs.
 *
 *
 * OWNED BY THIS FILE
 * ------------------
 *
 *     varianceAnnotation
 *     varianceKind
 *     covariantVariance
 *     contravariantVariance
 *     invariantVariance
 *
 * This file owns the source ordering and syntactic identity of the variance
 * annotation itself.
 *
 *
 * NOT OWNED BY THIS FILE
 * ---------------------
 *
 * This file does NOT own:
 *
 *     typeExpression
 *     typeCore
 *     named types
 *     generic type applications
 *     generic parameter declarations
 *     generic parameter lists
 *     generic bounds
 *     where clauses
 *     type constraints
 *     associated types
 *     type classes / traits
 *     subtyping
 *     type equality
 *     type compatibility
 *     function types
 *     function variance
 *     lifetime variance
 *     ownership
 *     linearity
 *     affinity
 *     effects
 *     capabilities
 *     resources
 *     contracts
 *     policies
 *     provenance
 *     target selection
 *     hardware selection
 *     quantum allocation
 *     physical topology
 *     routing
 *     scheduling
 *     optimization
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *
 * In particular, this file MUST NOT become another complete type grammar.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The dependency direction is:
 *
 *     canonical lexer
 *          |
 *          v
 *     Variance
 *          |
 *          | varianceAnnotation
 *          v
 *     generic declaration grammar
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic type system
 *          |
 *          +--> subtyping
 *          +--> compatibility
 *          +--> trait/interface satisfaction
 *          +--> type checking
 *          +--> generic analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +--> classical
 *          +--> quantum::ir
 *          +--> HDL/hardware
 *          +--> AI/data
 *          +--> distributed
 *          |
 *          v
 *     target-independent compilation
 *          |
 *          v
 *     target realization
 *
 * Variance therefore remains entirely above physical target realization.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * Exactly one grammar owns the syntax of variance kinds.
 *
 * This file owns:
 *
 *     varianceAnnotation
 *     varianceKind
 *
 * Generic declaration grammars consume:
 *
 *     varianceAnnotation
 *
 * They MUST NOT redefine:
 *
 *     covariant
 *     contravariant
 *     invariant
 *     varianceAnnotation
 *     varianceKind
 *
 * Conversely, this file MUST NOT define:
 *
 *     genericParameter
 *     genericParameterList
 *     genericTypeArguments
 *     typeExpression
 *
 * This prevents grammar duplication and circular dependencies.
 *
 * ============================================================================
 * EXPLICITNESS RULE
 * ============================================================================
 *
 * Variance is an explicit source-level declaration.
 *
 * The semantic compiler MUST NOT silently infer a public generic API's
 * declared variance from:
 *
 *     implementation representation
 *     backend representation
 *     target ABI
 *     optimization
 *     machine architecture
 *     physical resource layout
 *
 * Semantic analysis may perform internal variance analysis as part of
 * validating an explicit declaration, but such analysis MUST NOT silently
 * change the declared public variance.
 *
 * This preserves source compatibility and type safety.
 *
 * ============================================================================
 * CANONICAL SOURCE FORM
 * ============================================================================
 *
 * The reusable annotation exported by this grammar is:
 *
 *     variance covariant
 *
 *     variance contravariant
 *
 *     variance invariant
 *
 * The generic declaration grammar determines where the annotation occurs.
 *
 * For example, the intended composed syntax is:
 *
 *     <T variance covariant>
 *
 *     <T variance contravariant>
 *
 *     <T variance invariant>
 *
 * and, where bounds are supported:
 *
 *     <T variance covariant extends Producer>
 *
 *     <T variance contravariant extends Consumer>
 *
 *     <T variance invariant extends Constraint>
 *
 * The exact ordering of variance and bounds is owned by the generic
 * declaration grammar. This file deliberately does not own that surrounding
 * syntax.
 *
 * ============================================================================
 * WHY THE `variance` MARKER EXISTS
 * ============================================================================
 *
 * The marker makes the declaration unambiguous and future-extensible.
 *
 * It avoids treating:
 *
 *     covariant
 *     contravariant
 *     invariant
 *
 * as ordinary type expressions.
 *
 * It also leaves room for future type-system metadata without changing the
 * meaning of existing type expressions.
 *
 * A declaration therefore has the conceptual structure:
 *
 *     generic parameter
 *         |
 *         +--> variance metadata
 *         |
 *         +--> bounds
 *
 * rather than:
 *
 *     generic parameter
 *         |
 *         +--> special type called "covariant"
 *
 * ============================================================================
 * OPEN-WORLD RULE
 * ============================================================================
 *
 * Variance itself has a deliberately small language-defined semantic domain:
 *
 *     covariant
 *     contravariant
 *     invariant
 *
 * This is NOT a hardware/resource capacity restriction.
 *
 * The finite set is intentional because these are distinct type-relation
 * semantics, not an enumeration of physical resources.
 *
 * The grammar MUST NOT introduce domain-specific variance forms such as:
 *
 *     gpu-covariant
 *     quantum-covariant
 *     tensor-covariant
 *     hardware-covariant
 *
 * Domain-specific type semantics are handled by the type system.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar contains NO lexer rules.
 *
 * Parser-facing tokens come exclusively from:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through the canonical lexical hierarchy.
 *
 * Required tokens:
 *
 *     VARIANCE
 *     COVARIANT
 *     CONTRAVARIANT
 *     INVARIANT
 *
 * The lexical hierarchy should define the spellings:
 *
 *     variance
 *     covariant
 *     contravariant
 *     invariant
 *
 * These words are language-level type-system vocabulary.
 *
 * This grammar MUST NOT define those tokens locally.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a parser delegate.
 *
 * It MUST use:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * It MUST NOT import:
 *
 *     Types
 *     Generic
 *     TypeBounds
 *     Associated
 *     Traits
 *
 * or any other grammar merely to recognize a variance annotation.
 *
 * Dependency direction:
 *
 *     Variance
 *          |
 *          | exports varianceAnnotation
 *          v
 *     generic declaration grammar
 *          |
 *          v
 *     Types / declarations / functions
 *
 * NOT:
 *
 *     Types
 *       -> Variance
 *       -> Types
 *
 * and NOT:
 *
 *     Variance
 *       -> Types
 *       -> Variance
 *
 * This keeps the variance component independently compilable.
 *
 * ============================================================================
 * PUBLIC GRAMMAR API
 * ============================================================================
 *
 * PUBLIC:
 *
 *     varianceAnnotation
 *     varianceKind
 *
 * The specialized named rules:
 *
 *     covariantVariance
 *     contravariantVariance
 *     invariantVariance
 *
 * are also exported as parser rules for diagnostic clarity and testing.
 *
 * No rule in this file consumes a type expression.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar Variance;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PUBLIC VARIANCE ANNOTATION
 * ========================================================================== */

/**
 * Explicit generic-parameter variance declaration.
 *
 * Examples:
 *
 *     variance covariant
 *     variance contravariant
 *     variance invariant
 *
 * The enclosing generic declaration grammar owns the parameter to which this
 * annotation applies.
 */
varianceAnnotation
    : VARIANCE varianceKind
    ;


/* ============================================================================
 * 2. VARIANCE KIND
 * ========================================================================== */

/**
 * The complete language-defined variance relation.
 *
 * No implementation-specific fourth/fifth/etc. variance mode is introduced
 * here.
 *
 * Future extensions to the type system should be introduced through an
 * explicit language specification change rather than silently changing the
 * meaning of an existing kind.
 */
varianceKind
    : covariantVariance
    | contravariantVariance
    | invariantVariance
    ;


/* ============================================================================
 * 3. COVARIANCE
 * ========================================================================== */

/**
 * Covariance means that, where the semantic subtype relation permits it,
 * substituting a more specific argument may preserve the corresponding
 * generic type relation.
 *
 * Example semantic shape:
 *
 *     Producer<Child> <: Producer<Parent>
 *
 * This example is explanatory only. The grammar does not perform subtyping.
 */
covariantVariance
    : COVARIANT
    ;


/* ============================================================================
 * 4. CONTRAVARIANCE
 * ========================================================================== */

/**
 * Contravariance means that, where the semantic subtype relation permits it,
 * the generic relation reverses the relevant type-argument direction.
 *
 * Example semantic shape:
 *
 *     Consumer<Parent> <: Consumer<Child>
 *
 * This example is explanatory only. The grammar does not perform subtyping.
 */
contravariantVariance
    : CONTRAVARIANT
    ;


/* ============================================================================
 * 5. INVARIANCE
 * ========================================================================== */

/**
 * Invariance requires the relevant type argument to remain exactly compatible
 * according to the semantic type relation.
 *
 * This does not mean that the underlying representation must be identical.
 * The semantic type system defines the actual equality/compatibility relation.
 */
invariantVariance
    : INVARIANT
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT construct Rust AST values directly.
 *
 * The parser/frontend adapter must lower the parser result into the
 * repository's canonical generic-parameter representation.
 *
 * IMPORTANT:
 *
 * The current TypeParameter representation in src/ast/mod.rs contains:
 *
 *     name
 *     bounds
 *
 * but does not yet contain variance.
 *
 * Therefore production integration requires the AST/semantic model to gain
 * an explicit variance field before this syntax can be considered fully
 * implemented.
 *
 * The required semantic representation is conceptually:
 *
 *     GenericVariance
 *         Covariant
 *         Contravariant
 *         Invariant
 *
 * and:
 *
 *     TypeParameter
 *         name
 *         variance
 *         bounds
 *
 * The AST owner, not this grammar, decides the exact Rust representation.
 *
 * The representation MUST:
 *
 *     - preserve explicit source variance;
 *     - preserve the source span;
 *     - distinguish omitted variance from explicit invariant if the language
 *       permits an omitted state;
 *     - remain domain-neutral;
 *     - remain independent of backend representation.
 *
 * This grammar MUST NOT introduce:
 *
 *     VarianceTypeExpr
 *     VarianceType
 *     GenericVarianceAst2
 *     QuantumVariance
 *     HardwareVariance
 *
 * ============================================================================
 * DEFAULT-VARIANCE CONTRACT
 * ============================================================================
 *
 * This grammar deliberately does NOT assign a default variance.
 *
 * If a generic parameter has no:
 *
 *     varianceAnnotation
 *
 * the AST/semantic layer must represent that fact explicitly.
 *
 * The semantic specification must then define whether the language treats an
 * omitted annotation as:
 *
 *     inferred/internal-only
 *     invariant
 *     declaration-invalid
 *     context-dependent
 *
 * The parser must not make that semantic decision.
 *
 * For public generic APIs, the preferred production policy is:
 *
 *     explicit declaration
 *         -> explicit variance
 *
 * while any inference mechanism remains an internal semantic analysis rule
 * and cannot silently weaken type safety.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns all meaning beyond syntax.
 *
 * It MUST validate:
 *
 *     - whether variance is allowed for the generic parameter;
 *     - whether the declaration is type-level and supports variance;
 *     - whether the declared variance is compatible with all uses;
 *     - whether variance conflicts with mutable positions;
 *     - whether variance conflicts with ownership/borrowing rules;
 *     - whether variance conflicts with associated types;
 *     - whether variance conflicts with bounds;
 *     - whether variance conflicts with effect-qualified types;
 *     - whether variance conflicts with capability-bearing types;
 *     - whether variance conflicts with resource-bearing types;
 *     - whether variance conflicts with quantum resource semantics;
 *     - whether variance conflicts with HDL/hardware semantics;
 *     - whether the resulting subtype relation is sound.
 *
 * The parser performs none of these checks.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Variance modifies the relationship between generic type arguments.
 *
 * It does NOT modify:
 *
 *     the identity of the type constructor;
 *     the identity of the type argument;
 *     the physical representation;
 *     the ABI;
 *     the resource capacity;
 *     the hardware target.
 *
 * For example:
 *
 *     Producer<T>
 *
 * may have a covariant parameter:
 *
 *     Producer<T variance covariant>
 *
 * while:
 *
 *     Consumer<T>
 *
 * may have a contravariant parameter:
 *
 *     Consumer<T variance contravariant>
 *
 * and a mutable/resource-sensitive container may require:
 *
 *     invariant
 *
 * The actual validity is semantic, not grammatical.
 *
 * ============================================================================
 * FUNCTION VARIANCE CONTRACT
 * ============================================================================
 *
 * Function compatibility is a related but separate semantic concept.
 *
 * The grammar does NOT define:
 *
 *     parameter variance
 *     return variance
 *     effect variance
 *     capability variance
 *     resource variance
 *
 * Those relationships belong to function-type semantic checking.
 *
 * In particular, the language specification already requires function
 * compatibility to account for:
 *
 *     contravariant parameters
 *     covariant returns
 *     effect/resource/capability constraints
 *
 * This file must not duplicate that logic.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Variance declarations do not themselves create effects.
 *
 * Therefore:
 *
 *     varianceAnnotation
 *
 * has no inherent:
 *
 *     IO
 *     mutation
 *     network
 *     quantum measurement
 *     learning
 *     adaptation
 *     reflection
 *     native
 *     foreign
 *     distributed
 *     simulation
 *
 * effect.
 *
 * Semantic checking may consider effects when determining whether a type
 * relation is valid.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * A variance declaration requires no capability by itself.
 *
 * Semantic validation may inspect capability-bearing generic arguments or
 * bounds when determining whether a variance relationship is sound.
 *
 * This grammar MUST NOT resolve capabilities.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * A variance declaration consumes no physical resource and declares no
 * physical capacity.
 *
 * It MUST NOT encode:
 *
 *     memory size
 *     processor count
 *     accelerator count
 *     qubit count
 *     device count
 *     register width
 *     topology size
 *
 * Generic variance remains valid from the smallest supported execution
 * environment through arbitrarily large environments subject only to the
 * actual resources and semantic constraints of the program.
 *
 * ============================================================================
 * CONTRACT / VALIDATION CONTRACT
 * ============================================================================
 *
 * Source-level contracts may constrain values involving generic types, but
 * variance itself is not a runtime contract.
 *
 * For example, a semantic contract may later establish relationships between
 * two instantiations of a generic type.
 *
 * This grammar does not parse:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Those constructs remain owned by the validation subsystem.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Variance syntax is not a policy mechanism.
 *
 * Security, sandboxing, deployment, execution, resource, and adaptation
 * policies remain owned by their respective policy grammars.
 *
 * A policy may affect whether a particular generic specialization is allowed
 * to execute, but that decision occurs after parsing and semantic analysis.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The parser must preserve source location information for:
 *
 *     variance
 *     variance kind
 *
 * The frontend/semantic layer should record the origin of an explicit
 * variance declaration so diagnostics and compiler explanations can state
 * where the variance came from.
 *
 * This is especially important when:
 *
 *     generic types
 *     aliases
 *     traits
 *     interfaces
 *     associated types
 *     specialization
 *
 * are involved.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Variance is domain-neutral.
 *
 * A generic type may eventually describe:
 *
 *     quantum state
 *     logical qubit
 *     measurement result
 *     quantum operation
 *     quantum channel
 *
 * but this grammar does not know those domains.
 *
 * Semantic validation must ensure that variance never permits an unsound
 * transformation of a quantum/resource-bearing abstraction.
 *
 * No physical qubit is allocated by this grammar.
 *
 * No quantum operation is emitted by this grammar.
 *
 * No quantum::ir node is emitted directly by this grammar.
 *
 * If a generic quantum abstraction survives semantic analysis, its domain
 * semantics are eventually lowered through the canonical:
 *
 *     semantic model
 *         |
 *         v
 *     quantum::ir
 *
 * boundary.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * The same rule applies to HDL and hardware abstractions.
 *
 * Variance may appear on generic abstractions used by:
 *
 *     signals
 *     interfaces
 *     components
 *     memories
 *     accelerators
 *     hardware resources
 *
 * but this grammar does not select:
 *
 *     FPGA
 *     ASIC
 *     CPU
 *     GPU
 *     QPU
 *     device
 *     topology
 *     placement
 *
 * Those decisions remain downstream.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Variance must preserve source portability.
 *
 * A source declaration such as:
 *
 *     T variance covariant
 *
 * describes a type relationship.
 *
 * It does NOT prescribe:
 *
 *     processor architecture
 *     register width
 *     memory organization
 *     accelerator implementation
 *     physical device
 *     number of execution units
 *     quantum hardware topology
 *
 * Therefore the same source-level generic declaration can participate in
 * compilation for different realizations without changing its grammar-level
 * meaning.
 *
 * Target-specific feasibility is evaluated after source semantics have been
 * established.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar contains no fixed limit on:
 *
 *     number of generic parameters
 *     number of declarations
 *     nesting depth
 *     number of type constructors
 *     number of instantiations
 *     number of program modules
 *     number of target realizations
 *
 * Repetition is represented structurally by the enclosing generic grammar.
 *
 * Any implementation resource limits used to defend against hostile compiler
 * input must remain configurable implementation/security policy, not
 * language semantics.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing this construct MUST be deterministic.
 *
 * It must not depend on:
 *
 *     wall-clock time
 *     randomness
 *     target hardware
 *     target availability
 *     filesystem state
 *     network state
 *     environment state
 *     runtime state
 *
 * Identical source/token input under the same grammar and lexical versions
 * must produce the same parser structure.
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust
 *     no parser actions
 *     no semantic predicates
 *     no unsafe operations
 *     no filesystem access
 *     no network access
 *     no hardware access
 *     no runtime execution
 *
 * The Rust compiler/frontend/runtime consuming this grammar remains required
 * to use:
 *
 *     Rust 2021
 *     Rust 1.97+
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser-level diagnostics should identify malformed variance syntax.
 *
 * Examples:
 *
 *     variance
 *
 *     variance unknown
 *
 *     variance covariant extra
 *
 *     variance covariant contravariant
 *
 * must not be accepted as a valid variance annotation.
 *
 * Semantic diagnostics, rather than parser diagnostics, should report:
 *
 *     variance not permitted here
 *     conflicting variance declarations
 *     unsound variance
 *     variance incompatible with mutable use
 *     variance incompatible with associated constraints
 *
 * The grammar must not attempt to produce those semantic diagnostics itself.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive parser tests:
 *
 *     variance covariant
 *     variance contravariant
 *     variance invariant
 *
 * Boundary tests:
 *
 *     variance covariant
 *     variance invariant
 *     variance contravariant
 *
 *     multiple generic parameters each carrying independent variance
 *
 *     nested generic declarations
 *
 * Negative parser tests:
 *
 *     variance
 *     variance unknown
 *     variance covariance
 *     variance contravariantly
 *     variance covariant invariant
 *
 * Integration tests must additionally verify:
 *
 *     generic declaration + variance
 *     generic declaration + variance + bounds
 *     trait + generic variance
 *     interface + generic variance
 *     type alias + generic variance
 *     function generic declarations where variance is permitted
 *
 * Semantic tests must verify:
 *
 *     valid covariance
 *     valid contravariance
 *     valid invariance
 *     invalid covariance
 *     invalid contravariance
 *     invalid variance on unsupported parameters
 *     mutable-position restrictions
 *     resource-sensitive type restrictions
 *     quantum/resource safety
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Adding the new variance syntax is a lexical and source-language extension.
 *
 * Existing programs containing:
 *
 *     variance
 *     covariant
 *     contravariant
 *     invariant
 *
 * as ordinary identifiers may become incompatible if those spellings are
 * newly reserved.
 *
 * Therefore the compatibility subsystem must explicitly classify the
 * reservation change before these tokens are promoted to stable language
 * vocabulary.
 *
 * If backward compatibility requires contextual treatment, the lexical
 * design must be changed at the lexer level rather than duplicating
 * variance recognition inside this parser.
 *
 * This grammar itself must not implement compatibility hacks.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * indirectly composed from:
 *
 *     grammar/lexer/lexer.g4
 *     grammar/lexer/keywords.g4
 *     grammar/lexer/tokens.g4
 *
 * EXPORTS:
 *
 *     varianceAnnotation
 *     varianceKind
 *     covariantVariance
 *     contravariantVariance
 *     invariantVariance
 *
 * CONSUMED_BY:
 *
 *     grammar/functions/generics.g4
 *     grammar/declarations/traits.g4
 *     generic declaration grammars
 *     type declaration grammars
 *
 * AST_OWNER:
 *
 *     canonical frontend AST type-parameter representation
 *
 * SEMANTIC_OWNER:
 *
 *     canonical type-system semantic analysis
 *
 * IR_OWNER:
 *
 *     canonical semantic/IR layer
 *
 * TEST_OWNER:
 *
 *     grammar/tests/types/
 *     grammar/tests/parser/
 *     grammar/tests/semantic/
 *     grammar/tests/compatibility/
 *
 * SPEC_OWNER:
 *
 *     grammar/specification/types.md
 *     grammar/spec/type-system.md
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * The integration sequence is:
 *
 *     ZamaniLexer
 *          |
 *          v
 *     Variance
 *          |
 *          v
 *     generic declaration grammar
 *          |
 *          v
 *     TypeParameter AST
 *          |
 *          v
 *     semantic variance validation
 *          |
 *          v
 *     subtype / compatibility engine
 *
 * The generic declaration grammar owns the attachment point.
 *
 * Conceptually:
 *
 *     genericParameter
 *         : identifier
 *           varianceAnnotation?
 *           genericParameterBounds?
 *         ;
 *
 * The exact existing generic rule should be updated to consume this
 * delegate rather than duplicating its contents.
 *
 * The variance delegate must remain independently complete and must not need
 * later modification merely because generic bounds, type expressions, traits,
 * quantum types, hardware types, or AI/data types are extended.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] it compiles as an ANTLR parser delegate;
 *     [ ] it imports no complete type grammar;
 *     [ ] it defines no lexer rules;
 *     [ ] it has exactly one owner for variance syntax;
 *     [ ] it recognizes all three specified variance kinds;
 *     [ ] malformed variance syntax is rejected;
 *     [ ] generic declaration grammars consume varianceAnnotation;
 *     [ ] the canonical AST preserves explicit variance;
 *     [ ] semantic analysis validates variance soundness;
 *     [ ] specification and grammar agree;
 *     [ ] positive tests exist;
 *     [ ] negative tests exist;
 *     [ ] boundary tests exist;
 *     [ ] compatibility tests exist;
 *     [ ] deterministic parsing is verified;
 *     [ ] no hardware/resource limits are encoded;
 *     [ ] no unsafe Rust requirement exists;
 *     [ ] quantum/resource-bearing types remain semantically protected;
 *     [ ] no duplicate variance grammar exists elsewhere.
 *
 * ============================================================================
 */