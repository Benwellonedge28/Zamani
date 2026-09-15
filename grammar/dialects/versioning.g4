/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/dialects/versioning.g4
 *
 * Role:
 *     Dialect-specific versioning grammar.
 *
 * Status:
 *     Canonical dialect-version syntax adapter.
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     ANTLR4
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
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
 *     dialects::Versioning              <-- THIS FILE
 *       |
 *       v
 *     dialect grammar
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> dialect registry
 *       +--> compatibility analysis
 *       +--> capability resolution
 *       +--> compilation
 *       +--> runtime negotiation
 *
 * This file is intentionally an ADAPTER around the canonical core versioning
 * model.
 *
 * It MUST NOT define a second version language.
 *
 * ============================================================================
 * CORE OWNERSHIP BOUNDARY
 * ============================================================================
 *
 * grammar/core/versioning.g4 owns:
 *
 *     - version expressions;
 *     - exact versions;
 *     - version components;
 *     - version suffixes;
 *     - pre-release identifiers;
 *     - build metadata;
 *     - version ranges;
 *     - version constraints;
 *     - version references;
 *     - version channels;
 *     - general compatibility version syntax.
 *
 * This file consumes those rules.
 *
 * This file owns only the CONTEXT in which a version expression applies to
 * a dialect.
 *
 * ============================================================================
 * IMPORTANT
 * ============================================================================
 *
 * DO NOT duplicate rules such as:
 *
 *     versionCore
 *     exactVersion
 *     versionRange
 *     versionComparator
 *     versionConstraintExpression
 *     versionIdentifier
 *
 * here.
 *
 * Such duplication would allow the core and dialect grammars to evolve into
 * incompatible version languages.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Dialect versioning describes the version of a source-level semantic
 * contract.
 *
 * It MUST NOT encode:
 *
 *     - CPU model;
 *     - GPU model;
 *     - FPGA model;
 *     - ASIC model;
 *     - QPU model;
 *     - physical qubit count;
 *     - memory size;
 *     - device count;
 *     - topology;
 *     - deployment location;
 *     - scheduler version;
 *     - calibration state;
 *     - runtime resource capacity.
 *
 * Those properties belong to target, capability, resource, compilation,
 * scheduling, deployment, and runtime layers.
 *
 * ============================================================================
 * OPEN-WORLD VERSIONING
 * ============================================================================
 *
 * Dialects are not enumerated here.
 *
 * The following are all structurally valid dialect identities when the
 * surrounding dialect grammar supplies them:
 *
 *     quantum::standard
 *     quantum::future
 *     hardware::fpga
 *     hdl::rtl
 *     ai::tensor
 *     distributed::consensus
 *     vendor::example::quantum
 *     organization::research::extension
 *
 * Adding another dialect MUST NOT require modifying this file.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There is intentionally no finite limit on:
 *
 *     - version component magnitude;
 *     - pre-release identifiers;
 *     - build identifiers;
 *     - version-expression count;
 *     - compatibility requirements;
 *     - dialect declarations;
 *     - dialect inheritance depth;
 *     - dialect version metadata.
 *
 * This grammar does not define:
 *
 *     MAX_VERSION
 *     MAX_VERSION_COMPONENT
 *     MAX_DIALECT_VERSIONS
 *     MAX_COMPATIBILITY_REQUIREMENTS
 *
 * Practical resource limits belong to compiler/runtime policy.
 *
 * ============================================================================
 * VERSION SEMANTICS
 * ============================================================================
 *
 * This grammar does NOT decide whether:
 *
 *     1.2.0 < 2.0.0
 *
 * or whether:
 *
 *     >= 1.0.0 and < 2.0.0
 *
 * is compatible with a particular dialect.
 *
 * The parser only establishes structure.
 *
 * Semantic analysis owns:
 *
 *     - comparison;
 *     - normalization;
 *     - compatibility;
 *     - satisfiability;
 *     - migration;
 *     - deprecation;
 *     - conflict detection.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no actions;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access;
 *     - no runtime calls;
 *     - no randomness.
 *
 * Identical token streams therefore receive identical syntactic treatment.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend must preserve:
 *
 *     - source span;
 *     - version expression structure;
 *     - original spelling where required for diagnostics/round-tripping;
 *     - dialect-version relationship;
 *     - declaration/constraint distinction.
 *
 * The AST should distinguish at least:
 *
 *     DialectVersionDeclaration
 *     DialectVersionConstraint
 *     DialectVersionRequirement
 *     DialectCompatibilityDeclaration
 *
 * The AST MUST NOT resolve the version during parsing.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces no IR.
 *
 * Version information may later become semantic metadata attached to:
 *
 *     - dialect contracts;
 *     - compilation units;
 *     - module/package contracts;
 *     - canonical semantic representations.
 *
 * The grammar MUST NOT create:
 *
 *     Quantum IR
 *     Classical IR
 *     Hardware IR
 *     Runtime state
 *     Resource state
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * The canonical dialect grammar currently expects:
 *
 *     dialectVersionDeclaration
 *
 * and:
 *
 *     dialectVersionConstraint
 *
 * This file provides both rules.
 *
 * Existing simple syntax:
 *
 *     version "1.0.0";
 *
 * remains supported.
 *
 * General version expressions are also supported through core/versioning.g4.
 *
 * Examples:
 *
 *     version 1;
 *     version 1.0;
 *     version 1.0.0;
 *     version "1.0.0";
 *
 *     version >= 1.0.0;
 *
 *     version >= 1.0.0 and < 2.0.0;
 *
 *     version 1.0.0 .. 2.0.0;
 *
 * The exact accepted expression forms are inherited from core/versioning.g4.
 *
 * ============================================================================
 * NO CIRCULAR DEPENDENCY
 * ============================================================================
 *
 * Dependency direction:
 *
 *     ZamaniLexer
 *          |
 *          v
 *     core::Names
 *          |
 *          v
 *     core::Versioning
 *          |
 *          v
 *     dialects::Versioning
 *          |
 *          v
 *     dialects::Dialects
 *
 * This file MUST NOT depend on:
 *
 *     quantum
 *     hardware
 *     runtime
 *     scheduling
 *     routing
 *     optimization
 *     QEC
 *     ZQN
 *     resilience
 *     canonical IR
 *
 * ============================================================================
 */

