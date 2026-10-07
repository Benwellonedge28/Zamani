/*
 * ============================================================================
 * ZAMANI PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/compile/features.g4
 *
 * GRAMMAR
 * -------
 * CompileFeatures
 *
 * STATUS
 * ------
 * CANONICAL / PRODUCTION FEATURE-SYNTAX FOUNDATION
 *
 * BASELINE
 * --------
 * Rust 1.97+
 * Rust 2021
 * Safe Rust only
 * No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE REUSABLE SYNTAX AUTHORITY for compilation-feature
 * references and feature expressions.
 *
 * It provides the common syntax consumed by:
 *
 *     feature-selection.g4
 *     compile-time.g4
 *     conditional-compilation.g4
 *     compilation.g4
 *     future compilation-policy grammars
 *
 * This file deliberately does NOT own feature-selection branches.
 *
 * Branch/container ownership remains:
 *
 *     grammar/compile/feature-selection.g4
 *
 * This separation prevents feature identity/expression syntax from being
 * duplicated in multiple compilation grammars.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     parser
 *          |
 *          +--> CompileFeatures
 *          |       |
 *          |       +--> feature reference
 *          |       +--> feature arguments
 *          |       +--> feature expression
 *          |       +--> feature conjunction/disjunction
 *          |       +--> feature negation
 *          |
 *          +--> feature-selection.g4
 *          |       |
 *          |       +--> branch selection
 *          |
 *          +--> compile-time.g4
 *          |       |
 *          |       +--> compile-time feature control
 *          |
 *          +--> conditional-compilation.g4
 *                  |
 *                  +--> conditional feature predicates
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic feature resolution
 *          |
 *          +--> feature registry
 *          +--> dialect registry
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> target-independent specialization
 *          +--> compilation context
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical representation
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed representation
 *          +--> heterogeneous representation
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     featureKeyword
 *     featureName
 *     featureReference
 *     featureArguments
 *     featureArgumentList
 *     featureArgument
 *     featureExpression
 *     featureDisjunction
 *     featureConjunction
 *     featureNegation
 *     featurePrimary
 *     featureNameList
 *     featureReferenceList
 *
 * THIS FILE DOES NOT OWN
 * ---------------------
 *
 *     feature registry
 *     feature availability
 *     feature implementation
 *     feature selection policy
 *     conditional-compilation branch syntax
 *     branch bodies
 *     target selection
 *     target realization
 *     hardware discovery
 *     resource allocation
 *     resource implementation
 *     capability implementation
 *     dialect registration
 *     ordinary expressions
 *     ordinary statements
 *     macros
 *     compile-time execution
 *     specialization algorithms
 *     optimization
 *     lowering
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *     AST construction
 *     semantic analysis
 *     IR construction
 *
 * ============================================================================
 * CORE DESIGN RULE
 * ============================================================================
 *
 * A feature is a SOURCE-LEVEL SEMANTIC/COMPILATION CONCEPT.
 *
 * A feature is NOT inherently:
 *
 *     a CPU
 *     a GPU
 *     an FPGA
 *     an ASIC
 *     a QPU
 *     a device
 *     a machine
 *     a vendor
 *     a memory size
 *     a processor count
 *     a qubit count
 *     a topology
 *     a physical address
 *     a runtime handle
 *
 * Therefore:
 *
 *     feature != target
 *     feature != resource
 *     feature != capability instance
 *     feature != physical device
 *
 * Semantic analysis may establish relationships among these concepts, but
 * this grammar must not collapse them into one syntax category.
 *
 * ============================================================================
 * OPEN-WORLD FEATURE MODEL
 * ============================================================================
 *
 * Feature names are intentionally OPEN-WORLD.
 *
 * The grammar MUST NOT enumerate:
 *
 *     quantum
 *     classical
 *     hdl
 *     gpu
 *     cpu
 *     ai
 *     distributed
 *     tensor
 *     networking
 *     accelerator
 *     vendor names
 *     processor models
 *     QPU models
 *     future architectures
 *
 * Such names are ordinary semantic identities.
 *
 * Examples:
 *
 *     quantum
 *     quantum.dynamic
 *     quantum.error_correction
 *     classical.parallel
 *     tensor.compute
 *     hdl.synthesis
 *     distributed.collectives
 *     hardware.reconfigurable
 *     future.computation.model
 *
 * are all syntactically valid without changing this grammar.
 *
 * Adding a new feature therefore does NOT require editing this file.
 *
 * ============================================================================
 * LEXICAL STRATEGY
 * ============================================================================
 *
 * The current canonical token registry does not require a dedicated FEATURE
 * token for every feature identity.
 *
 * Consequently the source spelling:
 *
 *     feature
 *
 * is represented at this parser boundary through IDENTIFIER.
 *
 * The semantic layer MUST validate the contextual keyword spelling where the
 * grammar uses featureKeyword.
 *
 * This is deliberate.
 *
 * It avoids creating a global reserved-word requirement merely to support an
 * open-ended feature namespace.
 *
 * If the language specification later promotes `feature` to a globally
 * reserved lexical token, the migration must preserve these public parser
 * contracts:
 *
 *     featureKeyword
 *     featureName
 *     featureReference
 *     featureExpression
 *
 * Downstream AST/semantic contracts must remain stable.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/names.g4
 *     grammar/expressions/expressions.g4
 *
 * Imported parser grammars:
 *
 *     Names
 *     Expressions
 *
 * Names supplies:
 *
 *     qualifiedName
 *
 * Expressions supplies:
 *
 *     expression
 *
 * Feature syntax deliberately reuses those canonical authorities.
 *
 * No feature grammar may create another identifier, qualified-name, or
 * general-expression language.
 *
 * ============================================================================
 * EXPORT CONTRACT
 * ============================================================================
 *
 * PUBLIC
 * ------
 *
 *     featureKeyword
 *     featureName
 *     featureReference
 *     featureArguments
 *     featureArgumentList
 *     featureArgument
 *     featureExpression
 *     featureDisjunction
 *     featureConjunction
 *     featureNegation
 *     featurePrimary
 *     featureNameList
 *     featureReferenceList
 *
 * These are reusable parser contracts.
 *
 * `featureSelection` is intentionally NOT exported here.
 *
 * ============================================================================
 * CONSUMERS
 * ============================================================================
 *
 * Primary consumers:
 *
 *     grammar/compile/feature-selection.g4
 *     grammar/compile/compile-time.g4
 *     grammar/compile/conditional-compilation.g4
 *
 * Possible future consumers:
 *
 *     grammar/compile/compilation.g4
 *     grammar/compile/intent.g4
 *     grammar/compile/optimization.g4
 *     grammar/compile/specialization.g4
 *     grammar/policies/
 *     grammar/resources/
 *     grammar/dialects/
 *
 * Consumers MUST reuse these rules rather than reproduce feature syntax.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates parser contexts only.
 *
 * The frontend AST owns the semantic representation.
 *
 * The AST should conceptually preserve:
 *
 *     FeatureReference
 *         name
 *         arguments
 *         source_span
 *
 *     FeatureExpression
 *         operator
 *         operands
 *         source_span
 *
 * The AST MUST preserve source locations.
 *
 * The AST MUST preserve the distinction between:
 *
 *     feature identity
 *     feature argument
 *     feature expression
 *
 * The AST MUST NOT contain:
 *
 *     physical device identifiers
 *     hardware handles
 *     resource allocations
 *     scheduler state
 *     routing state
 *     QEC state
 *     calibration state
 *     backend handles
 *     runtime state
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis resolves feature references.
 *
 * A feature reference may resolve to:
 *
 *     known enabled
 *     known disabled
 *     unknown
 *     incompatible
 *     invalid
 *
 * The parser does NOT determine any of these states.
 *
 * UNKNOWN MUST NOT silently mean FALSE.
 *
 * This distinction is required for portable compilation and diagnostics.
 *
 * ============================================================================
 * FEATURE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * A feature may be satisfied by one or more capabilities.
 *
 * For example, semantically:
 *
 *     quantum.dynamic
 *
 * might be satisfied by a compilation context exposing the appropriate
 * capabilities.
 *
 * However:
 *
 *     feature quantum.dynamic
 *
 * does NOT mean:
 *
 *     choose a particular QPU.
 *
 * Capability resolution belongs to the resource/capability semantic layer.
 *
 * ============================================================================
 * FEATURE / RESOURCE CONTRACT
 * ============================================================================
 *
 * Feature syntax MUST NOT translate directly into resource quantities.
 *
 * For example:
 *
 *     quantum
 *
 * must NOT grammatically imply:
 *
 *     some fixed qubit count
 *
 * and:
 *
 *     tensor.compute
 *
 * must NOT grammatically imply:
 *
 *     some fixed memory size
 *
 * Resource requirements remain symbolic and open-ended.
 *
 * ============================================================================
 * FEATURE / TARGET CONTRACT
 * ============================================================================
 *
 * Feature resolution is separate from target selection.
 *
 * The target subsystem remains responsible for:
 *
 *     target intent
 *     target expressions
 *     target selection
 *     target realization
 *
 * Feature syntax must not introduce:
 *
 *     device IDs
 *     vendor IDs
 *     processor IDs
 *     physical topology
 *     physical placement
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Referencing a feature has no implicit runtime effect.
 *
 * Feature-dependent source may later introduce effects such as:
 *
 *     compile_time
 *     code_generation
 *     reflection
 *     native
 *     foreign
 *     simulation
 *     network
 *     distributed
 *     quantum
 *     learning
 *     adaptation
 *
 * Those effects belong to the effect/semantic systems.
 *
 * This grammar does not assign effects.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource requirements remain owned by:
 *
 *     grammar/resources/
 *
 * A feature expression may participate in resource/capability resolution, but
 * this grammar does not duplicate:
 *
 *     resourceRequirementExpression
 *     resourceCapabilityCall
 *     resourceCapabilityReference
 *
 * Resource semantics remain independent.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Feature conditions may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Contract ownership remains under:
 *
 *     grammar/validation/
 *
 * This grammar does not redefine contract syntax.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Feature resolution may be constrained by:
 *
 *     compilation policy
 *     security policy
 *     dialect policy
 *     reproducibility policy
 *     deployment policy
 *     optimization policy
 *
 * Policy evaluation is downstream.
 *
 * A feature reference does not bypass policy.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Feature decisions may affect compilation provenance.
 *
 * Downstream provenance should be capable of recording:
 *
 *     feature expression
 *     feature-resolution result
 *     compilation context
 *     selected branch
 *     compiler version
 *     language version
 *     dialect/version information
 *     reason/evidence where applicable
 *
 * This grammar preserves the syntax necessary to construct that record.
 *
 * ============================================================================
 * REPRODUCIBILITY CONTRACT
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * Feature availability MUST NOT affect parsing.
 *
 * For a fixed compilation context:
 *
 *     same source
 *     same language version
 *     same feature registry/context
 *
 * MUST produce the same semantic feature-resolution result.
 *
 * Host state MUST NOT silently alter feature resolution.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * This grammar has NO direct dependency on quantum::ir.
 *
 * If feature selection causes quantum source to be selected, the normal
 * pipeline remains:
 *
 *     feature resolution
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum semantic representation
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     decomposition
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     resilience / QEC
 *          |
 *          v
 *     ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target realization
 *
 * No physical quantum implementation is encoded here.
 *
 * ============================================================================
 * HDL BOUNDARY
 * ============================================================================
 *
 * Feature references may identify HDL-related semantic capabilities, such as:
 *
 *     hdl.synthesis
 *     hdl.simulation
 *     hdl.verification
 *
 * These remain semantic names.
 *
 * HDL syntax and hardware realization remain owned by:
 *
 *     grammar/hdl/
 *     grammar/hardware/
 *
 * ============================================================================
 * DIALECT CONTRACT
 * ============================================================================
 *
 * Dialects may define new feature identities.
 *
 * A dialect MUST NOT require this grammar to enumerate every feature it
 * introduces.
 *
 * Example:
 *
 *     vendor.domain.feature
 *
 * remains syntactically valid.
 *
 * Dialect registration, compatibility and semantic meaning belong to:
 *
 *     grammar/dialects/
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There is NO grammar-level maximum for:
 *
 *     number of feature names
 *     number of qualified-name components
 *     number of feature arguments
 *     number of feature predicates
 *     number of conjunction operands
 *     number of disjunction operands
 *     nesting depth
 *     feature-reference count
 *
 * Repetition is represented with recursive/EBNF forms.
 *
 * No:
 *
 *     MAX_FEATURES
 *     MAX_FEATURE_ARGUMENTS
 *     MAX_FEATURE_PREDICATES
 *     MAX_FEATURE_DEPTH
 *
 * or equivalent constants may be introduced.
 *
 * Physical/compiler resource limits remain implementation policy.
 *
 * ============================================================================
 * GRAMMAR DECLARATION
 * ============================================================================
 */

