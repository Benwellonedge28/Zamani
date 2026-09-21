/**
 * Zamani — Dependency Grammar
 *
 * File:
 *   grammar/modules/dependencies.g4
 *
 * Purpose:
 *   Defines the portable source-language syntax for declaring dependencies
 *   and dependency requirements.
 *
 * Architectural role:
 *   Dependency declarations are source-level intent. This grammar records
 *   what a program depends on and the constraints/preferences associated
 *   with that dependency.
 *
 * This grammar DOES:
 *   - define dependency declarations;
 *   - define dependency identity;
 *   - define dependency aliases;
 *   - define optional dependency declarations;
 *   - define dependency version constraints;
 *   - define dependency source specifications;
 *   - define dependency features/options;
 *   - define dependency capability requirements;
 *   - define dependency metadata;
 *   - provide unbounded lists/sets/maps through repetition;
 *   - preserve target-independent dependency intent.
 *
 * This grammar DOES NOT:
 *   - resolve dependencies;
 *   - access a registry;
 *   - access a filesystem;
 *   - access a network;
 *   - download packages;
 *   - select a compiler;
 *   - select hardware;
 *   - select a CPU/GPU/FPGA/QPU;
 *   - assign physical devices;
 *   - determine a dependency's implementation;
 *   - lower dependencies to quantum::ir;
 *   - perform linking;
 *   - perform runtime loading;
 *   - impose maximum dependency counts;
 *   - impose maximum dependency-tree depth;
 *   - impose maximum feature counts;
 *   - impose maximum version components;
 *   - impose hardware/resource limits.
 *
 * Integration:
 *
 *   source
 *      |
 *      v
 *   Zamani.g4
 *      |
 *      +--> packageDeclaration      (packages.g4)
 *      +--> moduleDeclaration       (modules.g4)
 *      +--> importDeclaration       (imports.g4)
 *      +--> exportDeclaration       (exports.g4)
 *      +--> dependencyDeclaration   (this grammar)
 *      |
 *      v
 *   Frontend AST
 *      |
 *      v
 *   semantic dependency graph
 *      |
 *      +--> package/module resolver
 *      +--> version resolver
 *      +--> registry/filesystem/network tooling
 *      +--> build graph
 *      +--> compiler
 *
 * Dependencies are therefore semantic/build-graph input and are not an IR
 * replacement for classical, quantum, HDL, AI, distributed, or hardware
 * semantics.
 *
 * Rust compatibility:
 *   This grammar contains no target-specific actions or embedded code.
 *   Generated/parser integration must remain compatible with Rust 1.97 /
 *   Rust 1.97.1 and must not require unsafe Rust.
 *
 * Canonical lexer:
 *   ZamaniLexer
 *
 * Important:
 *   Do not introduce a second token vocabulary such as ZamaniTokens here.
 *
 * Ownership:
 *   Dependencies.g4 owns dependency declaration syntax only.
 *
 * Non-ownership:
 *   packages.g4 owns package declarations.
 *   modules.g4 owns module declarations.
 *   imports.g4 owns imports.
 *   exports.g4 owns exports.
 *   QualifiedNames owns qualified-name syntax.
 *   semantic analysis owns dependency validation/resolution semantics.
 *   compiler/build tooling owns actual dependency resolution.
 */

parser grammar Dependencies;

options {
    tokenVocab = ZamaniLexer;
}

import QualifiedNames;


/* ============================================================================
 * TOP-LEVEL DEPENDENCY DECLARATION
 * ========================================================================== */

/**
 * A dependency declaration.
 *
 * Examples:
 *
 *   dependency core.math;
 *
 *   dependency core.math version ">=1";
 *
 *   dependency core.math version ">=1" as math;
 *
 *   dependency quantum.runtime {
 *       version ">=1";
 *       feature "measurement";
 *   }
 *
 *   dependency optional.ml;
 *
 * The grammar deliberately does not limit the number of dependency
 * declarations in a source unit.
 */
