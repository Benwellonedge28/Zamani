/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/modules/versioning.g4
 *
 * Grammar:
 *     ModuleVersioning
 *
 * Status:
 *     CANONICAL MODULE-VERSIONING GRAMMAR COMPONENT
 *
 * Purpose:
 *     Define source-level syntax that associates version contracts,
 *     compatibility requirements, and version metadata with modules.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * Zamani has ONE general version language.
 *
 * The canonical version-expression syntax is owned by:
 *
 *     grammar/core/versioning.g4
 *
 * This file MUST NOT redefine:
 *
 *     versionCore
 *     exactVersion
 *     versionRange
 *     versionComparator
 *     versionConstraintExpression
 *     versionIdentifier
 *     versionSuffix
 *     versionChannel
 *     versionReference
 *
 * This file owns only the MODULE CONTEXT in which those canonical version
 * expressions are used.
 *
 * The dependency direction is:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     core::Versioning
 *       |
 *       v
 *     modules::ModuleVersioning
 *       |
 *       v
 *     module/package semantic analysis
 *       |
 *       v
 *     canonical semantic model
 *       |
 *       +------------------+------------------+
 *       |                  |                  |
 *       v                  v                  v
 *   classical         quantum::ir       HDL/hardware
 *       |                  |                  |
 *       +------------------+------------------+
 *                          |
 *                          v
 *                 optimization/lowering
 *                          |
 *                  routing/scheduling
 *                          |
 *                   QEC/resilience/ZQN
 *                          |
 *                          v
 *                         HAL
 *                          |
 *                          v
 *                  target realization
 *
 * This file is SYNTAX ONLY.
 *
 * ============================================================================
 * NON-RESPONSIBILITIES
 * ============================================================================
 *
 * This grammar MUST NOT:
 *
 *     - compare versions;
 *     - determine compatibility;
 *     - solve dependency constraints;
 *     - resolve modules;
 *     - resolve packages;
 *     - access registries;
 *     - access the filesystem;
 *     - access the network;
 *     - inspect environment variables;
 *     - inspect compiler configuration;
 *     - select a compiler;
 *     - select a runtime;
 *     - select a CPU;
 *     - select a GPU;
 *     - select an FPGA;
 *     - select an ASIC;
 *     - select a QPU;
 *     - select a physical qubit;
 *     - select a topology;
 *     - select a scheduler;
 *     - perform routing;
 *     - perform QEC;
 *     - perform ZQN analysis;
 *     - construct quantum::ir;
 *     - construct any competing IR;
 *     - execute code.
 *
 * ============================================================================
 * MODULE VERSION != LANGUAGE VERSION
 * ============================================================================
 *
 * This distinction is mandatory.
 *
 * These are different contracts:
 *
 *     language version
 *     module version
 *     package version
 *     dependency version
 *     dialect version
 *     API version
 *     ABI version
 *     compiler version
 *     runtime version
 *     artifact version
 *
 * For example:
 *
 *     language Zamani 1.2.0;
 *
 * and:
 *
 *     module quantum::algorithms version 4.7.0;
 *
 * do NOT describe the same version domain.
 *
 * A module version identifies the contract/evolution of the module.
 *
 * It does not change the meaning of the Zamani language itself.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - module-version clauses;
 *     - module compatibility clauses;
 *     - module version requirements;
 *     - module version declarations;
 *     - module version metadata wrappers;
 *     - module-version structural composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - module declarations;
 *     - module names;
 *     - identifiers;
 *     - qualified names;
 *     - package declarations;
 *     - dependency declarations;
 *     - import declarations;
 *     - export declarations;
 *     - namespace declarations;
 *     - visibility;
 *     - generic attributes;
 *     - core version syntax;
 *     - version comparison;
 *     - dependency solving;
 *     - compatibility algorithms;
 *     - migration;
 *     - deprecation;
 *     - AST implementation;
 *     - semantic implementation;
 *     - IR implementation.
 *
 * ============================================================================
 * AUTHORITATIVE SOURCES
 * ============================================================================
 *
 * Normative architecture:
 *
 *     grammar/DESIGN.md
 *
 * Normative general syntax:
 *
 *     grammar/spec/syntax.md
 *
 * Normative versioning contract:
 *
 *     grammar/spec/versioning.md
 *
 * Language-version semantics:
 *
 *     grammar/specification/language-version.md
 *
 * Compatibility:
 *
 *     grammar/spec/compatibility.md
 *
 * Release/version compatibility:
 *
 *     grammar/compatibility/versions.md
 *
 * Migration:
 *
 *     grammar/compatibility/migrations.md
 *
 * Deprecation:
 *
 *     grammar/compatibility/deprecated.md
 *
 * Canonical general version syntax:
 *
 *     grammar/core/versioning.g4
 *
 * Canonical names:
 *
 *     grammar/core/names.g4
 *     grammar/core/qualified-names.g4
 *
 * Module declaration:
 *
 *     grammar/modules/modules.g4
 *
 * Package declaration:
 *
 *     grammar/modules/packages.g4
 *
 * Dependency declaration:
 *
 *     grammar/modules/dependencies.g4
 *
 * Module aliases:
 *
 *     grammar/modules/aliases.g4
 *
 * Imports:
 *
 *     grammar/modules/imports.g4
 *
 * Exports:
 *
 *     grammar/modules/exports.g4
 *
 * Namespaces:
 *
 *     grammar/modules/namespaces.g4
 *
 * Visibility:
 *
 *     grammar/modules/visibility.g4
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It consumes:
 *
 *     ZamaniLexer
 *
 * and imports:
 *
 *     Versioning
 *     QualifiedNames
 *
 * The general version syntax MUST come from Versioning.
 *
 * Qualified module names MUST come from QualifiedNames.
 *
 * No lexical token is defined here.
 *
 * ============================================================================
 * MODULE VERSION MODEL
 * ============================================================================
 *
 * A module version is an association:
 *
 *     module identity
 *          +
 *     canonical version expression
 *
 * Examples:
 *
 *     module quantum::algorithms version 1.0.0;
 *
 *     module quantum::algorithms version 1.0.0-alpha;
 *
 *     module quantum::algorithms version 1.0.0+build42;
 *
 *     module quantum::algorithms version >= 1.0.0;
 *
 *     module quantum::algorithms version >= 1.0.0 and < 2.0.0;
 *
 * Whether a particular expression is legal as a DECLARED MODULE VERSION
 * rather than as a REQUIREMENT is a semantic question.
 *
 * This grammar deliberately preserves the distinction between:
 *
 *     declaration
 *     requirement
 *     compatibility request
 *
 * ============================================================================
 * MODULE VERSION DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     module <qualified-name> version <version-expression> ;
 *
 * Examples:
 *
 *     module math version 1.0.0;
 *
 *     module math::linear version 2.4.0;
 *
 *     module quantum::algorithms version 1.0.0;
 *
 *     module hdl::pipeline version 3.1.0;
 *
 *     module hardware::accelerators version 7.0.0;
 *
 * This rule is deliberately reusable independently of modules.g4.
 *
 * The surrounding module grammar decides where the clause can occur.
 *
 * ============================================================================
 * MODULE VERSION CLAUSE
 * ============================================================================
 *
 * A version clause is intentionally separate from the complete module
 * declaration.
 *
 * This permits modules.g4 to compose:
 *
 *     attributes
 *     visibility
 *     module
 *     name
 *     version
 *     body
 *
 * without moving version semantics into modules.g4.
 */