parser grammar CompileFeatures;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Expressions
;


/*
 * ============================================================================
 * 1. FEATURE KEYWORD
 * ============================================================================
 *
 * The current lexer intentionally keeps the open feature namespace flexible.
 *
 * Therefore `feature` is recognized contextually through IDENTIFIER here.
 *
 * Semantic validation MUST verify that the identifier spelling is exactly:
 *
 *     feature
 *
 * This avoids introducing a second lexical authority.
 */
featureKeyword
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 2. FEATURE NAME
 * ============================================================================
 *
 * Feature names are qualified semantic identities.
 *
 * Examples:
 *
 *     quantum
 *     quantum.dynamic
 *     quantum.error_correction
 *     classical.parallel
 *     tensor.compute
 *     future.domain.capability
 *
 * The number of qualification segments is not fixed.
 */
featureName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 3. FEATURE REFERENCE
 * ============================================================================
 *
 * Supported forms include:
 *
 *     feature("quantum")
 *
 * and:
 *
 *     feature quantum
 *
 * and:
 *
 *     quantum
 *
 * The semantic layer determines whether a bare qualified name is interpreted
 * as a feature reference in the enclosing construct.
 *
 * The explicit `feature(...)` form is the preferred unambiguous form for
 * contexts where a generic expression is also possible.
 */
