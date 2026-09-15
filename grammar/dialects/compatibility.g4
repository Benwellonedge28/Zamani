/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/dialects/compatibility.g4
 *
 * Role:
 *     Canonical dialect compatibility-contract grammar.
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
 *     core::Names
 *       |
 *       v
 *     core::Versioning
 *       |
 *       v
 *     dialects::Compatibility        <-- THIS FILE
 *       |
 *       v
 *     dialect AST
 *       |
 *       v
 *     semantic compatibility analysis
 *       |
 *       +--> dialect registry
 *       +--> capability resolution
 *       +--> package/module compatibility
 *       +--> compiler compatibility
 *       +--> runtime compatibility
 *       +--> target-contract compatibility
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware semantic models
 *       +--> control/data/temporal IR
 *       |
 *       v
 *     optimization / routing / scheduling / QEC / ZQN /
 *     resilience / hardware HAL / runtime
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines SOURCE-LEVEL COMPATIBILITY CONTRACT SYNTAX for
 * dialects.
 *
 * It allows a dialect to express relationships such as:
 *
 *     compatible with another dialect/version
 *     requires another dialect/version
 *     supports another dialect/version
 *     conflicts with another dialect/version
 *     supersedes another dialect/version
 *     replaces another dialect/version
 *     deprecates another dialect/version
 *     provides a compatibility profile
 *     declares migration relationships
 *     declares compatibility policies
 *
 * Compatibility is a semantic contract.
 *
 * This grammar does NOT decide whether a contract is satisfied.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - dialect compatibility declaration syntax;
 *   - compatibility relationship syntax;
 *   - compatibility subject syntax;
 *   - compatibility profile syntax;
 *   - compatibility requirement syntax;
 *   - compatibility conflict syntax;
 *   - compatibility migration syntax;
 *   - compatibility deprecation syntax;
 *   - compatibility replacement syntax;
 *   - compatibility policy metadata syntax;
 *   - structural compatibility predicates;
 *   - compatibility annotations;
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical identifiers;
 *   - keywords;
 *   - version-number syntax;
 *   - semantic version comparison;
 *   - version satisfiability;
 *   - package resolution;
 *   - module resolution;
 *   - filesystem access;
 *   - network access;
 *   - plugin discovery;
 *   - hardware discovery;
 *   - hardware topology;
 *   - resource allocation;
 *   - scheduling;
 *   - routing;
 *   - optimization;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - simulation;
 *   - runtime execution;
 *   - canonical IR;
 *   - quantum::ir;
 *   - vendor implementation;
 *   - compiler implementation;
 *
 * ============================================================================
 * CRITICAL BOUNDARY
 * ============================================================================
 *
 * Compatibility MUST remain independent of physical realization.
 *
 * A compatibility declaration MUST NOT mean:
 *
 *     use CPU X
 *     use GPU X
 *     use FPGA X
 *     use ASIC X
 *     use QPU X
 *     use device X
 *     use topology X
 *     use N qubits
 *     use N nodes
 *     use N cores
 *     use N GPUs
 *     use a particular calibration
 *     use a particular scheduler
 *
 * Such information belongs to:
 *
 *     target descriptions
 *     capability models
 *     resource models
 *     deployment configuration
 *     runtime discovery
 *     scheduling
 *     hardware abstraction
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Compatibility is part of the durable SOURCE CONTRACT.
 *
 * It therefore describes:
 *
 *     what language/dialect contract is expected
 *
 * rather than:
 *
 *     what machine happens to execute it.
 *
 * This preserves:
 *
 *     Program Once
 *     Compile Once
 *     Run Everywhere
 *     Run Anywhere
 *     Run Forever
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO grammar-level limits on:
 *
 *   - number of compatibility declarations;
 *   - number of subjects;
 *   - number of requirements;
 *   - number of profiles;
 *   - number of migration entries;
 *   - number of deprecated contracts;
 *   - version magnitude;
 *   - version-expression complexity;
 *   - dialect nesting;
 *   - compatibility relationships.
 *
 * The repetition operators in this grammar are intentionally open-ended.
 *
 * No:
 *
 *     MAX_DIALECTS
 *     MAX_COMPATIBILITY_RULES
 *     MAX_VERSIONS
 *     MAX_REQUIREMENTS
 *     MAX_FEATURES
 *     MAX_TARGETS
 *     MAX_QUBITS
 *     MAX_DEVICES
 *
 * exists here.
 *
 * Practical limits are compiler/resource-policy concerns.
 *
 * ============================================================================
 * VERSION OWNERSHIP
 * ============================================================================
 *
 * Version syntax is owned by:
 *
 *     grammar/core/versioning.g4
 *
 * This file MUST consume:
 *
 *     versionExpression
 *     exactVersion
 *     versionRange
 *     versionConstraintSet
 *     versionReference
 *     versionChannel
 *
 * from that canonical grammar.
 *
 * This file MUST NOT redefine them.
 *
 * ============================================================================
 * DIALECT OWNERSHIP
 * ============================================================================
 *
 * Dialect declaration syntax remains owned by:
 *
 *     grammar/dialects/dialects.g4
 *
 * This file only provides compatibility members that can be incorporated
 * by the dialect grammar.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve:
 *
 *   - source span;
 *   - relationship kind;
 *   - subject identity;
 *   - optional version expression;
 *   - optional profile;
 *   - optional predicate;
 *   - optional migration target;
 *   - optional reason/metadata;
 *   - original source spelling where diagnostics/round-tripping require it.
 *
 * Parsing MUST NOT:
 *
 *   - resolve a dialect;
 *   - compare versions;
 *   - query a registry;
 *   - inspect hardware;
 *   - query capabilities;
 *   - select a backend.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *   - resolving compatibility subjects;
 *   - resolving dialect identities;
 *   - evaluating version expressions;
 *   - detecting unsatisfied requirements;
 *   - detecting incompatible contracts;
 *   - detecting circular compatibility/migration relationships;
 *   - detecting impossible combinations;
 *   - applying compatibility policy;
 *   - applying deprecation policy;
 *   - applying migration policy;
 *   - producing diagnostics;
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces NO IR.
 *
 * Compatibility information may later become semantic metadata attached to:
 *
 *   - dialect contracts;
 *   - module contracts;
 *   - package contracts;
 *   - compilation-unit contracts;
 *   - canonical semantic representations.
 *
 * It MUST NOT create:
 *
 *   - Quantum IR;
 *   - Classical IR;
 *   - Hardware IR;
 *   - Runtime state;
 *   - Resource state.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *   - no embedded Rust;
 *   - no actions;
 *   - no semantic predicates;
 *   - no filesystem access;
 *   - no network access;
 *   - no hardware access;
 *   - no runtime calls;
 *   - no randomness.
 *
 * Identical token streams therefore receive identical syntactic treatment.
 *
 * ============================================================================
 * CONTEXTUAL KEYWORD DESIGN
 * ============================================================================
 *
 * The current canonical lexer does not make every future compatibility word
 * a reserved token.
 *
 * Therefore compatibility relation names are represented structurally through
 * identifiers where appropriate.
 *
 * This is deliberate.
 *
 * It prevents this grammar from forcing every future compatibility concept
 * into the global lexical keyword namespace.
 *
 * Semantic analysis MUST validate the recognized contextual names.
 *
 * This permits future contracts such as:
 *
 *     compatible
 *     requires
 *     supports
 *     conflicts
 *     supersedes
 *     replaces
 *     deprecated
 *     migrates
 *     equivalent
 *     conditional
 *
 * without requiring a new lexer keyword merely to extend compatibility
 * semantics.
 *
 * ============================================================================
 */