moduleVersionClause
    : VERSION versionExpression
    ;


/*
 * ============================================================================
 * MODULE VERSION DECLARATION
 * ============================================================================
 *
 * Complete standalone module-version declaration.
 *
 * This rule is useful to source-unit composition and compatibility tooling.
 *
 * It does NOT declare a module by itself.
 *
 * The module identity is supplied explicitly so the rule remains useful to
 * tooling that parses module-version records independently.
 */
moduleVersionDeclaration
    : MODULE moduleVersionIdentity moduleVersionClause SEMICOLON
    ;


/*
 * ============================================================================
 * MODULE VERSION IDENTITY
 * ============================================================================
 *
 * Module identity is a canonical qualified name.
 *
 * This grammar does not resolve the name.
 *
 * Examples:
 *
 *     math
 *     math::linear
 *     quantum::algorithms
 *     hardware::accelerators
 *     vendor::research::quantum
 */
moduleVersionIdentity
    : qualifiedName
    ;


/*
 * ============================================================================
 * MODULE VERSION REQUIREMENT
 * ============================================================================
 *
 * A module can require a version contract from another module without
 * confusing that requirement with the version of the current module.
 *
 * Example:
 *
 *     module quantum::algorithms requires quantum::runtime version >= 2.0.0;
 *
 * This is source-level intent.
 *
 * It does NOT:
 *
 *     - resolve the dependency;
 *     - download anything;
 *     - select a runtime;
 *     - select a QPU;
 *     - select hardware;
 *     - allocate resources.
 *
 * Dependency ownership remains with dependencies.g4.
 *
 * This rule exists as a module-context compatibility wrapper. If dependency
 * syntax already represents the required relationship, modules.g4 SHOULD NOT
 * expose this wrapper as a second dependency declaration.
 */
moduleVersionRequirement
    : moduleVersionRequirementTarget moduleVersionClause
    ;


moduleVersionRequirementTarget
    : qualifiedName
    ;


/*
 * ============================================================================
 * MODULE COMPATIBILITY CLAUSE
 * ============================================================================
 *
 * Compatibility is represented structurally.
 *
 * Examples:
 *
 *     compatible version >= 1.0.0
 *
 *     compatible version >= 1.0.0 and < 2.0.0
 *
 * The grammar does not determine whether the requested range is compatible.
 *
 * Semantic analysis owns that decision.
 */
moduleCompatibilityClause
    : COMPATIBLE VERSION versionExpression
    ;


/*
 * ============================================================================
 * MODULE LANGUAGE-COMPATIBILITY CLAUSE
 * ============================================================================
 *
 * A module may state the language-version contract under which it is valid.
 *
 * Example:
 *
 *     requires language version >= 1.2.0;
 *
 * This is deliberately different from:
 *
 *     module foo version 2.0.0;
 *
 * The first identifies a language compatibility requirement.
 *
 * The second identifies the module's own version.
 */
moduleLanguageCompatibilityClause
    : REQUIRES LANGUAGE VERSION versionExpression
    ;


/*
 * ============================================================================
 * MODULE GRAMMAR-COMPATIBILITY CLAUSE
 * ============================================================================
 *
 * A module may require a grammar contract.
 *
 * This identifies a grammar/tooling contract, not a machine target.
 *
 * Example:
 *
 *     requires grammar version >= 1.0.0;
 *
 * Semantic analysis determines whether the compiler can satisfy it.
 */
moduleGrammarCompatibilityClause
    : REQUIRES GRAMMAR VERSION versionExpression
    ;


/*
 * ============================================================================
 * MODULE DIALECT-COMPATIBILITY CLAUSE
 * ============================================================================
 *
 * Module syntax can identify a dialect contract without defining dialect
 * semantics here.
 *
 * Example:
 *
 *     requires dialect quantum::standard version >= 1.0.0;
 *
 * The dialect name is an ordinary qualified name.
 *
 * The dialect's actual semantics remain owned by dialect grammar and semantic
 * infrastructure.
 */
moduleDialectCompatibilityClause
    : REQUIRES DIALECT moduleDialectIdentity VERSION versionExpression
    ;


moduleDialectIdentity
    : qualifiedName
    ;


/*
 * ============================================================================
 * MODULE API-COMPATIBILITY CLAUSE
 * ============================================================================
 *
 * Example:
 *
 *     requires api quantum::runtime version >= 3.0.0;
 *
 * API versioning is independent of module versioning.
 */
moduleApiCompatibilityClause
    : REQUIRES API moduleContractIdentity VERSION versionExpression
    ;


/*
 * ============================================================================
 * MODULE ABI-COMPATIBILITY CLAUSE
 * ============================================================================
 *
 * Example:
 *
 *     requires abi quantum::runtime version >= 2.0.0;
 *
 * ABI compatibility is not source-language compatibility.
 *
 * This grammar only preserves the requested contract.
 */