featureReference
    : featureKeyword
      LPAREN
      featureArgumentList?
      RPAREN
    | featureKeyword
      featureName
      featureArguments?
    | featureName
      featureArguments?
    ;


/*
 * ============================================================================
 * 4. FEATURE ARGUMENTS
 * ============================================================================
 *
 * Arguments reuse the canonical expression grammar.
 *
 * This is important:
 *
 *     feature syntax
 *
 * does NOT create:
 *
 *     feature-expression-language
 *
 * Arguments can therefore evolve with the normal Zamani expression system.
 */
featureArguments
    : LPAREN
      featureArgumentList?
      RPAREN
    ;


featureArgumentList
    : featureArgument
      (
          COMMA featureArgument
      )*
    ;


featureArgument
    : expression
    ;


/*
 * ============================================================================
 * 5. FEATURE EXPRESSION
 * ============================================================================
 *
 * Feature expressions provide a small semantic Boolean algebra:
 *
 *     NOT
 *     AND
 *     OR
 *     grouping
 *
 * The grammar does not enumerate feature identities.
 */
featureExpression
    : featureDisjunction
    ;


featureDisjunction
    : featureConjunction
      (
          LOGICAL_OR
          featureConjunction
      )*
    ;


featureConjunction
    : featureNegation
      (
          LOGICAL_AND
          featureNegation
      )*
    ;