parser grammar DialectCompatibility;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Versioning;


/* ============================================================================
 * 1. TOP-LEVEL COMPATIBILITY MEMBER
 * ========================================================================== */

/*
 * This is the integration entry point for dialects.g4.
 *
 * Recommended integration:
 *
 *     dialectMember
 *         : ...
 *         | dialectCompatibilityMember
 *         ;
 *
 * The dialect grammar remains responsible for determining that the member
 * occurs inside a dialect declaration.
 */
dialectCompatibilityMember
    : dialectCompatibilityDeclaration
    | dialectCompatibilityProfileDeclaration
    | dialectMigrationDeclaration
    | dialectDeprecationDeclaration
    | dialectReplacementDeclaration
    | dialectSupersessionDeclaration
    ;


/* ============================================================================
 * 2. GENERIC COMPATIBILITY DECLARATION
 * ========================================================================== */

/*
 * Generic contextual form:
 *
 *     compatible <subject> [versionExpression] ;
 *
 *     requires <subject> [versionExpression] ;
 *
 *     supports <subject> [versionExpression] ;
 *
 *     conflicts <subject> [versionExpression] ;
 *
 *     equivalent <subject> [versionExpression] ;
 *
 * Relation names are contextual identifiers rather than a closed lexer
 * enumeration.
 */