parser grammar DialectVersioning;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Versioning;


/*
 * ============================================================================
 * 1. DIALECT VERSION DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     version 1.0.0;
 *
 *     version "1.0.0";
 *
 *     version >= 1.0.0;
 *
 * The surrounding Dialects grammar determines that this declaration belongs
 * to a dialect.
 *
 * This rule therefore does NOT consume:
 *
 *     DIALECT
 *
 * because dialect identity belongs to dialects/dialects.g4.
 */
dialectVersionDeclaration
    : VERSION versionExpression SEMICOLON
    ;


/*
 * ============================================================================
 * 2. DIALECT VERSION CONSTRAINT
 * ============================================================================
 *
 * This form is used where the enclosing dialect grammar needs a version
 * constraint without terminating the surrounding declaration.
 *
 * Example:
 *
 *     use quantum::dynamic version >= 1.0.0;
 *
 * The `VERSION` marker belongs to the dialect-version syntax.
 */
dialectVersionConstraint
    : VERSION versionExpression
    ;


/*
 * ============================================================================
 * 3. DIALECT VERSION REFERENCE
 * ============================================================================
 *
 * A named version contract may be referenced rather than embedding a
 * concrete version.
 *
 * Examples:
 *
 *     version stable;
 *
 *     version compatibility::stable;
 *
 * Resolution is semantic.
 */
dialectVersionReference
    : versionReference
    ;


/*
 * ============================================================================
 * 4. DIALECT VERSION EXPRESSION
 * ============================================================================
 *
 * Named wrapper for consumers that need to preserve the fact that an
 * expression is attached specifically to a dialect.
 *
 * The underlying syntax remains owned by core/versioning.g4.
 */
dialectVersionExpression
    : versionExpression
    ;


/*
 * ============================================================================
 * 5. DIALECT EXACT VERSION
 * ============================================================================
 *
 * Wrapper around the canonical core version representation.
 *
 * This rule exists as an explicit integration boundary rather than redefining
 * exactVersion.
 */
dialectExactVersion
    : exactVersion
    ;


/*
 * ============================================================================
 * 6. DIALECT VERSION RANGE
 * ============================================================================
 *
 * Wrapper around the canonical range representation.
 */
dialectVersionRange
    : versionRange
    ;


/*
 * ============================================================================
 * 7. DIALECT VERSION CONSTRAINT SET
 * ============================================================================
 *
 * Wrapper around the canonical constraint-set representation.
 */
dialectVersionConstraintSet
    : versionConstraintSet
    ;


/*
 * ============================================================================
 * 8. DIALECT VERSION CHANNEL
 * ============================================================================
 *
 * Examples:
 *
 *     stable
 *     beta
 *     alpha
 *     rc
 *     dev
 *
 * Additional channels remain possible because the underlying core grammar
 * permits identifier-based version channels.
 */
dialectVersionChannel
    : versionChannel
    ;


/*
 * ============================================================================
 * 9. DIALECT VERSION METADATA
 * ============================================================================
 *
 * A dialect may expose a version contract as metadata.
 *
 * Example:
 *
 *     version_metadata stable;
 *
 * The metadata keyword itself is intentionally NOT introduced here.
 *
 * Instead, this rule is a structural wrapper around an existing version
 * expression and is intended for higher-level grammar composition.
 *
 * Semantic interpretation remains outside the grammar.
 */