featureNegation
    : NOT featureNegation
    | featurePrimary
    ;


featurePrimary
    : featureReference
    | LPAREN featureExpression RPAREN
    ;


/*
 * ============================================================================
 * 6. FEATURE NAME LIST
 * ============================================================================
 *
 * Reusable list for compilation contexts that need symbolic feature names.
 */
featureNameList
    : featureName
      (
          COMMA featureName
      )*
    ;


/*
 * ============================================================================
 * 7. FEATURE REFERENCE LIST
 * ============================================================================
 *
 * Reusable list for compilation contexts that need feature expressions or
 * parameterized references.
 */
featureReferenceList
    : featureReference
      (
          COMMA featureReference
      )*
    ;


/*
 * ============================================================================
 * 8. INTEGRATION ADAPTERS
 * ============================================================================
 *
 * These aliases provide stable semantic/parser boundaries for downstream
 * compilation grammars.
 *
 * They intentionally do not create new syntax.
 */

compileFeatureReference
    : featureReference
    ;


compileFeatureExpression
    : featureExpression
    ;


/*
 * ============================================================================
 * 9. SOURCE/AST BOUNDARY
 * ============================================================================
 *
 * No AST is created here.
 *
 * The parser produces:
 *
 *     CompileFeaturesParser contexts
 *
 * The frontend maps them into the domain-neutral AST.
 *
 * The AST builder must not infer:
 *
 *     hardware
 *     resources
 *     capabilities
 *     targets
 *
 * merely from a feature name.
 *
 * Such relationships belong to semantic resolution.
 */