moduleAbiCompatibilityClause
    : REQUIRES ABI moduleContractIdentity VERSION versionExpression
    ;


/*
 * ============================================================================
 * MODULE COMPILER-COMPATIBILITY CLAUSE
 * ============================================================================
 *
 * Example:
 *
 *     requires compiler version >= 1.0.0;
 *
 * This does not select a compiler executable.
 *
 * The compiler/toolchain resolves this requirement later.
 */
moduleCompilerCompatibilityClause
    : REQUIRES COMPILER VERSION versionExpression
    ;


/*
 * ============================================================================
 * MODULE RUNTIME-COMPATIBILITY CLAUSE
 * ============================================================================
 *
 * Example:
 *
 *     requires runtime version >= 1.0.0;
 *
 * This does not identify a particular runtime installation.
 */
moduleRuntimeCompatibilityClause
    : REQUIRES RUNTIME VERSION versionExpression
    ;


/*
 * ============================================================================
 * MODULE TARGET-CONTRACT CLAUSE
 * ============================================================================
 *
 * Target versioning is a contract-level concept.
 *
 * It MUST NOT become hardware identity syntax.
 *
 * Valid structural example:
 *
 *     requires target accelerator version >= 1.0.0;
 *
 * Invalid architectural interpretation:
 *
 *     requires target gpu0;
 *
 * The latter belongs to downstream target/resource semantics and is not
 * created by this grammar.
 */
moduleTargetCompatibilityClause
    : REQUIRES TARGET moduleContractIdentity VERSION versionExpression
    ;


/*
 * ============================================================================
 * GENERIC MODULE COMPATIBILITY REQUIREMENT
 * ============================================================================
 *
 * This is the open-world integration point for future contract classes.
 *
 * Known core subjects have dedicated rules above.
 *
 * A generic named contract permits future module-level contracts without
 * modifying this grammar for every new semantic subsystem.
 *
 * Example:
 *
 *     requires contract quantum::runtime version >= 1.0.0;
 *
 * The word `contract` is represented by the canonical identifier vocabulary
 * only if the lexer/specification permits it in this context. Therefore this
 * grammar intentionally does NOT introduce a new keyword for it.
 *
 * Instead, a generic contract subject can be expressed through:
 *
 *     moduleContractReference
 *
 * only where the surrounding specification explicitly enables that form.
 */
moduleContractRequirement
    : REQUIRES moduleContractReference VERSION versionExpression
    ;


moduleContractReference
    : qualifiedName
    ;


moduleContractIdentity
    : qualifiedName
    ;


/*
 * ============================================================================
 * MODULE VERSION METADATA
 * ============================================================================
 *
 * Version metadata is structural metadata.
 *
 * It does not redefine version syntax.
 *
 * Example conceptual form:
 *
 *     version {
 *         compatibility = ...;
 *         channel = stable;
 *     }
 *
 * This grammar deliberately keeps the metadata value structural.
 *
 * The exact metadata schema belongs to semantic/tooling specifications.
 */
moduleVersionMetadata
    : VERSION LBRACE moduleVersionMetadataEntry* RBRACE
    ;


moduleVersionMetadataEntry
    : identifier ASSIGN moduleVersionMetadataValue SEMICOLON?
    ;


moduleVersionMetadataValue
    : versionExpression
    | STRING
    | INTEGER
    | FLOAT
    | identifier
    ;


/*
 * ============================================================================
 * MODULE VERSION ANNOTATION WRAPPER
 * ============================================================================
 *
 * Generic attributes remain owned by the canonical attribute grammar.
 *
 * This wrapper exists only for tools that need to identify an attribute
 * occurring in a module-version context.
 *
 * The surrounding module grammar SHOULD normally use its canonical attribute
 * machinery rather than importing this wrapper.
 *
 * If Attributes is available to the composition layer, this rule may be
 * implemented there instead of being duplicated here.
 *
 * No new attribute syntax is defined by this file.
 */


/*
 * ============================================================================
 * MODULE VERSION CONTRACT
 * ============================================================================
 *
 * A complete module versioning contract can contain any number of:
 *
 *     version clause
 *     compatibility clause
 *     language compatibility requirement
 *     grammar compatibility requirement
 *     dialect compatibility requirement
 *     API compatibility requirement
 *     ABI compatibility requirement
 *     compiler compatibility requirement
 *     runtime compatibility requirement
 *     target contract requirement
 *
 * No fixed number is encoded.
 *
 * This rule is intentionally an integration wrapper rather than a second
 * module declaration grammar.
 */
moduleVersionContract
    : moduleVersionContractEntry*
    ;


moduleVersionContractEntry
    : moduleVersionClause
    | moduleCompatibilityClause
    | moduleLanguageCompatibilityClause
    | moduleGrammarCompatibilityClause
    | moduleDialectCompatibilityClause
    | moduleApiCompatibilityClause
    | moduleAbiCompatibilityClause
    | moduleCompilerCompatibilityClause
    | moduleRuntimeCompatibilityClause
    | moduleTargetCompatibilityClause
    | moduleContractRequirement
    | moduleVersionMetadata
    ;


/*
 * ============================================================================
 * MODULE VERSION REQUIREMENT LIST
 * ============================================================================
 *
 * Explicit non-empty list for consumers that require at least one
 * requirement.
 */
moduleVersionRequirementList
    : moduleVersionContractEntry+
    ;


/*
 * ============================================================================
 * OPTIONAL MODULE VERSION CONTRACT
 * ============================================================================
 */
optionalModuleVersionContract
    : moduleVersionContract?
    ;


/*
 * ============================================================================
 * VERSIONED MODULE HEADER
 * ============================================================================
 *
 * This is the stable integration rule modules.g4 should consume.
 *
 * It contains:
 *
 *     module keyword
 *     canonical module identity
 *     optional module version contract
 *
 * It deliberately does NOT consume:
 *
 *     visibility
 *     attributes
 *     body
 *     imports
 *     exports
 *     declarations
 *
 * Those remain owned by their canonical grammars.
 *
 * The recommended modules.g4 integration is therefore:
 *
 *     moduleDeclaration
 *         : moduleAttributes?
 *           visibilityModifier?
 *           K_MODULE
 *           moduleName
 *           moduleVersionTail
 *     ;
 *
 * where:
 *
 *     moduleVersionTail
 *         : moduleVersionContract? (SEMICOLON | moduleBody)
 *         ;
 *
 * This file provides the version portion only.
 */