dialectVersionMetadata
    : dialectVersionExpression
    ;


/*
 * ============================================================================
 * 10. DIALECT VERSION REQUIREMENT
 * ============================================================================
 *
 * A dialect may require another version contract.
 *
 * Example:
 *
 *     requires_version 1.2.0;
 *
 * This rule intentionally uses the existing VERSION marker and does not
 * introduce another version language.
 *
 * The actual `requires` declaration remains owned by the dialect grammar.
 */
dialectVersionRequirement
    : VERSION versionExpression SEMICOLON
    ;


/*
 * ============================================================================
 * 11. DIALECT VERSION CAPABILITY
 * ============================================================================
 *
 * Represents a version expression that a dialect declares it understands.
 *
 * Example:
 *
 *     supports_version >= 1.0.0;
 *
 * The surrounding dialect grammar owns the `supports` declaration itself.
 */
dialectVersionCapability
    : versionExpression
    ;


/*
 * ============================================================================
 * 12. DIALECT VERSION COMPATIBILITY
 * ============================================================================
 *
 * A dialect may expose a compatibility relationship with another version.
 *
 * The semantic layer determines whether that relationship is actually valid.
 *
 * No compatibility algorithm belongs here.
 */
dialectVersionCompatibility
    : versionExpression
    ;


/*
 * ============================================================================
 * 13. DIALECT VERSION TARGET
 * ============================================================================
 *
 * A target-version expression remains a version contract only.
 *
 * It does NOT identify a physical target.
 *
 * This wrapper exists for higher-level target compatibility grammars that
 * need to distinguish:
 *
 *     dialect version
 *
 * from:
 *
 *     target resource description.
 */
dialectTargetVersion
    : versionExpression
    ;


/*
 * ============================================================================
 * 14. DIALECT VERSION LIST
 * ============================================================================
 *
 * An unbounded list of dialect version expressions.
 *
 * No fixed number of versions is encoded.
 */
dialectVersionList
    : dialectVersionExpression
      (COMMA dialectVersionExpression)*
    ;


/*
 * ============================================================================
 * 15. OPTIONAL DIALECT VERSION LIST
 * ============================================================================
 */
optionalDialectVersionList
    : dialectVersionList?
    ;


/*
 * ============================================================================
 * 16. DIALECT VERSION REFERENCE LIST
 * ============================================================================
 */
dialectVersionReferenceList
    : dialectVersionReference
      (COMMA dialectVersionReference)*
    ;


/*
 * ============================================================================
 * 17. OPTIONAL DIALECT VERSION REFERENCE LIST
 * ============================================================================
 */
optionalDialectVersionReferenceList
    : dialectVersionReferenceList?
    ;


/*
 * ============================================================================
 * 18. DIALECT VERSION REQUIREMENT LIST
 * ============================================================================
 *
 * The surrounding declaration supplies the actual requirement keyword.
 *
 * No fixed number of requirements exists.
 */
dialectVersionRequirementList
    : dialectVersionExpression
      (COMMA dialectVersionExpression)*
    ;


/*
 * ============================================================================
 * 19. DIALECT VERSION CONSTRAINT LIST
 * ============================================================================
 */
dialectVersionConstraintList
    : dialectVersionExpression
      (COMMA dialectVersionExpression)*
    ;


/*
 * ============================================================================
 * 20. OPTIONAL DIALECT VERSION CONSTRAINT LIST
 * ============================================================================
 */
optionalDialectVersionConstraintList
    : dialectVersionConstraintList?
    ;


/*
 * ============================================================================
 * 21. DIALECT VERSION RANGE LIST
 * ============================================================================
 */
dialectVersionRangeList
    : dialectVersionRange
      (COMMA dialectVersionRange)*
    ;


/*
 * ============================================================================
 * 22. OPTIONAL DIALECT VERSION RANGE LIST
 * ============================================================================
 */
optionalDialectVersionRangeList
    : dialectVersionRangeList?
    ;


/*
 * ============================================================================
 * 23. DIALECT VERSION CONTRACT
 * ============================================================================
 *
 * A generic wrapper used by integration grammars that need to accept any
 * canonical version expression without duplicating the version grammar.
 */
dialectVersionContract
    : versionExpression
    ;


/*
 * ============================================================================
 * 24. DIALECT VERSION CONTRACT LIST
 * ============================================================================
 */
dialectVersionContractList
    : dialectVersionContract
      (COMMA dialectVersionContract)*
    ;


/*
 * ============================================================================
 * 25. OPTIONAL DIALECT VERSION CONTRACT LIST
 * ============================================================================
 */
optionalDialectVersionContractList
    : dialectVersionContractList?
    ;