dialectCompatibilityDeclaration
    : compatibilityRelation compatibilitySubjectList
      compatibilityVersionClause?
      compatibilityConditionClause*
      compatibilityMetadataClause*
      SEMICOLON
    ;


/* ============================================================================
 * 3. RELATIONSHIP KIND
 * ========================================================================== */

/*
 * The semantic layer recognizes the canonical contextual relation names.
 *
 * Canonical names:
 *
 *     compatible
 *     requires
 *     supports
 *     conflicts
 *     equivalent
 *
 * Additional implementation-neutral relationship names may be introduced
 * through dialect policy, but they MUST NOT change the grammar's physical
 * independence.
 */
compatibilityRelation
    : identifier
    ;


/* ============================================================================
 * 4. SUBJECT LIST
 * ========================================================================== */

dialectCompatibilitySubjectList
    : compatibilitySubjectList
    ;

compatibilitySubjectList
    : compatibilitySubject
      (COMMA compatibilitySubject)*
    ;


/* ============================================================================
 * 5. COMPATIBILITY SUBJECT
 * ========================================================================== */

/*
 * A subject identifies a semantic contract.
 *
 * Examples:
 *
 *     quantum::standard
 *     hardware::abstract
 *     hdl::rtl
 *     classical::core
 *     ai::tensor
 *     distributed::execution
 *
 * A subject is NOT a device identifier.
 *
 * The same qualified-name mechanism can therefore describe future domains
 * without changing this grammar.
 */
compatibilitySubject
    : qualifiedName
    ;


/* ============================================================================
 * 6. VERSION CLAUSE
 * ========================================================================== */

/*
 * Version semantics come entirely from core/versioning.g4.
 *
 * The compatibility grammar merely associates a canonical version expression
 * with a compatibility subject.
 */
compatibilityVersionClause
    : compatibilityVersionMarker versionExpression
    ;


/*
 * Contextual marker.
 *
 * Canonical spelling:
 *
 *     version
 *
 * It remains an identifier at this grammar layer so the compatibility grammar
 * does not require a new globally reserved lexer token.
 */
compatibilityVersionMarker
    : identifier
    ;


/* ============================================================================
 * 7. EXPLICIT VERSION REQUIREMENT
 * ========================================================================== */

/*
 * Supports:
 *
 *     requires quantum::standard version >= 1.0.0;
 *
 * The version itself is parsed by Versioning.
 */
dialectCompatibilityRequirement
    : compatibilityRequirementMarker
      compatibilitySubject
      compatibilityVersionClause?
      compatibilityConditionClause*
      SEMICOLON
    ;

compatibilityRequirementMarker
    : identifier
    ;


/* ============================================================================
 * 8. CONDITIONS
 * ========================================================================== */

/*
 * Conditions are structural.
 *
 * They do not execute code.
 *
 * They do not inspect hardware.
 *
 * They do not query runtime state.
 */
compatibilityConditionClause
    : compatibilityConditionMarker compatibilityPredicate
    ;

compatibilityConditionMarker
    : identifier
    ;

compatibilityPredicate
    : compatibilityPredicateOr
    ;

compatibilityPredicateOr
    : compatibilityPredicateAnd
      (
          OR
          compatibilityPredicateAnd
      )*
    ;

compatibilityPredicateAnd
    : compatibilityPredicateUnary
      (
          AND
          compatibilityPredicateUnary
      )*
    ;

compatibilityPredicateUnary
    : NOT compatibilityPredicatePrimary
    | compatibilityPredicatePrimary
    ;

compatibilityPredicatePrimary
    : compatibilityPredicateAtom
    | LPAREN compatibilityPredicateOr RPAREN
    ;

compatibilityPredicateAtom
    : qualifiedName
    | qualifiedName compatibilityComparisonOperator compatibilityValue
    ;


/* ============================================================================
 * 9. COMPARISON OPERATORS
 * ========================================================================== */

/*
 * These operators compare semantic compatibility metadata.
 *
 * They do not compare physical resources.
 */