/*
 * ============================================================================
 * 10. SEMANTIC RESOLUTION BOUNDARY
 * ============================================================================
 *
 * Conceptual pipeline:
 *
 *     FeatureReference
 *          |
 *          v
 *     name resolution
 *          |
 *          v
 *     feature registry/context
 *          |
 *          +--> enabled
 *          +--> disabled
 *          +--> unknown
 *          +--> incompatible
 *          +--> invalid
 *          |
 *          v
 *     feature decision
 *
 * Feature decisions may subsequently participate in:
 *
 *     target-independent specialization
 *     conditional source selection
 *     capability analysis
 *     resource analysis
 *     compilation planning
 *     optimization planning
 *     provenance
 *
 * None of those operations happen in this grammar.
 */


/*
 * ============================================================================
 * 11. UNKNOWN-FEATURE RULE
 * ============================================================================
 *
 * Unknown feature names remain syntactically valid.
 *
 * Example:
 *
 *     future.computation.model
 *
 * MUST parse.
 *
 * Whether it is supported is a semantic question.
 *
 * This is essential for:
 *
 *     future compatibility
 *     dialects
 *     vendor extensions
 *     experimental features
 *     cross-version tooling
 *
 * The compiler MUST NOT silently reinterpret an unknown feature as an
 * unrelated known feature.
 */


/*
 * ============================================================================
 * 12. FEATURE ARGUMENT SEMANTICS
 * ============================================================================
 *
 * Feature arguments are semantic values.
 *
 * The grammar imposes no universal meaning on them.
 *
 * Examples may include:
 *
 *     feature("quantum", mode)
 *     feature("tensor", precision)
 *     feature("dialect", version)
 *
 * No grammar rule maps an argument to:
 *
 *     device
 *     memory
 *     processor
 *     qubit
 *     topology
 *
 * without semantic ownership explicitly defining that relationship.
 */


/*
 * ============================================================================
 * 13. SECURITY
 * ============================================================================
 *
 * Parsing a feature reference MUST NOT:
 *
 *     read files;
 *     access environment variables;
 *     inspect hardware;
 *     open network connections;
 *     execute commands;
 *     inspect credentials;
 *     invoke plugins;
 *     execute arbitrary code.
 *
 * Feature resolution receives an explicit compilation context from trusted
 * compiler infrastructure.
 */


/*
 * ============================================================================
 * 14. DETERMINISM
 * ============================================================================
 *
 * Parsing is a pure function of:
 *
 *     source tokens
 *     grammar
 *     lexer vocabulary
 *
 * Feature availability cannot alter the parse.
 *
 * This keeps parsing reproducible.
 */


/*
 * ============================================================================
 * 15. COMPATIBILITY
 * ============================================================================
 *
 * Historical feature spellings should be handled by the compatibility layer.
 *
 * This grammar must not accumulate duplicate aliases indefinitely.
 *
 * A compatibility alias should normalize semantically after parsing while
 * preserving the original source span for diagnostics/provenance.
 */


/*
 * ============================================================================
 * 16. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST NOT contain:
 *
 *     MAX_FEATURES
 *     MAX_FEATURE_ARGUMENTS
 *     MAX_FEATURE_PREDICATES
 *     MAX_FEATURE_DEPTH
 *     MAX_TARGETS
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_NODES
 *     MAX_TENSOR_RANK
 *     MAX_NETWORK_SIZE
 *
 * It must also not contain disguised fixed equivalents.
 */


/*
 * ============================================================================
 * 17. RUST SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust;
 *     no actions;
 *     no semantic predicates;
 *     no unsafe code;
 *     no runtime execution.
 *
 * The generated Rust frontend must:
 *
 *     compile with Rust 1.97 or later;
 *     use Rust 2021 or the repository's newer compatible edition;
 *     contain no unsafe implementation requirement.
 */