versionedModuleHeader
    : MODULE moduleVersionIdentity optionalModuleVersionContract
    ;


/*
 * ============================================================================
 * MODULE VERSION TAIL
 * ============================================================================
 *
 * This rule is deliberately body-neutral.
 *
 * The canonical modules.g4 remains responsible for deciding whether the
 * declaration has:
 *
 *     ;
 *
 * or:
 *
 *     { ... }
 *
 * The version grammar must not create a second module-body grammar.
 */
moduleVersionTail
    : moduleVersionContract?
    ;


/*
 * ============================================================================
 * MODULE VERSION REFERENCE
 * ============================================================================
 *
 * A reference identifies a module whose version will be resolved later.
 *
 * Example:
 *
 *     quantum::runtime
 *
 * This is intentionally just a canonical qualified name.
 */
moduleVersionReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * MODULE VERSION CONSTRAINT
 * ============================================================================
 *
 * A reusable wrapper around the canonical version-expression grammar.
 *
 * It exists so module-specific tooling can identify a version constraint
 * without owning its syntax.
 */
moduleVersionConstraint
    : versionExpression
    ;


/*
 * ============================================================================
 * MODULE VERSION RANGE
 * ============================================================================
 *
 * Compatibility wrapper only.
 *
 * The actual range syntax remains owned by core/versioning.g4.
 */
moduleVersionRange
    : versionRange
    ;


/*
 * ============================================================================
 * MODULE EXACT VERSION
 * ============================================================================
 *
 * Compatibility wrapper only.
 *
 * The actual exact-version syntax remains owned by core/versioning.g4.
 */
moduleExactVersion
    : exactVersion
    ;


/*
 * ============================================================================
 * MODULE VERSION CHANNEL
 * ============================================================================
 *
 * Compatibility wrapper only.
 *
 * Examples:
 *
 *     stable
 *     beta
 *     alpha
 *     rc
 *     dev
 *
 * The semantic interpretation remains downstream.
 */
moduleVersionChannel
    : versionChannel
    ;


/*
 * ============================================================================
 * MODULE VERSION REFERENCE EXPRESSION
 * ============================================================================
 *
 * Compatibility wrapper around the canonical version-reference syntax.
 */
moduleVersionReferenceExpression
    : versionReference
    ;