compatibilityComparisonOperator
    : EQ
    | NE
    | LT
    | LE
    | GT
    | GE
    ;


/* ============================================================================
 * 10. COMPATIBILITY VALUES
 * ========================================================================== */

compatibilityValue
    : STRING
    | INTEGER
    | FLOAT
    | TRUE
    | FALSE
    | NIL
    | NULL
    | identifier
    | qualifiedName
    | compatibilityListValue
    | compatibilityMapValue
    ;

compatibilityListValue
    : LBRACKET
      compatibilityValueList?
      RBRACKET
    ;

compatibilityValueList
    : compatibilityValue
      (COMMA compatibilityValue)*
      COMMA?
    ;

compatibilityMapValue
    : LBRACE
      compatibilityMapEntry*
      RBRACE
    ;

compatibilityMapEntry
    : identifier
      COLON
      compatibilityValue
      COMMA?
    ;


/* ============================================================================
 * 11. METADATA
 * ========================================================================== */

/*
 * Metadata is open-ended.
 *
 * Unknown metadata is retained syntactically and validated semantically.
 *
 * This prevents future compatibility information from requiring grammar
 * rewrites.
 */
compatibilityMetadataClause
    : compatibilityMetadataMarker
      compatibilityMetadataValue?
    ;

compatibilityMetadataMarker
    : identifier
    ;

compatibilityMetadataValue
    : ASSIGN compatibilityValue
    | LPAREN compatibilityValueList? RPAREN
    ;


/* ============================================================================
 * 12. COMPATIBILITY PROFILES
 * ========================================================================== */

/*
 * A profile groups compatibility contracts under one semantic name.
 *
 * Example:
 *
 *     compatibility profile "portable" {
 *         ...
 *     }
 *
 * The profile itself does not perform compatibility checking.
 */
dialectCompatibilityProfileDeclaration
    : compatibilityProfileMarker
      compatibilityProfileName
      LBRACE
      dialectCompatibilityProfileMember*
      RBRACE
    ;

compatibilityProfileMarker
    : identifier
    ;

compatibilityProfileName
    : identifier
    | STRING
    ;

dialectCompatibilityProfileMember
    : dialectCompatibilityDeclaration
    | dialectCompatibilityRequirement
    | dialectMigrationDeclaration
    | dialectDeprecationDeclaration
    | dialectReplacementDeclaration
    | dialectSupersessionDeclaration
    | compatibilityMetadataClause
    ;


/* ============================================================================
 * 13. MIGRATION DECLARATIONS
 * ========================================================================== */

/*
 * Migration expresses a semantic evolution path.
 *
 * Example:
 *
 *     migrate quantum::old to quantum::new;
 *
 * Migration is not automatic source rewriting.
 *
 * The compiler/tooling layer determines whether and how a migration can be
 * performed.
 */
dialectMigrationDeclaration
    : migrationMarker
      compatibilitySubject
      migrationTargetClause
      migrationVersionClause?
      compatibilityConditionClause*
      compatibilityMetadataClause*
      SEMICOLON
    ;

migrationMarker
    : identifier
    ;

migrationTargetClause
    : migrationTargetMarker compatibilitySubject
    ;

migrationTargetMarker
    : identifier
    ;

migrationVersionClause
    : compatibilityVersionClause
    ;


/* ============================================================================
 * 14. DEPRECATION
 * ========================================================================== */

/*
 * Deprecation is source-contract metadata.
 *
 * It does not automatically reject a program.
 *
 * Semantic analysis decides whether the active compatibility policy treats
 * deprecation as:
 *
 *     warning
 *     error
 *     informational
 *     migration-required
 */
dialectDeprecationDeclaration
    : deprecationMarker
      compatibilitySubject
      deprecationMetadataClause*
      SEMICOLON
    ;

deprecationMarker
    : identifier
    ;

deprecationMetadataClause
    : compatibilityMetadataClause
    | compatibilityVersionClause
    ;


/* ============================================================================
 * 15. REPLACEMENT
 * ========================================================================== */

/*
 * Example:
 *
 *     replace quantum::legacy with quantum::standard;
 *
 * The grammar records the relationship.
 *
 * Semantic analysis determines:
 *
 *     - whether replacement is valid;
 *     - whether source migration is possible;
 *     - whether semantics are preserved.
 */