/*
 * ============================================================================
 * 18. TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 *     feature("quantum")
 *     feature("classical")
 *     feature("hdl")
 *     feature("distributed")
 *     feature("future.computation.model")
 *
 *     feature quantum
 *     feature quantum.dynamic
 *     quantum
 *     quantum.dynamic
 *
 *     feature("tensor", precision)
 *     feature("dialect", version)
 *
 *     quantum && classical
 *     quantum || classical
 *     !quantum
 *     !(quantum || classical)
 *     quantum && !classical
 *
 * NEGATIVE
 * --------
 *
 *     feature(
 *     feature()
 *       where the enclosing semantic context requires an identity
 *     malformed qualified names
 *     malformed argument lists
 *     unbalanced parentheses
 *     missing expression operands
 *
 * SEMANTIC BOUNDARY
 * -----------------
 *
 *     unknown feature
 *     disabled feature
 *     incompatible feature
 *     invalid feature arguments
 *     contradictory feature expression
 *
 * CROSS-DOMAIN
 * ------------
 *
 *     quantum
 *     classical
 *     hdl
 *     ai
 *     tensor
 *     distributed
 *     networking
 *     simulation
 *     interoperability
 *     future.domain.feature
 *
 * SCALABILITY
 * -----------
 *
 *     deeply composed feature expressions
 *     many OR operands
 *     many AND operands
 *     long qualified names
 *     many arguments
 *     nested feature expressions
 *
 * No test may introduce an artificial language-level finite limit.
 */


/*
 * ============================================================================
 * 19. COMPLETION CRITERIA
 * ============================================================================
 *
 * CompileFeatures is DONE when:
 *
 * [ ] ZamaniLexer is the only lexical authority.
 *
 * [ ] Names is the only qualified-name authority.
 *
 * [ ] Expressions is the only general-expression authority.
 *
 * [ ] Feature identity is open-world.
 *
 * [ ] Feature expressions are reusable.
 *
 * [ ] Feature selection is NOT duplicated here.
 *
 * [ ] Target selection is NOT duplicated here.
 *
 * [ ] Resource syntax is NOT duplicated here.
 *
 * [ ] Capability syntax is NOT duplicated here.
 *
 * [ ] Feature availability is not decided during parsing.
 *
 * [ ] Unknown features remain representable.
 *
 * [ ] No physical hardware is encoded.
 *
 * [ ] No resource ceilings are encoded.
 *
 * [ ] No fixed feature catalogue is encoded.
 *
 * [ ] No AST implementation is embedded.
 *
 * [ ] No IR is embedded.
 *
 * [ ] No quantum::ir dependency is introduced.
 *
 * [ ] No HDL backend dependency is introduced.
 *
 * [ ] No Rust actions exist.
 *
 * [ ] No unsafe Rust is required.
 *
 * [ ] feature-selection.g4 consumes this grammar.
 *
 * [ ] compile-time.g4 consumes this grammar.
 *
 * [ ] conditional-compilation.g4 consumes this grammar.
 *
 * [ ] duplicate feature predicate/reference syntax is removed from those
 *     consumers.
 *
 * [ ] parser, semantic, negative, boundary and scalability tests exist.
 *
 * [ ] feature-resolution provenance can identify the source feature
 *     expression.
 *
 * ============================================================================
 * FINAL RULE
 * ============================================================================
 *
 * This file defines:
 *
 *     WHAT A FEATURE REFERENCE LOOKS LIKE
 *
 * It does not define:
 *
 *     WHETHER A FEATURE EXISTS
 *     WHETHER A FEATURE IS ENABLED
 *     WHICH HARDWARE PROVIDES IT
 *     WHICH RESOURCE PROVIDES IT
 *     WHICH TARGET REALIZES IT
 *     HOW COMPILATION IMPLEMENTS IT
 *
 * Therefore:
 *
 *     source
 *       |
 *       v
 *     feature syntax
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic feature resolution
 *       |
 *       +--> capabilities
 *       +--> resources
 *       +--> policies
 *       +--> targets
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical
 *       +--> quantum::ir
 *       +--> HDL
 *       +--> heterogeneous
 *       |
 *       v
 *     target realization
 *
 * This is the required separation for POCO-REAF.
 *
 * ============================================================================
 */