/*
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * This grammar establishes structure only.
 *
 * Semantic analysis owns:
 *
 *     - module identity resolution;
 *     - module-version identity;
 *     - version normalization;
 *     - version comparison;
 *     - compatibility;
 *     - satisfiability;
 *     - dependency resolution;
 *     - conflict detection;
 *     - language compatibility;
 *     - grammar compatibility;
 *     - dialect compatibility;
 *     - API compatibility;
 *     - ABI compatibility;
 *     - compiler compatibility;
 *     - runtime compatibility;
 *     - target-contract compatibility;
 *     - migration;
 *     - deprecation;
 *     - feature availability.
 *
 * A parser MUST NOT decide:
 *
 *     1.0.0 < 2.0.0
 *
 * or:
 *
 *     >= 1.0.0 and < 2.0.0
 *
 * is satisfiable.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must preserve, as applicable:
 *
 *     - module version clause;
 *     - module identity;
 *     - version-expression structure;
 *     - compatibility subject;
 *     - requested version expression;
 *     - metadata ordering;
 *     - source spans;
 *     - source spelling/provenance where required.
 *
 * Conceptually:
 *
 *     ModuleVersion
 *         module
 *         version
 *         source_span
 *
 *     ModuleCompatibilityRequirement
 *         subject
 *         version
 *         source_span
 *
 * Exact Rust AST names belong to:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT define Rust AST types.
 *
 * It MUST NOT create:
 *
 *     ModuleVersionIR
 *     QuantumModuleVersionIR
 *     HardwareModuleVersionIR
 *
 * or any equivalent competing representation.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Module version information is compilation metadata.
 *
 * It does not directly become an execution operation.
 *
 * The information is consumed by:
 *
 *     module/package semantic analysis
 *             |
 *             v
 *     dependency/compatibility model
 *             |
 *             v
 *     canonical semantic model
 *             |
 *             v
 *     existing domain IRs
 *
 * Quantum code remains:
 *
 *     quantum semantic model
 *          |
 *          v
 *     quantum::ir
 *
 * Module versioning MUST NOT create another quantum IR.
 *
 * ============================================================================
 * PACKAGE INTEGRATION
 * ============================================================================
 *
 * Package versions are independent from module versions.
 *
 * packages.g4 owns package declaration syntax.
 *
 * dependencies.g4 owns dependency declaration syntax.
 *
 * This grammar does not redefine either.
 *
 * A semantic package graph MAY associate:
 *
 *     package
 *       |
 *       +--> modules
 *       |
 *       +--> dependencies
 *       |
 *       +--> dialects
 *       |
 *       +--> APIs
 *
 * but that relationship is semantic/toolchain information.
 *
 * ============================================================================
 * DEPENDENCY INTEGRATION
 * ============================================================================
 *
 * `dependencies.g4` already owns dependency-version syntax.
 *
 * Therefore:
 *
 *     dependency foo version >= 1.0.0;
 *
 * remains a dependency construct.
 *
 * This grammar MUST NOT replace it.
 *
 * If a module-version requirement is merely expressing a dependency, the
 * aggregate grammar SHOULD use dependencies.g4 rather than exposing two
 * equivalent source syntaxes.
 *
 * The dedicated module-version requirement rules in this file are therefore
 * intended for MODULE CONTRACT metadata, not for duplicating dependency
 * declarations.
 *
 * ============================================================================
 * MODULE INTEGRATION
 * ============================================================================
 *
 * `grammar/modules/modules.g4` remains the sole owner of:
 *
 *     moduleDeclaration
 *     moduleDeclarationTail
 *     moduleName
 *     moduleBody
 *
 * It SHOULD import:
 *
 *     ModuleVersioning
 *
 * and use:
 *
 *     moduleVersionContract
 *
 * as an optional header component.
 *
 * Recommended final composition:
 *
 *     moduleDeclaration
 *         : moduleAttributes?
 *           visibilityModifier?
 *           K_MODULE
 *           moduleName
 *           moduleVersionContract?
 *           moduleDeclarationTail
 *         ;
 *
 * The exact keyword token MUST follow the repository's current canonical
 * lexer vocabulary. The existing modules.g4 currently uses its canonical
 * module keyword token; this file does not rename it.
 *
 * No second module declaration rule should be introduced.
 *
 * ============================================================================
 * IMPORT / EXPORT INTEGRATION
 * ============================================================================
 *
 * imports.g4 and exports.g4 remain independent.
 *
 * A module version does not alter import/export syntax.
 *
 * Semantic analysis may use the module's effective version when resolving
 * imports and exports.
 *
 * The parser must not perform that resolution.
 *
 * ============================================================================
 * NAMESPACE INTEGRATION
 * ============================================================================
 *
 * namespaces.g4 remains the owner of namespace declarations.
 *
 * A module version does not turn a module name into a namespace or package.
 *
 * For example:
 *
 *     quantum::algorithms
 *
 * remains a canonical qualified name.
 *
 * Its relationship to:
 *
 *     namespace
 *     module
 *     package
 *     dialect
 *
 * is semantic.
 *
 * ============================================================================
 * ALIAS INTEGRATION
 * ============================================================================
 *
 * aliases.g4 remains the owner of module-alias syntax.
 *
 * A module alias does not change the module's version.
 *
 * Example:
 *
 *     module alias q = quantum::algorithms;
 *
 * and:
 *
 *     module quantum::algorithms version 2.0.0;
 *
 * remain distinct concepts.
 *
 * Alias resolution uses the effective module version downstream.
 *
 * ============================================================================
 * VISIBILITY INTEGRATION
 * ============================================================================
 *
 * visibility.g4 remains the sole owner of visibility syntax.
 *
 * This file MUST NOT define:
 *
 *     public
 *     private
 *     protected
 *     internal
 *
 * or another visibility vocabulary.
 *
 * Module version clauses are independent from visibility.
 *
 * ============================================================================
 * DIALECT INTEGRATION
 * ============================================================================
 *
 * dialects/versioning.g4 is an adapter around core/versioning.g4.
 *
 * It MUST consume the same version-expression model.
 *
 * A module may require a dialect version:
 *
 *     requires dialect quantum::standard version >= 1.0.0;
 *
 * without making the dialect version part of the module's own version.
 *
 * ============================================================================
 * LANGUAGE-VERSION INTEGRATION
 * ============================================================================
 *
 * Language version is owned by the language-version specification and
 * core/versioning.g4.
 *
 * A module compatibility declaration such as:
 *
 *     requires language version >= 1.2.0;
 *
 * is a REQUIREMENT on the language contract.
 *
 * It does not declare:
 *
 *     module version 1.2.0
 *
 * These must remain different AST/semantic concepts.
 *
 * ============================================================================
 * API / ABI INTEGRATION
 * ============================================================================
 *
 * API and ABI versions remain independent.
 *
 * API compatibility may affect:
 *
 *     symbol contracts
 *     callable interfaces
 *     data contracts
 *
 * ABI compatibility may affect:
 *
 *     representation
 *     calling conventions
 *     layout
 *     binary interoperability
 *
 * This grammar merely preserves the requested version contracts.
 *
 * ============================================================================
 * COMPILER / RUNTIME INTEGRATION
 * ============================================================================
 *
 * Compiler and runtime version requirements do not select concrete
 * implementations.
 *
 * They are compatibility constraints consumed by toolchain resolution.
 *
 * They must never become:
 *
 *     compiler-id
 *     runtime-id
 *     device-id
 *
 * syntax.
 *
 * ============================================================================
 * TARGET INTEGRATION
 * ============================================================================
 *
 * Target contracts remain distinct from hardware identity.
 *
 * A target compatibility expression may identify a semantic target contract.
 *
 * It must NOT encode:
 *
 *     CPU model;
 *     GPU model;
 *     FPGA part number;
 *     ASIC instance;
 *     QPU device ID;
 *     physical qubit;
 *     memory bank;
 *     node address;
 *     topology;
 *     deployment location.
 *
 * Resource and capability requirements belong to:
 *
 *     grammar/resources/
 *     grammar/hardware/
 *     grammar/compile/
 *     grammar/execution/
 *
 * Target realization belongs downstream.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Module versioning is quantum-neutral.
 *
 * These are all ordinary module identities:
 *
 *     quantum::algorithms
 *     quantum::runtime
 *     quantum::error_correction
 *     quantum::hardware
 *
 * A module version does NOT:
 *
 *     - identify a QPU;
 *     - identify physical qubits;
 *     - select a gate set;
 *     - select a topology;
 *     - select calibration;
 *     - select QEC;
 *     - create quantum::ir.
 *
 * Quantum semantics remain:
 *
 *     source
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     QEC / resilience / ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     hardware
 *
 * ============================================================================
 * CLASSICAL / HDL / HYBRID / OTHER DOMAIN INTEGRATION
 * ============================================================================
 *
 * The module-version grammar is intentionally domain-neutral.
 *
 * It applies equally to:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware/software co-design
 *     distributed
 *     HPC
 *     AI/ML
 *     data
 *     networking
 *     security
 *     embedded
 *     accelerators
 *     scientific computing
 *     future domains
 *
 * Adding a new computational domain MUST NOT require changing this grammar
 * merely to recognize its module version.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Module versioning imposes NO limits on:
 *
 *     - number of modules;
 *     - module graph size;
 *     - module nesting depth;
 *     - qualified-name depth;
 *     - dependency count;
 *     - version requirements;
 *     - compatibility clauses;
 *     - dialects;
 *     - APIs;
 *     - ABIs;
 *     - target contracts;
 *     - source size;
 *     - quantum resources;
 *     - classical resources;
 *     - hardware resources;
 *     - distributed nodes;
 *     - accelerators.
 *
 * There are no grammar constants such as:
 *
 *     MAX_MODULES
 *     MAX_MODULE_VERSION_REQUIREMENTS
 *     MAX_VERSION_COMPONENT
 *     MAX_DEPENDENCIES
 *     MAX_DIALECTS
 *     MAX_TARGETS
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *
 * Version components may be represented by the canonical version grammar
 * without imposing language-level magnitude limits.
 *
 * Practical limits imposed by:
 *
 *     compiler memory
 *     parser stack
 *     operating system
 *     deployment
 *     resource availability
 *
 * remain implementation/resource constraints.
 *
 * They MUST NOT be encoded in this grammar.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no environment access;
 *     - no randomness;
 *     - no hardware discovery;
 *     - no resource discovery;
 *     - no target selection;
 *     - no runtime execution.
 *
 * Given the same:
 *
 *     source token stream
 *     grammar version
 *
 * the syntactic result MUST be deterministic.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Version strings and identifiers are untrusted source input.
 *
 * Parsing them MUST NOT:
 *
 *     - execute code;
 *     - load packages;
 *     - access URLs;
 *     - access the filesystem;
 *     - invoke registries;
 *     - access credentials;
 *     - select hardware.
 *
 * Semantic/toolchain resolution MUST remain a separate, explicitly controlled
 * operation.
 *
 * ============================================================================
 * SOURCE-PRESERVATION
 * ============================================================================
 *
 * The parser must preserve enough information for:
 *
 *     - diagnostics;
 *     - formatter;
 *     - LSP;
 *     - migration;
 *     - compatibility tooling;
 *     - provenance;
 *     - source maps;
 *     - reproducible builds.
 *
 * Where the AST requires it, the original spelling of:
 *
 *     version expressions
 *     module names
 *     compatibility subjects
 *
 * must remain recoverable from source spans/token information.
 *
 * ============================================================================
 * ERROR MODEL
 * ============================================================================
 *
 * Syntax errors should be deterministic and localized.
 *
 * Examples that should fail structurally:
 *
 *     module foo version;
 *
 *     module foo version = 1.0.0;
 *
 *     module foo version 1..0;
 *
 *     module foo version >=;
 *
 *     requires language version;
 *
 *     requires dialect version 1.0.0;
 *
 *     requires compiler version;
 *
 *     requires runtime version;
 *
 *     requires api version 1.0.0;
 *
 *     requires abi version;
 *
 * Semantic errors include:
 *
 *     unsatisfiable version constraints
 *     unsupported version
 *     conflicting module versions
 *     duplicate module identity
 *     incompatible language version
 *     incompatible dialect
 *     incompatible API
 *     incompatible ABI
 *
 * Those MUST NOT be implemented as parser predicates.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This new grammar component is compatible with the repository's existing
 * versioning architecture because it:
 *
 *     1. reuses core/versioning.g4;
 *     2. does not redefine version syntax;
 *     3. does not redefine names;
 *     4. does not redefine dependencies;
 *     5. does not redefine packages;
 *     6. does not redefine module declarations;
 *     7. does not redefine visibility;
 *     8. does not redefine attributes;
 *     9. does not introduce target-specific syntax;
 *    10. does not create a second IR.
 *
 * Existing package/dependency version syntax remains owned by their existing
 * grammar components.
 *
 * Existing module syntax remains valid when modules.g4 integrates this
 * component as an OPTIONAL version clause.
 *
 * ============================================================================
 * MIGRATION
 * ============================================================================
 *
 * Migration from older module syntax must preserve:
 *
 *     module identity
 *     module version
 *     compatibility requirements
 *     dependency semantics
 *
 * A migration MUST NOT silently transform:
 *
 *     module version
 *
 * into:
 *
 *     language version
 *
 * or:
 *
 *     package version
 *
 * or:
 *
 *     compiler version
 *
 * or:
 *
 *     hardware target version.
 *
 * Migration logic belongs to:
 *
 *     grammar/compatibility/migrations.md
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * ============================================================================
 *
 *     module foo version 1.0.0;
 *
 *     module foo::bar version 2.4.7;
 *
 *     module quantum::algorithms version 1.0.0-alpha;
 *
 *     module quantum::algorithms version 1.0.0-alpha+build42;
 *
 *     module hardware::accelerators version 7.2.0;
 *
 *     requires language version >= 1.0.0;
 *
 *     requires grammar version >= 1.0.0;
 *
 *     requires compiler version >= 1.0.0;
 *
 *     requires runtime version >= 1.0.0;
 *
 *     requires dialect quantum::standard version >= 1.0.0;
 *
 *     requires api quantum::runtime version >= 2.0.0;
 *
 *     requires abi quantum::runtime version >= 2.0.0;
 *
 *     requires target accelerator version >= 1.0.0;
 *
 *     compatible version >= 1.0.0 and < 2.0.0;
 *
 *
 * ============================================================================
 * NEGATIVE TESTS
 * ============================================================================
 *
 *     module version 1.0.0;
 *
 *     module foo version;
 *
 *     module foo version = 1.0.0;
 *
 *     module foo version >=;
 *
 *     module foo version 1..0;
 *
 *     requires language version;
 *
 *     requires dialect version 1.0.0;
 *
 *     requires api version;
 *
 *     requires abi version;
 *
 *     requires runtime version;
 *
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * The test suite MUST cover:
 *
 *     - smallest representable version;
 *     - very large numeric version components;
 *     - long pre-release identifiers;
 *     - long build identifiers;
 *     - deeply qualified module names;
 *     - many compatibility clauses;
 *     - many version requirements;
 *     - many modules;
 *     - nested module graphs;
 *     - mixed classical/quantum/HDL modules;
 *     - large distributed module graphs;
 *     - future dialect identifiers.
 *
 * These are parser/resource tests, not language-level maxima.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * The grammar MUST be tested with:
 *
 *     one module
 *     many modules
 *     deeply qualified modules
 *     many version requirements
 *     many dialect requirements
 *     many compatibility contracts
 *     large source units
 *     large quantum programs
 *     large classical programs
 *     large HDL programs
 *     large distributed programs
 *
 * The implementation may impose configurable operational resource budgets,
 * but those budgets MUST NOT appear in this grammar.
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * Identical:
 *
 *     source
 *     token vocabulary
 *     grammar version
 *
 * MUST yield identical parse structure.
 *
 * Tests MUST verify that parsing is independent of:
 *
 *     CPU count;
 *     GPU count;
 *     QPU availability;
 *     memory capacity;
 *     network availability;
 *     filesystem state;
 *     runtime state;
 *     random state.
 *
 * ============================================================================
 * ROUND-TRIP TESTS
 * ============================================================================
 *
 * Where source serialization exists:
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
 *     AST
 *       |
 *       v
 *     serializer
 *       |
 *       v
 *     source
 *
 * must preserve the intended module-version semantics.
 *
 * ============================================================================
 * CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * Versioning MUST work identically for modules containing:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware intent
 *     distributed computation
 *     AI/ML
 *     data
 *     networking
 *     security
 *     accelerator intent
 *     future dialects.
 *
 * No domain may require a separate version grammar.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST NOT contain:
 *
 *     MAX_MODULES
 *     MAX_MODULE_DEPTH
 *     MAX_VERSION
 *     MAX_VERSION_COMPONENT
 *     MAX_VERSION_REQUIREMENTS
 *     MAX_DEPENDENCIES
 *     MAX_DIALECTS
 *     MAX_TARGETS
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_ACCELERATORS
 *
 * Numeric literals in a version are PROGRAM/CONTRACT DATA.
 *
 * For example:
 *
 *     1024.2048.4096
 *
 * must not be interpreted as a hardware limit merely because the numbers are
 * large.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust code.
 *
 * Generated parser/frontend integration MUST:
 *
 *     - compile on Rust 1.97;
 *     - compile on Rust 1.97.1;
 *     - remain Rust 2021 compatible;
 *     - use safe Rust only;
 *     - require no unsafe;
 *     - preserve source spans;
 *     - preserve deterministic behavior;
 *     - avoid machine-specific assumptions.
 *
 * Rust version MUST NOT become a Zamani language-version component.
 *
 * ============================================================================
 * VALIDATION CONTRACT
 * ============================================================================
 *
 * grammar/validation/ MUST verify:
 *
 *     - grammar name matches filename;
 *     - ModuleVersioning is a parser grammar;
 *     - tokenVocab is the canonical Zamani lexer;
 *     - core Versioning is the sole owner of general version syntax;
 *     - QualifiedNames is the sole owner of qualified-name syntax;
 *     - no duplicate version grammar exists here;
 *     - no duplicate identifier grammar exists here;
 *     - no module-body grammar exists here;
 *     - no dependency grammar is duplicated here;
 *     - no package grammar is duplicated here;
 *     - no semantic predicates exist;
 *     - no actions exist;
 *     - no unsafe Rust is embedded;
 *     - no hardware limits exist;
 *     - no quantum limits exist;
 *     - no fixed module count exists;
 *     - no fixed version-component magnitude exists;
 *     - no second IR is introduced.
 *
 * ============================================================================
 * FILE COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is independently complete when:
 *
 *     [x] module version syntax is contextually defined;
 *     [x] general version syntax is delegated to core/versioning.g4;
 *     [x] module names are delegated to canonical qualified names;
 *     [x] module language compatibility is represented;
 *     [x] module grammar compatibility is represented;
 *     [x] module dialect compatibility is represented;
 *     [x] module API compatibility is represented;
 *     [x] module ABI compatibility is represented;
 *     [x] module compiler compatibility is represented;
 *     [x] module runtime compatibility is represented;
 *     [x] module target-contract compatibility is represented;
 *     [x] package versions remain separate;
 *     [x] dependency versions remain separate;
 *     [x] language versions remain separate;
 *     [x] dialect versions remain separate;
 *     [x] module aliases remain separate;
 *     [x] visibility remains separately owned;
 *     [x] no version comparison is performed;
 *     [x] no compatibility decision is performed;
 *     [x] no dependency solving is performed;
 *     [x] no hardware is selected;
 *     [x] no resource is allocated;
 *     [x] no quantum IR is created;
 *     [x] no machine-size limits exist;
 *     [x] no unsafe Rust is required;
 *     [x] Rust 1.97 / 1.97.1 integration is specified;
 *     [x] positive tests are specified;
 *     [x] negative tests are specified;
 *     [x] boundary tests are specified;
 *     [x] scalability tests are specified;
 *     [x] determinism tests are specified;
 *     [x] cross-domain tests are specified;
 *     [x] hard-coding audit is specified;
 *     [x] downstream integration is specified in advance.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * Module versioning answers:
 *
 *     "Which source-level module contract/version does this module expose
 *      or require?"
 *
 * It does NOT answer:
 *
 *     "Which machine runs this module?"
 *
 *     "Which CPU/GPU/FPGA/QPU is selected?"
 *
 *     "How many resources are available?"
 *
 *     "Which topology is used?"
 *
 *     "How is quantum computation physically realized?"
 *
 * Those decisions remain downstream.
 *
 * Therefore:
 *
 *     one source program
 *          |
 *          v
 *     one module semantic contract
 *          |
 *          v
 *     one canonical semantic model
 *          |
 *          +-------------------+
 *          |                   |
 *          v                   v
 *     classical IR        quantum::ir
 *          |                   |
 *          +---------+---------+
 *                    |
 *                    v
 *             optimization
 *                    |
 *             routing/scheduling
 *                    |
 *              QEC/resilience/ZQN
 *                    |
 *                    v
 *                   HAL
 *                    |
 *                    v
 *             target realization
 *
 * Module versioning therefore remains compatible with:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * subject only to the actual semantic contract, implementation compatibility,
 * and resources/capabilities available at realization time.
 *
 * ============================================================================
 */