dialectReplacementDeclaration
    : replacementMarker
      compatibilitySubject
      replacementTargetClause
      compatibilityConditionClause*
      compatibilityMetadataClause*
      SEMICOLON
    ;

replacementMarker
    : identifier
    ;

replacementTargetClause
    : replacementTargetMarker compatibilitySubject
    ;

replacementTargetMarker
    : identifier
    ;


/* ============================================================================
 * 16. SUPERSESSION
 * ========================================================================== */

/*
 * Supersession differs from replacement:
 *
 *     superseded contract
 *
 * may remain semantically meaningful while another contract becomes the
 * preferred successor.
 */
dialectSupersessionDeclaration
    : supersessionMarker
      compatibilitySubject
      supersessionTargetClause
      compatibilityConditionClause*
      compatibilityMetadataClause*
      SEMICOLON
    ;

supersessionMarker
    : identifier
    ;

supersessionTargetClause
    : supersessionTargetMarker compatibilitySubject
    ;

supersessionTargetMarker
    : identifier
    ;


/* ============================================================================
 * 17. COMPATIBILITY MATRIX ENTRY
 * ========================================================================== */

/*
 * A compatibility matrix entry allows many-to-many relationships without
 * requiring a fixed number of subjects.
 *
 * Example:
 *
 *     compatibility {
 *         quantum::standard -> quantum::future >= 2.0;
 *     }
 *
 * The arrow represents a semantic relationship, not execution flow.
 */
dialectCompatibilityMatrixEntry
    : compatibilitySubject
      compatibilityMatrixOperator
      compatibilitySubject
      compatibilityVersionClause?
      compatibilityConditionClause*
      compatibilityMetadataClause*
      SEMICOLON
    ;

compatibilityMatrixOperator
    : ARROW
    ;


/* ============================================================================
 * 18. COMPATIBILITY MATRIX
 * ========================================================================== */

/*
 * The matrix itself is deliberately open-ended.
 */
dialectCompatibilityMatrixDeclaration
    : compatibilityMatrixMarker
      LBRACE
      dialectCompatibilityMatrixEntry*
      RBRACE
    ;

compatibilityMatrixMarker
    : identifier
    ;


/* ============================================================================
 * 19. COMPATIBILITY SET
 * ========================================================================== */

/*
 * A compatibility set groups subjects that are intended to participate in
 * one semantic compatibility contract.
 *
 * Example:
 *
 *     compatibility_set universal_quantum {
 *         quantum::standard,
 *         quantum::future,
 *         quantum::openqasm
 *     }
 *
 * The set does not imply that every member is mutually compatible.
 *
 * Actual relationships remain explicit.
 */
dialectCompatibilitySetDeclaration
    : compatibilitySetMarker
      identifier
      LBRACE
      compatibilitySubjectList?
      RBRACE
    ;

compatibilitySetMarker
    : identifier
    ;


/* ============================================================================
 * 20. COMPATIBILITY ASSERTION
 * ========================================================================== */

/*
 * Assertions are declarative statements that the semantic compatibility
 * analyzer may validate.
 *
 * Example:
 *
 *     assert_compatible quantum::standard version >= 1.0;
 *
 * The assertion does not execute.
 */
dialectCompatibilityAssertion
    : compatibilityAssertionMarker
      compatibilitySubject
      compatibilityVersionClause?
      compatibilityConditionClause*
      SEMICOLON
    ;

compatibilityAssertionMarker
    : identifier
    ;


/* ============================================================================
 * 21. COMPATIBILITY NEGATION
 * ========================================================================== */

/*
 * Explicit incompatibility can be represented without introducing a second
 * conflict grammar.
 */
dialectCompatibilityConflict
    : conflictMarker
      compatibilitySubjectList
      compatibilityVersionClause?
      compatibilityConditionClause*
      compatibilityMetadataClause*
      SEMICOLON
    ;

conflictMarker
    : identifier
    ;


/* ============================================================================
 * 22. COMPATIBILITY EQUIVALENCE
 * ========================================================================== */

/*
 * Equivalence is a semantic claim that must later be proven/validated.
 *
 * The grammar merely records the claim.
 */
dialectCompatibilityEquivalence
    : equivalenceMarker
      compatibilitySubjectList
      compatibilityVersionClause?
      compatibilityConditionClause*
      compatibilityMetadataClause*
      SEMICOLON
    ;