dependencyDeclaration
    : DEPENDENCY dependencySpecification SEMICOLON?
    ;


/**
 * Complete dependency specification.
 *
 * The dependency identity is mandatory.
 *
 * Everything following the identity is optional and may be expressed
 * either inline or through a dependency body.
 */
dependencySpecification
    : dependencyIdentity
      dependencyInlineClause*
      dependencyBody?
    ;


/* ============================================================================
 * DEPENDENCY IDENTITY
 * ========================================================================== */

/**
 * Canonical dependency identity.
 *
 * A dependency is identified using the same qualified-name model used by
 * the rest of the module system.
 *
 * This avoids creating a competing package/module naming grammar.
 */
dependencyIdentity
    : qualifiedName
    ;


/* ============================================================================
 * INLINE CLAUSES
 * ========================================================================== */

/**
 * Inline dependency clauses.
 *
 * Each clause has one semantic responsibility.
 *
 * The order is intentionally unrestricted so that source formatting does
 * not become semantic.
 */
dependencyInlineClause
    : dependencyVersionClause
    | dependencyAliasClause
    | dependencyOptionalClause
    | dependencySourceClause
    | dependencyFeatureClause
    | dependencyCapabilityClause
    | dependencyMetadataClause
    ;


/* ============================================================================
 * VERSION CONSTRAINTS
 * ========================================================================== */

/**
 * Version requirement.
 *
 * Version syntax is represented as source data rather than as a fixed
 * three-component grammar such as:
 *
 *   major "." minor "." patch
 *
 * This permits arbitrary versioning schemes and prevents the grammar from
 * imposing artificial limits on version structure.
 */
dependencyVersionClause
    : VERSION dependencyVersionConstraint
    ;


/**
 * A version constraint is represented as a string literal.
 *
 * The semantic/versioning subsystem interprets the value according to
 * the dependency ecosystem and declared compatibility rules.
 *
 * Examples:
 *
 *   "1"
 *   "1.2"
 *   "1.2.3"
 *   ">=1"
 *   ">=1,<2"
 *   "^2"
 *   "~3.4"
 *   "stable"
 *   "rolling"
 *
 * The grammar deliberately does not hard-code one versioning algorithm.
 */
dependencyVersionConstraint
    : STRING_LITERAL
    ;


/* ============================================================================
 * ALIASES
 * ========================================================================== */

/**
 * Local dependency alias.
 *
 * The alias affects source-level name lookup only.
 *
 * It does not change dependency identity.
 */
dependencyAliasClause
    : AS identifier
    ;


/* ============================================================================
 * OPTIONAL DEPENDENCIES
 * ========================================================================== */

/**
 * Marks a dependency as optional.
 *
 * Whether an optional dependency is selected is a semantic/build decision,
 * not a parser decision.
 */
dependencyOptionalClause
    : OPTIONAL
    ;


/* ============================================================================
 * SOURCE SPECIFICATION
 * ========================================================================== */

/**
 * Dependency source.
 *
 * Source syntax is intentionally represented as data.
 *
 * The compiler/build system decides how to resolve the source.
 *
 * Examples:
 *
 *   source "registry://..."
 *   source "git://..."
 *   source "https://..."
 *   source "path://..."
 *   source "file://..."
 *   source "vendor://..."
 *
 * The grammar does not know which registries, URLs, repositories, paths,
 * transports, or package managers exist.
 */
dependencySourceClause
    : SOURCE dependencySourceValue
    ;


/**
 * Source values are strings so the grammar remains independent from any
 * particular package registry, VCS, operating system, filesystem layout,
 * network protocol, or deployment environment.
 */
dependencySourceValue
    : STRING_LITERAL
    ;


/* ============================================================================
 * FEATURES
 * ========================================================================== */

/**
 * Dependency feature selection.
 *
 * Multiple feature clauses are allowed.
 *
 * No artificial feature-count limit exists.
 */