parser grammar ModuleVersioning;

options {
    tokenVocab = ZamaniLexer;
}

import Versioning, QualifiedNames;


/*
 * ============================================================================
 * MODULE VERSION CLAUSE
 * ============================================================================
 *
 * Version syntax is owned by core/versioning.g4.
 *
 * This rule owns only its association with the module context.
 */
moduleVersionClause
    : VERSION versionExpression
    ;


/*
 * ============================================================================
 * STANDALONE MODULE VERSION DECLARATION
 * ============================================================================
 *
 * This is a reusable structural form for tools and source-unit composition.
 *
 * The normal module declaration remains owned by modules.g4.
 */
moduleVersionDeclaration
    : MODULE moduleVersionIdentity moduleVersionClause SEMICOLON
    ;


moduleVersionIdentity
    : qualifiedName
    ;


/*
 * ============================================================================
 * MODULE VERSION REQUIREMENT
 * ============================================================================
 *
 * This rule is intentionally separate from dependencyDeclaration.
 *
 * A dependency declaration remains owned by dependencies.g4.
 *
 * This rule is appropriate only where a module contract explicitly refers
 * to another module's version without becoming a dependency declaration.
 */
moduleVersionRequirement
    : moduleVersionRequirementTarget moduleVersionClause
    ;