equivalenceMarker
    : identifier
    ;


/* ============================================================================
 * 23. COMPATIBILITY RANGE
 * ========================================================================== */

/*
 * Uses the canonical version range model.
 */
dialectCompatibilityRange
    : compatibilitySubject
      compatibilityRangeMarker
      versionRange
      SEMICOLON
    ;

compatibilityRangeMarker
    : identifier
    ;


/* ============================================================================
 * 24. COMPATIBILITY CONSTRAINT SET
 * ========================================================================== */

/*
 * Uses the canonical Versioning constraint-set representation.
 */
dialectCompatibilityConstraintSet
    : compatibilitySubject
      compatibilityConstraintMarker
      versionConstraintSet
      SEMICOLON
    ;

compatibilityConstraintMarker
    : identifier
    ;


/* ============================================================================
 * 25. COMPATIBILITY VERSION REFERENCE
 * ========================================================================== */

/*
 * Named version contracts remain resolved by semantic analysis.
 */
dialectCompatibilityVersionReference
    : compatibilitySubject
      compatibilityVersionReferenceMarker
      versionReference
      SEMICOLON
    ;

compatibilityVersionReferenceMarker
    : identifier
    ;


/* ============================================================================
 * 26. OPTIONAL COMPATIBILITY VERSION
 * ========================================================================== */

optionalCompatibilityVersionClause
    : compatibilityVersionClause?
    ;


/* ============================================================================
 * 27. COMPATIBILITY SUBJECT WITH VERSION
 * ========================================================================== */

/*
 * Reusable integration production for other dialect grammar components.
 */
compatibilitySubjectWithVersion
    : compatibilitySubject
      compatibilityVersionClause?
    ;


/* ============================================================================
 * 28. COMPATIBILITY SUBJECT LIST WITH VERSIONS
 * ========================================================================== */

compatibilitySubjectWithVersionList
    : compatibilitySubjectWithVersion
      (COMMA compatibilitySubjectWithVersion)*
    ;


/* ============================================================================
 * 29. COMPATIBILITY CONTRACT
 * ========================================================================== */

/*
 * Generic structural contract used by higher-level grammar components.
 */
dialectCompatibilityContract
    : compatibilityRelation
      compatibilitySubjectWithVersionList
      compatibilityConditionClause*
      compatibilityMetadataClause*
    ;


/* ============================================================================
 * 30. COMPATIBILITY CONTRACT LIST
 * ========================================================================== */

dialectCompatibilityContractList
    : dialectCompatibilityContract
      (COMMA dialectCompatibilityContract)*
    ;


/* ============================================================================
 * 31. OPTIONAL COMPATIBILITY CONTRACT LIST
 * ========================================================================== */

optionalDialectCompatibilityContractList
    : dialectCompatibilityContractList?
    ;


/* ============================================================================
 * 32. COMPATIBILITY PROFILE BODY
 * ========================================================================== */

/*
 * Explicit reusable body production.
 */
dialectCompatibilityProfileBody
    : LBRACE
      dialectCompatibilityProfileMember*
      RBRACE
    ;


/* ============================================================================
 * 33. COMPATIBILITY POLICY
 * ========================================================================== */

/*
 * A policy names the semantic policy under which compatibility relationships
 * are interpreted.
 *
 * Examples of policy names:
 *
 *     strict
 *     permissive
 *     migration
 *     compatibility
 *     legacy
 *
 * These remain identifiers.
 */
dialectCompatibilityPolicyDeclaration
    : compatibilityPolicyMarker
      identifier
      compatibilityPolicyBody?
      SEMICOLON?
    ;

compatibilityPolicyMarker
    : identifier
    ;

compatibilityPolicyBody
    : LBRACE
      compatibilityPolicyMember*
      RBRACE
    ;

compatibilityPolicyMember
    : compatibilityMetadataClause
    | compatibilityConditionClause
    | dialectCompatibilityContract
    ;


/* ============================================================================
 * 34. COMPATIBILITY POLICY REFERENCE
 * ========================================================================== */

dialectCompatibilityPolicyReference
    : compatibilityPolicyReferenceMarker
      qualifiedName
      SEMICOLON
    ;

compatibilityPolicyReferenceMarker
    : identifier
    ;


/* ============================================================================
 * 35. COMPATIBILITY PROVENANCE
 * ========================================================================== */