dependencyFeatureClause
    : FEATURE dependencyFeatureList
    ;


/**
 * One or more dependency feature names.
 */
dependencyFeatureList
    : dependencyFeature
    | LBRACKET dependencyFeatureItems? RBRACKET
    ;


/**
 * Feature names are qualified names so namespaces can be represented
 * without introducing a second naming system.
 */
dependencyFeature
    : qualifiedName
    ;


/**
 * Comma-separated feature collection.
 *
 * The list is intentionally unbounded.
 */
dependencyFeatureItems
    : dependencyFeature (COMMA dependencyFeature)*
    ;


/* ============================================================================
 * CAPABILITY REQUIREMENTS
 * ========================================================================== */

/**
 * Dependency capability requirement.
 *
 * This records semantic capability intent.
 *
 * It must not be confused with hardware selection.
 *
 * Valid conceptually:
 *
 *   dependency quantum.runtime capability "quantum.measurement";
 *
 * Invalid architectural interpretation:
 *
 *   dependency quantum.runtime use_qpu_0;
 *
 * Hardware/resource realization belongs to the resource/capability and
 * backend layers, not this grammar.
 */
dependencyCapabilityClause
    : CAPABILITY dependencyCapabilityValue
    ;


dependencyCapabilityValue
    : qualifiedName
    | STRING_LITERAL
    ;


/* ============================================================================
 * GENERIC METADATA
 * ========================================================================== */

/**
 * Dependency metadata.
 *
 * Metadata is intentionally generic. Known metadata keys may later receive
 * semantic validation without requiring a grammar rewrite.
 *
 * This permits future dependency ecosystems without turning every new
 * metadata key into a language keyword.
 */
dependencyMetadataClause
    : dependencyMetadataKey ASSIGN dependencyMetadataValue
    ;


dependencyMetadataKey
    : identifier
    | qualifiedName
    ;


dependencyMetadataValue
    : dependencyMetadataAtom
    | dependencyMetadataList
    | dependencyMetadataMap
    ;


/**
 * Scalar dependency metadata.
 */
dependencyMetadataAtom
    : STRING_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | TRUE
    | FALSE
    | qualifiedName
    ;


/**
 * Arbitrarily sized metadata list.
 */
dependencyMetadataList
    : LBRACKET dependencyMetadataElements? RBRACKET
    ;


dependencyMetadataElements
    : dependencyMetadataValue
      (COMMA dependencyMetadataValue)*
    ;


/**
 * Arbitrarily sized metadata map.
 */
dependencyMetadataMap
    : LBRACE dependencyMetadataEntries? RBRACE
    ;


dependencyMetadataEntries
    : dependencyMetadataEntry
      (COMMA dependencyMetadataEntry)*
    ;


dependencyMetadataEntry
    : dependencyMetadataKey ASSIGN dependencyMetadataValue
    ;


/* ============================================================================
 * DEPENDENCY BODY
 * ========================================================================== */

/**
 * Dependency body.
 *
 * The body contains dependency-local clauses only.
 *
 * Module declarations, package declarations, imports, exports, functions,
 * quantum operations, HDL constructs, etc. do not belong here.
 */
dependencyBody
    : LBRACE dependencyBodyItem* RBRACE
    ;


dependencyBodyItem
    : dependencyBodyClause SEMICOLON?
    ;


dependencyBodyClause
    : dependencyVersionClause
    | dependencyAliasClause
    | dependencyOptionalClause
    | dependencySourceClause
    | dependencyFeatureClause
    | dependencyCapabilityClause
    | dependencyMetadataClause
    ;


/* ============================================================================
 * SHARED NAME CONTRACT
 * ========================================================================== */

/**
 * Dependency-local identifier.
 *
 * The canonical identifier rule is supplied by QualifiedNames.
 *
 * Keeping this rule local as an alias makes the dependency grammar's
 * contract explicit without creating a second identifier implementation.
 */
dependencyIdentifier
    : identifier
    ;