moduleVersionRequirementTarget
    : qualifiedName
    ;


/*
 * ============================================================================
 * MODULE COMPATIBILITY
 * ============================================================================
 */

moduleCompatibilityClause
    : COMPATIBLE VERSION versionExpression
    ;


/*
 * ============================================================================
 * LANGUAGE COMPATIBILITY
 * ============================================================================
 */

moduleLanguageCompatibilityClause
    : REQUIRES LANGUAGE VERSION versionExpression
    ;


/*
 * ============================================================================
 * GRAMMAR COMPATIBILITY
 * ============================================================================
 */

moduleGrammarCompatibilityClause
    : REQUIRES GRAMMAR VERSION versionExpression
    ;


/*
 * ============================================================================
 * DIALECT COMPATIBILITY
 * ============================================================================
 */

moduleDialectCompatibilityClause
    : REQUIRES DIALECT moduleDialectIdentity VERSION versionExpression
    ;


moduleDialectIdentity
    : qualifiedName
    ;


/*
 * ============================================================================
 * API COMPATIBILITY
 * ============================================================================
 */

moduleApiCompatibilityClause
    : REQUIRES API moduleContractIdentity VERSION versionExpression
    ;


/*
 * ============================================================================
 * ABI COMPATIBILITY
 * ============================================================================
 */