/*
 * Provenance is structural metadata only.
 *
 * It does not grant trust.
 *
 * It does not constitute a cryptographic proof.
 */
dialectCompatibilityProvenance
    : compatibilityProvenanceMarker
      compatibilityValue
      SEMICOLON
    ;

compatibilityProvenanceMarker
    : identifier
    ;


/* ============================================================================
 * 36. COMPATIBILITY DIAGNOSTIC METADATA
 * ========================================================================== */

/*
 * Human-readable explanation can be retained without making diagnostics part
 * of semantic execution.
 */
dialectCompatibilityReason
    : compatibilityReasonMarker
      STRING
      SEMICOLON
    ;

compatibilityReasonMarker
    : identifier
    ;


/* ============================================================================
 * 37. COMPATIBILITY DEPRECATION WINDOW
 * ========================================================================== */

/*
 * A deprecation window can reference canonical versions.
 *
 * It does not define policy.
 */
dialectCompatibilityDeprecationWindow
    : compatibilityDeprecationWindowMarker
      versionExpression
      compatibilityRangeSeparator
      versionExpression
      SEMICOLON
    ;

compatibilityDeprecationWindowMarker
    : identifier
    ;

compatibilityRangeSeparator
    : ARROW
    ;


/* ============================================================================
 * 38. COMPATIBILITY MIGRATION WINDOW
 * ========================================================================== */

dialectCompatibilityMigrationWindow
    : compatibilityMigrationWindowMarker
      versionExpression
      compatibilityRangeSeparator
      versionExpression
      SEMICOLON
    ;

compatibilityMigrationWindowMarker
    : identifier
    ;


/* ============================================================================
 * 39. COMPATIBILITY PROFILE REFERENCE
 * ========================================================================== */

dialectCompatibilityProfileReference
    : compatibilityProfileReferenceMarker
      compatibilityProfileName
      SEMICOLON
    ;

compatibilityProfileReferenceMarker
    : identifier
    ;


/* ============================================================================
 * 40. COMPATIBILITY ANNOTATION
 * ========================================================================== */

/*
 * Compatibility-specific annotations remain structural.
 */
dialectCompatibilityAnnotation
    : AT identifier
      compatibilityAnnotationArguments?
    ;

compatibilityAnnotationArguments
    : LPAREN compatibilityValueList? RPAREN
    ;


/* ============================================================================
 * 41. COMPATIBILITY MEMBER WITH ANNOTATIONS
 * ========================================================================== */

annotatedDialectCompatibilityMember
    : dialectCompatibilityAnnotation*
      dialectCompatibilityMember
    ;


/* ============================================================================
 * 42. COMPATIBILITY DOCUMENT
 * ========================================================================== */

/*
 * Reusable complete compatibility document.
 *
 * The enclosing dialect grammar normally supplies the surrounding dialect
 * declaration. This rule is also useful for isolated grammar tests.
 */
dialectCompatibilityDocument
    : annotatedDialectCompatibilityMember*
      EOF
    ;


/* ============================================================================
 * 43. INTEGRATION ALIASES
 * ========================================================================== */

/*
 * Explicitly named adapter rules allow the surrounding dialect grammar to
 * integrate this grammar without depending on implementation details.
 */

dialectCompatibility
    : dialectCompatibilityMember
    ;

dialectCompatibilityDeclarationMember
    : dialectCompatibilityDeclaration
    ;

dialectCompatibilityProfile
    : dialectCompatibilityProfileDeclaration
    ;

dialectCompatibilityMigration
    : dialectMigrationDeclaration
    ;

dialectCompatibilityDeprecation
    : dialectDeprecationDeclaration
    ;

dialectCompatibilityReplacement
    : dialectReplacementDeclaration
    ;

dialectCompatibilitySupersession
    : dialectSupersessionDeclaration
    ;


/*
 * ============================================================================
 * END OF FILE
 * ============================================================================
 *
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * 1. grammar/antlr/ZamaniLexer.g4
 *
 *    No new lexer keyword is required by this file.
 *
 *    This is intentional: compatibility terminology is contextual rather
 *    than globally reserved.
 *
 *
 * 2. grammar/core/Names.g4
 *
 *    Supplies:
 *
 *        identifier
 *        qualifiedName
 *
 *    This file does not redefine either.
 *
 *
 * 3. grammar/core/versioning.g4
 *
 *    Supplies:
 *
 *        versionExpression
 *        exactVersion
 *        versionRange
 *        versionConstraintSet
 *        versionReference
 *        versionChannel
 *
 *    This file MUST NOT duplicate those productions.
 *
 *
 * 4. grammar/dialects/dialects.g4
 *
 *    Integrate:
 *
 *        import Compatibility;
 *
 *    and extend:
 *
 *        dialectMember
 *            : ...
 *            | dialectCompatibilityMember
 *            ;
 *
 *    The dialect grammar remains the owner of dialect declaration structure.
 *
 *
 * 5. grammar/dialects/versioning.g4
 *
 *    Remains responsible for dialect-specific adaptation of the canonical
 *    version model.
 *
 *    It should NOT import this file merely to parse versions.
 *
 *    This file consumes canonical Versioning rules directly.
 *
 *
 * 6. grammar/dialects/registration.g4
 *
 *    May consume:
 *
 *        dialectCompatibilityContract
 *        dialectCompatibilityProfileReference
 *
 *    Registration remains responsible for registration semantics, not
 *    compatibility evaluation.
 *
 *
 * 7. grammar/dialects/capabilities.g4
 *
 *    Compatibility requirements may refer to capability contracts, but this
 *    grammar does NOT resolve those capabilities.
 *
 *    Capability resolution belongs to semantic analysis.
 *
 *
 * 8. grammar/dialects/vendor.g4
 *
 *    Vendor dialects may use the same compatibility contract syntax.
 *
 *    This grammar deliberately does not contain vendor names.
 *
 *
 * 9. grammar/dialects/namespaces.g4
 *
 *    Qualified dialect names are resolved through the canonical namespace
 *    model.
 *
 *    This file does not define namespace semantics.
 *
 *
 * 10. Quantum grammar
 *
 *     Quantum dialect compatibility may eventually feed semantic analysis
 *     before lowering into quantum::ir.
 *
 *     This grammar MUST NOT create quantum::ir structures.
 *
 *
 * 11. Hardware grammar
 *
 *     Hardware dialect compatibility can describe semantic contract versions.
 *
 *     It MUST NOT select a device, topology, qubit count, CPU count, GPU
 *     count, FPGA count, memory capacity, or physical address.
 *
 *
 * 12. Compiler
 *
 *     The compiler resolves compatibility after parsing.
 *
 *     Compatibility failure is a semantic diagnostic, not a parser action.
 *
 *
 * 13. Runtime
 *
 *     Runtime negotiation may consume the semantic compatibility result.
 *
 *     Runtime MUST NOT depend directly on this grammar.
 *
 *
 * 14. Rust 1.97 / 1.97.1
 *
 *     This .g4 file contains no Rust target-language code.
 *
 *     Generated parser/frontend integration MUST remain compatible with the
 *     repository's Rust 1.97 / 1.97.1 baseline and MUST use safe Rust only.
 *
 *
 * 15. No unsafe
 *
 *     This grammar contains no actions or embedded code and therefore cannot
 *     introduce Rust unsafe operations.
 *
 *
 * 16. Determinism
 *
 *     No semantic predicate, external lookup, filesystem access, network
 *     access, runtime call, hardware query, or random operation is permitted.
 *
 *
 * 17. Hard-coding audit
 *
 *     This file contains no:
 *
 *        machine maximum
 *        device count
 *        qubit count
 *        CPU count
 *        GPU count
 *        FPGA count
 *        node count
 *        memory size
 *        topology
 *        physical address
 *        backend identifier
 *        scheduler identifier
 *
 *
 * 18. Completion criterion
 *
 *     This file is complete when:
 *
 *        - ANTLR generation succeeds;
 *        - imports resolve;
 *        - no duplicate canonical version rules exist;
 *        - no lexer modification is required solely for compatibility;
 *        - dialects.g4 can consume dialectCompatibilityMember;
 *        - compatibility AST nodes preserve all declared structure;
 *        - version semantics remain delegated to Versioning;
 *        - compatibility semantics remain delegated to semantic analysis;
 *        - no hardware/runtime/IR dependency exists;
 *        - positive tests pass;
 *        - negative tests pass;
 *        - ambiguity tests pass;
 *        - deterministic parsing tests pass;
 *        - scalability tests contain no artificial grammar ceiling.
 *
 * ============================================================================
 */