moduleAbiCompatibilityClause
    : REQUIRES ABI moduleContractIdentity VERSION versionExpression
    ;


/*
 * ============================================================================
 * COMPILER COMPATIBILITY
 * ============================================================================
 */

moduleCompilerCompatibilityClause
    : REQUIRES COMPILER VERSION versionExpression
    ;


/*
 * ============================================================================
 * RUNTIME COMPATIBILITY
 * ============================================================================
 */

moduleRuntimeCompatibilityClause
    : REQUIRES RUNTIME VERSION versionExpression
    ;


/*
 * ============================================================================
 * TARGET-CONTRACT COMPATIBILITY
 * ============================================================================
 *
 * The target name is semantic contract data, not a physical device identity.
 */
moduleTargetCompatibilityClause
    : REQUIRES TARGET moduleContractIdentity VERSION versionExpression
    ;


moduleContractIdentity
    : qualifiedName
    ;


/*
 * ============================================================================
 * GENERIC CONTRACT REQUIREMENT
 * ============================================================================
 *
 * This is an intentionally open structural extension point.
 *
 * It does not create a new keyword vocabulary.
 *
 * The first name identifies the contract namespace/name; semantic analysis
 * determines whether that contract is known and what it means.
 *
 * Example:
 *
 *     requires quantum::runtime version >= 1.0.0;
 *
 * This rule should be used only by an aggregate grammar that has explicitly
 * selected the generic contract interpretation.
 */
moduleContractRequirement
    : REQUIRES moduleContractReference VERSION versionExpression
    ;


moduleContractReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * MODULE VERSION METADATA
 * ============================================================================
 *
 * Metadata is structural and intentionally open-ended.
 *
 * Version semantics still come from Versioning.
 */
moduleVersionMetadata
    : VERSION LBRACE moduleVersionMetadataEntry* RBRACE
    ;


moduleVersionMetadataEntry
    : identifier ASSIGN moduleVersionMetadataValue SEMICOLON?
    ;


moduleVersionMetadataValue
    : versionExpression
    | STRING
    | INTEGER
    | FLOAT
    | identifier
    ;


/*
 * ============================================================================
 * MODULE VERSION CONTRACT
 * ============================================================================
 *
 * This is the principal integration rule for modules.g4.
 *
 * It contains zero or more module-versioning constructs.
 *
 * No finite number is imposed.
 */
moduleVersionContract
    : moduleVersionContractEntry*
    ;


moduleVersionContractEntry
    : moduleVersionClause
    | moduleCompatibilityClause
    | moduleLanguageCompatibilityClause
    | moduleGrammarCompatibilityClause
    | moduleDialectCompatibilityClause
    | moduleApiCompatibilityClause
    | moduleAbiCompatibilityClause
    | moduleCompilerCompatibilityClause
    | moduleRuntimeCompatibilityClause
    | moduleTargetCompatibilityClause
    | moduleContractRequirement
    | moduleVersionMetadata
    ;


moduleVersionRequirementList
    : moduleVersionContractEntry+
    ;


optionalModuleVersionContract
    : moduleVersionContract?
    ;


/*
 * ============================================================================
 * STABLE MODULE HEADER ADAPTER
 * ============================================================================
 *
 * This rule is intentionally NOT the module declaration.
 *
 * modules.g4 remains the owner of:
 *
 *     visibility
 *     module keyword
 *     module body
 *     declaration termination
 *
 * This adapter only provides a stable version-aware header boundary.
 */
versionedModuleHeader
    : MODULE moduleVersionIdentity optionalModuleVersionContract
    ;


/*
 * ============================================================================
 * VERSION-COMPATIBILITY WRAPPERS
 * ============================================================================
 *
 * These wrappers allow tooling to refer to module-specific version concepts
 * without creating another version syntax.
 */
moduleVersionConstraint
    : versionExpression
    ;


moduleVersionRange
    : versionRange
    ;


moduleExactVersion
    : exactVersion
    ;


moduleVersionChannel
    : versionChannel
    ;


moduleVersionReferenceExpression
    : versionReference
    ;