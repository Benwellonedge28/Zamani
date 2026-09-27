/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/dialects/compatibility.g4
 *
 * Grammar:
 *     DialectCompatibility
 *
 * Status:
 *     CANONICAL DIALECT COMPATIBILITY GRAMMAR
 *
 * Baseline:
 *     ANTLR4
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL DIALECT COMPATIBILITY CONTRACT SYNTAX.
 *
 * It describes relationships between symbolic language/dialect contracts:
 *
 *     compatibility
 *     requirements
 *     support
 *     conflicts
 *     equivalence
 *     migration
 *     replacement
 *     supersession
 *     deprecation
 *     compatibility profiles
 *     compatibility assertions
 *     compatibility metadata
 *
 * This grammar records compatibility INTENT.
 *
 * It does NOT determine whether a compatibility relationship is satisfied.
 *
 * Semantic analysis owns:
 *
 *     version resolution
 *     dialect resolution
 *     compatibility solving
 *     migration validation
 *     conflict detection
 *     policy evaluation
 *     diagnostics
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
 *     ZamaniParser
 *       |
 *       v
 *     dialectRegistration
 *       |
 *       v
 *     dialectCompatibilityMember       <-- THIS FILE
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic compatibility model
 *       |
 *       +--> dialect registry
 *       +--> version resolver
 *       +--> package/module resolver
 *       +--> capability resolver
 *       +--> migration analysis
 *       +--> compatibility policy
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +-------------------------------+
 *       |               |               |
 *       v               v               v
 *   classical IR    quantum::ir    HDL/hardware IR
 *       |               |               |
 *       +---------------+---------------+
 *                       |
 *                       v
 *              optimization / lowering
 *                       |
 *               routing / scheduling
 *                       |
 *                QEC / ZQN / resilience
 *                       |
 *                       v
 *                    HAL/runtime
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - compatibility member dispatch;
 *   - compatibility relationship syntax;
 *   - compatibility subject syntax;
 *   - compatibility version attachment;
 *   - compatibility predicates;
 *   - compatibility metadata;
 *   - compatibility profiles;
 *   - migration declarations;
 *   - replacement declarations;
 *   - supersession declarations;
 *   - deprecation declarations;
 *   - compatibility assertions;
 *   - compatibility groups;
 *   - compatibility sets;
 *   - reusable compatibility contract syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical identifiers;
 *   - lexical keywords;
 *   - qualified-name syntax;
 *   - version syntax;
 *   - semantic version comparison;
 *   - version satisfiability;
 *   - dialect declaration syntax;
 *   - dialect registration;
 *   - namespace resolution;
 *   - package resolution;
 *   - module resolution;
 *   - capability discovery;
 *   - resource discovery;
 *   - hardware discovery;
 *   - target selection;
 *   - scheduling;
 *   - routing;
 *   - optimization;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - quantum::ir;
 *   - classical IR;
 *   - HDL IR;
 *   - runtime execution.
 *
 * ============================================================================
 * DEPENDENCY OWNERSHIP
 * ============================================================================
 *
 * Names are owned by:
 *
 *     grammar/core/names.g4
 *
 * Version syntax is owned by:
 *
 *     grammar/core/versioning.g4
 *
 * Lexical tokens are owned by:
 *
 *     grammar/lexer/*
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The compatibility grammar MUST NOT redefine any of those facilities.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Compatibility describes durable SOURCE CONTRACTS.
 *
 * It MUST NOT encode temporary properties of a machine.
 *
 * Compatibility MUST NOT select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     device
 *     physical qubit
 *     physical node
 *     memory bank
 *     network endpoint
 *     hardware topology
 *     scheduler
 *     calibration
 *     backend
 *
 * Compatibility may describe that a dialect requires a capability or
 * language contract.
 *
 * Target realization remains downstream.
 *
 * Therefore:
 *
 *     Program Once
 *          ->
 *     Compile Once
 *          ->
 *     Run Everywhere
 *          ->
 *     Run Anywhere
 *          ->
 *     Run Forever
 *
 * remains an architectural property of the complete compiler pipeline.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO grammar-level limits on:
 *
 *     compatibility declarations
 *     subjects
 *     versions
 *     profiles
 *     predicates
 *     metadata entries
 *     migrations
 *     replacements
 *     deprecations
 *     compatibility sets
 *     dialect relationships
 *     namespace depth
 *
 * Repetition is represented using `*`, `+`, and recursive expressions.
 *
 * This file contains no:
 *
 *     MAX_DIALECTS
 *     MAX_COMPATIBILITY_RULES
 *     MAX_VERSIONS
 *     MAX_FEATURES
 *     MAX_TARGETS
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * Any practical limit is an implementation/resource limit, never a language
 * limit.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust;
 *     no actions;
 *     no semantic predicates;
 *     no filesystem access;
 *     no network access;
 *     no environment inspection;
 *     no hardware discovery;
 *     no runtime callbacks;
 *     no randomness.
 *
 * Identical source/token streams therefore receive identical syntactic
 * treatment.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every compatibility construct must preserve, directly or indirectly:
 *
 *     source span
 *     relationship kind
 *     subject identity
 *     version expression where present
 *     predicate where present
 *     migration target where present
 *     profile identity where present
 *     metadata where present
 *     source ordering
 *
 * The AST MUST NOT resolve:
 *
 *     versions
 *     dialects
 *     packages
 *     capabilities
 *     hardware
 *     resources
 *     targets
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     resolving subjects;
 *     resolving dialect identities;
 *     evaluating version expressions;
 *     checking compatibility;
 *     checking conflicts;
 *     validating migrations;
 *     validating replacements;
 *     validating supersession;
 *     applying deprecation policy;
 *     evaluating predicates;
 *     resolving metadata semantics;
 *     producing diagnostics.
 *
 * A syntactically valid compatibility declaration is NOT necessarily a
 * semantically valid compatibility relationship.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * Compatibility information may become semantic metadata attached to:
 *
 *     dialect contracts
 *     module contracts
 *     package contracts
 *     compilation-unit contracts
 *     capability contracts
 *     canonical semantic representations
 *
 * It MUST NOT create:
 *
 *     quantum::ir
 *     classical IR
 *     HDL IR
 *     hardware IR
 *     runtime state
 *     resource state
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust.
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and must require no `unsafe`.
 *
 * ============================================================================
 */

parser grammar DialectCompatibility;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Versioning
    ;


/*
 * ============================================================================
 * 1. PUBLIC INTEGRATION ENTRY POINT
 * ============================================================================
 *
 * `dialectCompatibilityMember` is the only rule that a parent dialect grammar
 * needs to know about.
 *
 * Canonical integration:
 *
 *     dialectRegistrationMember
 *         : ...
 *         | dialectCompatibilityMember
 *         ;
 *
 * The parent grammar remains responsible for establishing the surrounding
 * dialect declaration.
 *
 * This file therefore remains independently complete.
 */

dialectCompatibilityMember
    : dialectCompatibilityDeclaration
    | dialectCompatibilityProfile
    | dialectCompatibilityGroupDeclaration
    | dialectCompatibilitySetDeclaration
    | dialectMigrationDeclaration
    | dialectReplacementDeclaration
    | dialectSupersessionDeclaration
    | dialectDeprecationDeclaration
    | dialectCompatibilityAssertion
    ;


/*
 * ============================================================================
 * 2. GENERIC COMPATIBILITY DECLARATION
 * ============================================================================
 *
 * Canonical forms include:
 *
 *     compatible quantum::standard;
 *     compatible quantum::standard version >= 1.0.0;
 *
 *     requires quantum::dynamic_control;
 *     requires quantum::dynamic_control version >= 1.0.0;
 *
 *     supports quantum::standard version >= 1.0.0;
 *
 *     conflicts quantum::legacy;
 *
 *     equivalent classical::numeric;
 *
 * Relationship names are intentionally open where possible.
 *
 * Existing globally reserved words such as `requires` remain supported through
 * their canonical lexer token. Future relationship names may remain ordinary
 * identifiers instead of forcing global keyword expansion.
 */

dialectCompatibilityDeclaration
    : compatibilityRelation
      compatibilitySubjectList
      compatibilityVersionClause?
      compatibilityConditionClause*
      compatibilityMetadataClause*
      SEMICOLON
    ;


/*
 * ============================================================================
 * 3. RELATIONSHIP KIND
 * ============================================================================
 *
 * Canonical semantic relationship names:
 *
 *     compatible
 *     requires
 *     supports
 *     conflicts
 *     equivalent
 *
 * `requires` is already a Zamani lexical keyword.
 *
 * The other contextual names remain identifiers unless the canonical lexer
 * eventually reserves them.
 *
 * This rule deliberately avoids introducing new lexer tokens.
 */

compatibilityRelation
    : REQUIRES
    | identifier
    ;


/*
 * ============================================================================
 * 4. COMPATIBILITY SUBJECT
 * ============================================================================
 *
 * A compatibility subject is symbolic language-level identity.
 *
 * Examples:
 *
 *     quantum::standard
 *     quantum::openqasm
 *     classical::numeric
 *     hdl::rtl
 *     hardware::abstract
 *     ai::tensor
 *     distributed::execution
 *     organization::domain::dialect
 *
 * The grammar does not decide whether the subject exists.
 *
 * It is NOT a:
 *
 *     device identifier
 *     physical qubit identifier
 *     memory address
 *     network endpoint
 *     filesystem path
 *     hardware location
 */

compatibilitySubject
    : qualifiedName
    ;

compatibilitySubjectList
    : compatibilitySubject
      (COMMA compatibilitySubject)*
    ;


/*
 * ============================================================================
 * 5. VERSION ATTACHMENT
 * ============================================================================
 *
 * Version syntax is entirely delegated to core/versioning.g4.
 *
 * No version grammar is duplicated here.
 */

compatibilityVersionClause
    : VERSION versionExpression
    ;


/*
 * ============================================================================
 * 6. CONDITIONS
 * ============================================================================
 *
 * Conditions are declarative compatibility predicates.
 *
 * They do not execute code.
 *
 * They do not query hardware.
 *
 * They do not inspect the filesystem.
 *
 * They do not access the network.
 *
 * They are later interpreted by semantic compatibility analysis.
 *
 * Canonical examples:
 *
 *     when feature == "x"
 *     where api::surface == "stable"
 *     if language::version >= 1
 *
 * The marker is intentionally open-world.
 */

compatibilityConditionClause
    : compatibilityConditionMarker compatibilityPredicate
    ;

compatibilityConditionMarker
    : WHEN
    | WHERE
    | identifier
    ;


/*
 * ============================================================================
 * 7. PREDICATE EXPRESSION
 * ============================================================================
 *
 * The predicate grammar uses the canonical Zamani logical keyword/operator
 * vocabulary.
 *
 * AND / OR are existing keyword tokens.
 *
 * Parentheses provide explicit grouping.
 *
 * Logical negation is intentionally not introduced here through the existing
 * `NOT` token because the repository currently has competing lexical ownership
 * for `not` and `!`. Compatibility syntax therefore remains monotonic until
 * that repository-wide lexical conflict is resolved.
 *
 * Semantic consumers can represent negation through explicit comparison
 * predicates or a future canonical logical-negation contract.
 */

compatibilityPredicate
    : compatibilityPredicateOr
    ;

compatibilityPredicateOr
    : compatibilityPredicateAnd
      (OR compatibilityPredicateAnd)*
    ;

compatibilityPredicateAnd
    : compatibilityPredicatePrimary
      (AND compatibilityPredicatePrimary)*
    ;

compatibilityPredicatePrimary
    : compatibilityPredicateAtom
    | LPAREN compatibilityPredicateOr RPAREN
    ;

compatibilityPredicateAtom
    : qualifiedName
    | qualifiedName compatibilityComparisonOperator compatibilityValue
    ;


/*
 * ============================================================================
 * 8. COMPARISON OPERATORS
 * ============================================================================
 *
 * These are the canonical lexer token names from grammar/lexer/operators.g4.
 *
 * Do NOT use obsolete aliases such as:
 *
 *     EQ
 *     NE
 *     LT
 *     LE
 *     GT
 *     GE
 *
 * The canonical names are:
 *
 *     EQUAL_EQUAL
 *     NOT_EQUAL
 *     LESS
 *     LESS_EQUAL
 *     GREATER
 *     GREATER_EQUAL
 *
 * This keeps compatibility.g4 aligned with the actual lexer vocabulary.
 */

compatibilityComparisonOperator
    : EQUAL_EQUAL
    | NOT_EQUAL
    | LESS
    | LESS_EQUAL
    | GREATER
    | GREATER_EQUAL
    ;


/*
 * ============================================================================
 * 9. COMPATIBILITY VALUES
 * ============================================================================
 *
 * Values are source-level data.
 *
 * They do not imply a machine representation.
 *
 * For example:
 *
 *     1024
 *
 * remains a source integer.
 *
 * It does NOT mean:
 *
 *     1024 qubits
 *     1024 CPUs
 *     1024 devices
 *
 * unless semantic analysis gives that value such meaning.
 */

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


/*
 * ============================================================================
 * 10. OPEN-WORLD METADATA
 * ============================================================================
 *
 * Metadata is intentionally open.
 *
 * This avoids requiring a grammar change every time compatibility tooling
 * gains a new descriptive property.
 *
 * Examples:
 *
 *     stability = stable
 *     owner = organization::domain
 *     reason = "migration required"
 *
 * Unknown metadata remains syntax-valid and is validated semantically.
 */

compatibilityMetadataClause
    : compatibilityMetadataKey
      compatibilityMetadataValue?
    ;

compatibilityMetadataKey
    : identifier
    ;

compatibilityMetadataValue
    : ASSIGN compatibilityValue
    | LPAREN compatibilityValueList? RPAREN
    ;


/*
 * ============================================================================
 * 11. COMPATIBILITY PROFILE
 * ============================================================================
 *
 * Profiles provide named groups of compatibility contracts.
 *
 * Example:
 *
 *     profile portable {
 *         compatible quantum::standard version >= 1.0.0;
 *         supports classical::numeric;
 *     }
 *
 * Profile membership does not automatically imply that every member is
 * mutually compatible.
 *
 * Semantic analysis determines profile meaning.
 */

dialectCompatibilityProfile
    : PROFILE compatibilityProfileName
      LBRACE
      dialectCompatibilityProfileMember*
      RBRACE
    ;

compatibilityProfileName
    : identifier
    | STRING
    ;

dialectCompatibilityProfileMember
    : dialectCompatibilityDeclaration
    | dialectCompatibilityGroupDeclaration
    | dialectCompatibilitySetDeclaration
    | dialectMigrationDeclaration
    | dialectReplacementDeclaration
    | dialectSupersessionDeclaration
    | dialectDeprecationDeclaration
    | dialectCompatibilityAssertion
    | compatibilityMetadataClause
    ;


/*
 * ============================================================================
 * 12. COMPATIBILITY GROUP
 * ============================================================================
 *
 * A group provides explicit structure for several compatibility contracts.
 *
 * Example:
 *
 *     compatibility {
 *         compatible quantum::standard;
 *         supports quantum::dynamic_control;
 *     }
 *
 * The grammar imposes no number of entries.
 */

dialectCompatibilityGroupDeclaration
    : compatibilityGroupMarker
      LBRACE
      dialectCompatibilityGroupMember*
      RBRACE
    ;

compatibilityGroupMarker
    : identifier
    ;

dialectCompatibilityGroupMember
    : dialectCompatibilityDeclaration
    | dialectMigrationDeclaration
    | dialectReplacementDeclaration
    | dialectSupersessionDeclaration
    | dialectDeprecationDeclaration
    | dialectCompatibilityAssertion
    | compatibilityMetadataClause
    ;


/*
 * ============================================================================
 * 13. COMPATIBILITY SET
 * ============================================================================
 *
 * A set groups subjects without claiming pairwise compatibility.
 *
 * Example:
 *
 *     compatibility_set portable {
 *         quantum::standard,
 *         quantum::openqasm,
 *         quantum::future
 *     }
 *
 * The actual relationships remain explicit compatibility contracts.
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


/*
 * ============================================================================
 * 14. MIGRATION
 * ============================================================================
 *
 * Migration describes a semantic evolution relationship.
 *
 * Example:
 *
 *     migrate quantum::legacy to quantum::standard;
 *
 * Migration does NOT perform rewriting.
 *
 * Migration algorithms remain outside the grammar.
 */

dialectMigrationDeclaration
    : migrationMarker
      compatibilitySubject
      migrationTargetClause
      compatibilityVersionClause?
      compatibilityConditionClause*
      compatibilityMetadataClause*
      SEMICOLON
    ;

migrationMarker
    : identifier
    ;

migrationTargetClause
    : migrationTargetMarker
      compatibilitySubject
    ;

migrationTargetMarker
    : identifier
    ;


/*
 * ============================================================================
 * 15. REPLACEMENT
 * ============================================================================
 *
 * Replacement records a successor contract.
 *
 * Example:
 *
 *     replace quantum::legacy with quantum::standard;
 *
 * The grammar records intent only.
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
    : replacementTargetMarker
      compatibilitySubject
    ;

replacementTargetMarker
    : identifier
    ;


/*
 * ============================================================================
 * 16. SUPERSESSION
 * ============================================================================
 *
 * Supersession differs from replacement in that the old contract may remain
 * semantically meaningful while another contract becomes its successor.
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
    : supersessionTargetMarker
      compatibilitySubject
    ;

supersessionTargetMarker
    : identifier
    ;


/*
 * ============================================================================
 * 17. DEPRECATION
 * ============================================================================
 *
 * Deprecation is metadata.
 *
 * It does not automatically reject source.
 *
 * Semantic/compiler policy determines whether the result is:
 *
 *     informational
 *     warning
 *     error
 *     migration-required
 */

dialectDeprecationDeclaration
    : deprecationMarker
      compatibilitySubject
      compatibilityVersionClause?
      compatibilityMetadataClause*
      SEMICOLON
    ;

deprecationMarker
    : identifier
    ;


/*
 * ============================================================================
 * 18. COMPATIBILITY ASSERTION
 * ============================================================================
 *
 * An assertion requests semantic validation of a compatibility claim.
 *
 * Example:
 *
 *     assert_compatible quantum::standard version >= 1.0.0;
 *
 * The assertion itself performs no evaluation during parsing.
 */

dialectCompatibilityAssertion
    : compatibilityAssertionMarker
      compatibilitySubjectList
      compatibilityVersionClause?
      compatibilityConditionClause*
      compatibilityMetadataClause*
      SEMICOLON
    ;

compatibilityAssertionMarker
    : identifier
    ;


/*
 * ============================================================================
 * 19. REUSABLE COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This rule is provided for other dialect grammar components that need to
 * embed a compatibility contract without duplicating its syntax.
 *
 * It intentionally excludes the terminating semicolon so that a containing
 * grammar can determine ownership of termination.
 */

compatibilityContract
    : compatibilityRelation
      compatibilitySubjectList
      compatibilityVersionClause?
      compatibilityConditionClause*
      compatibilityMetadataClause*
    ;


/*
 * ============================================================================
 * 20. REUSABLE VERSIONED SUBJECT
 * ============================================================================
 *
 * This is a small integration boundary for vendor, experimental, and
 * registration grammars.
 */

compatibilitySubjectWithVersion
    : compatibilitySubject
      compatibilityVersionClause?
    ;

compatibilitySubjectWithVersionList
    : compatibilitySubjectWithVersion
      (COMMA compatibilitySubjectWithVersion)*
    ;


/*
 * ============================================================================
 * 21. REUSABLE SUBJECT GROUP
 * ============================================================================
 */

compatibilitySubjectGroup
    : LBRACE
      compatibilitySubjectList?
      RBRACE
    ;


/*
 * ============================================================================
 * 22. REUSABLE METADATA LIST
 * ============================================================================
 */

compatibilityMetadataList
    : compatibilityMetadataClause*
    ;


/*
 * ============================================================================
 * 23. COMPATIBILITY MATRIX ENTRY
 * ============================================================================
 *
 * A matrix entry expresses a directed semantic relationship between two
 * symbolic subjects.
 *
 * Example:
 *
 *     quantum::legacy -> quantum::standard;
 *
 * The arrow is the canonical THIN_ARROW token.
 *
 * It does NOT represent execution flow.
 */

compatibilityMatrixEntry
    : compatibilitySubject
      THIN_ARROW
      compatibilitySubject
      compatibilityVersionClause?
      compatibilityConditionClause*
      compatibilityMetadataClause*
      SEMICOLON
    ;


/*
 * ============================================================================
 * 24. COMPATIBILITY MATRIX
 * ============================================================================
 *
 * Example:
 *
 *     matrix compatibility {
 *         quantum::legacy -> quantum::standard;
 *         hdl::legacy -> hdl::rtl;
 *     }
 *
 * The matrix is open-ended.
 */

dialectCompatibilityMatrix
    : compatibilityMatrixMarker
      compatibilityMatrixName
      LBRACE
      compatibilityMatrixEntry*
      RBRACE
    ;

compatibilityMatrixMarker
    : identifier
    ;

compatibilityMatrixName
    : identifier
    | STRING
    ;


/*
 * ============================================================================
 * 25. VERSIONED COMPATIBILITY SUBJECT
 * ============================================================================
 *
 * Convenience boundary for consumers that need a subject/version pair.
 */

versionedCompatibilitySubject
    : compatibilitySubject
      compatibilityVersionClause
    ;


/*
 * ============================================================================
 * 26. OPTIONAL VERSION
 * ============================================================================
 */

optionalCompatibilityVersionClause
    : compatibilityVersionClause?
    ;


/*
 * ============================================================================
 * 27. OPTIONAL CONDITION
 * ============================================================================
 */

optionalCompatibilityConditionClause
    : compatibilityConditionClause?
    ;


/*
 * ============================================================================
 * 28. OPTIONAL METADATA
 * ============================================================================
 */

optionalCompatibilityMetadata
    : compatibilityMetadataClause*
    ;


/*
 * ============================================================================
 * 29. COMPLETION / INTEGRATION CONTRACT
 * ============================================================================
 *
 * Parent grammar:
 *
 *     grammar/dialects/registration.g4
 *
 * MUST expose:
 *
 *     dialectRegistrationMember
 *
 * and integrate this grammar through:
 *
 *     dialectCompatibilityMember
 *
 * The preferred final dispatch is:
 *
 *     dialectRegistrationMember
 *         : dialectRegistrationImport
 *         | dialectRegistrationUse
 *         | dialectRegistrationExtends
 *         | dialectRegistrationRequires
 *         | dialectRegistrationProvides
 *         | dialectRegistrationExtension
 *         | dialectCompatibilityMember
 *         | dialectRegistrationSyntax
 *         | dialectRegistrationSemantics
 *         | dialectRegistrationLowering
 *         | dialectRegistrationProperty
 *         | dialectRegistrationAnnotation
 *         ;
 *
 * The old compatibility implementation in registration.g4:
 *
 *     dialectRegistrationCompatibility
 *     dialectCompatibilityExpression
 *     dialectCompatibilityAtom
 *     dialectCompatibilityGroup
 *
 * MUST NOT remain as a second compatibility authority.
 *
 * It must be removed from the registration grammar when this grammar is
 * imported.
 *
 * This is deliberate: two compatibility grammars would violate the repository
 * requirement of one authoritative grammar contract.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * `dialects.g4` remains the public dialect parser boundary.
 *
 * `registration.g4` remains the dialect registration owner.
 *
 * `compatibility.g4` owns only compatibility members.
 *
 * `versioning.g4` remains the dialect-specific wrapper around core versioning.
 *
 * `core/versioning.g4` remains the canonical version-expression authority.
 *
 * `core/names.g4` remains the canonical qualified-name authority.
 *
 * ============================================================================
 * VENDOR / EXPERIMENTAL INTEGRATION
 * ============================================================================
 *
 * Vendor and experimental dialect grammars may consume:
 *
 *     dialectCompatibilityMember
 *     compatibilityContract
 *     compatibilitySubjectWithVersion
 *     compatibilitySubjectWithVersionList
 *     compatibilityMetadataClause
 *
 * They MUST NOT duplicate compatibility predicates or version syntax.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Compatibility may refer to capability identities as symbolic qualified
 * names:
 *
 *     requires quantum::dynamic_control;
 *     supports hardware::programmable_logic;
 *
 * This grammar does not determine whether the capability is actually
 * available.
 *
 * Capability resolution belongs to:
 *
 *     dialects/capabilities.g4
 *     semantic analysis
 *     target capability negotiation
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum dialect compatibility remains symbolic.
 *
 * Valid semantic examples include:
 *
 *     compatible quantum::standard version >= 1.0.0;
 *     supports quantum::dynamic_control;
 *     conflicts quantum::legacy;
 *
 * This grammar MUST NOT contain:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     topology
 *     calibration
 *     pulse
 *     routing
 *     scheduling
 *     QEC implementation
 *
 * Quantum compatibility eventually attaches metadata to the canonical
 * semantic model and, where applicable, the existing `quantum::ir` boundary.
 *
 * It does NOT create another quantum IR.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Compatibility subjects may identify HDL/hardware language contracts:
 *
 *     hdl::rtl
 *     hardware::programmable_logic
 *     hardware::synthesis
 *
 * They do not identify:
 *
 *     a particular FPGA
 *     a particular ASIC
 *     a particular CPU
 *     a particular GPU
 *     a particular device
 *     a fixed register width
 *     a fixed memory capacity
 *     a fixed topology
 *
 * ============================================================================
 * DISTRIBUTED / AI / DATA INTEGRATION
 * ============================================================================
 *
 * The same symbolic compatibility mechanism applies to:
 *
 *     distributed
 *     networking
 *     AI
 *     tensor
 *     data
 *     security
 *     accelerators
 *     embedded systems
 *     HPC
 *     cloud
 *     future computational domains
 *
 * No domain-specific compatibility enumeration belongs here.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     MAX_DIALECTS
 *     MAX_COMPATIBILITY_RULES
 *     MAX_VERSIONS
 *     MAX_FEATURES
 *     MAX_TARGETS
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *
 * Also forbidden are fixed enumerations of:
 *
 *     known dialect names
 *     known vendors
 *     known hardware
 *     known quantum processors
 *     known AI frameworks
 *     known HDL implementations
 *
 * New domains must be representable using the existing symbolic structures.
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The grammar test suite MUST reject malformed compatibility constructs such
 * as:
 *
 *     compatible ;
 *     compatible ::name;
 *     compatible name::;
 *     compatible name::::other;
 *     compatible name version;
 *     migrate source;
 *     migrate source to;
 *     replace source;
 *     replace source with;
 *     profile;
 *     profile { }
 *     matrix;
 *
 * Exact diagnostics belong to parser/diagnostic infrastructure.
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The grammar test suite MUST accept, at minimum:
 *
 *     compatible quantum::standard;
 *
 *     compatible quantum::standard version >= 1.0.0;
 *
 *     requires quantum::dynamic_control;
 *
 *     supports hardware::programmable_logic;
 *
 *     conflicts quantum::legacy;
 *
 *     equivalent classical::numeric;
 *
 *     migrate quantum::legacy to quantum::standard;
 *
 *     replace hdl::legacy with hdl::rtl;
 *
 *     supersede ai::legacy with ai::tensor;
 *
 *     deprecated quantum::old;
 *
 *     profile portable {
 *         compatible quantum::standard;
 *         supports quantum::dynamic_control;
 *     }
 *
 *     compatibility {
 *         compatible quantum::standard;
 *         requires classical::numeric;
 *     }
 *
 *     compatibility_set portable {
 *         quantum::standard,
 *         quantum::openqasm,
 *         quantum::future
 *     }
 *
 *     assert_compatible quantum::standard version >= 1.0.0;
 *
 *     matrix compatibility {
 *         quantum::legacy -> quantum::standard;
 *     }
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Tests MUST cover:
 *
 *     one subject
 *     many subjects
 *     deeply qualified names
 *     long metadata lists
 *     nested predicates
 *     nested profiles
 *     nested compatibility groups
 *     large migration sets
 *     large compatibility sets
 *     large matrices
 *     arbitrary version magnitude
 *
 * No test may establish a finite language-level maximum.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Given identical:
 *
 *     source
 *     lexer vocabulary
 *     grammar version
 *
 * parsing must produce identical:
 *
 *     parse structure
 *     token consumption
 *     source spans
 *     diagnostics
 *
 * Behavior must not depend upon:
 *
 *     CPU count
 *     GPU availability
 *     QPU availability
 *     filesystem state
 *     network state
 *     wall-clock time
 *     environment state
 *     random state
 *     deployment topology
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] Compatibility has one grammar authority.
 *
 * [x] Names delegate to core/names.g4.
 *
 * [x] Versions delegate to core/versioning.g4.
 *
 * [x] Lexer tokens match the canonical lexer vocabulary.
 *
 * [x] No obsolete EQ/NE/LT/LE/GT/GE aliases are used.
 *
 * [x] No new global compatibility keywords are required.
 *
 * [x] Compatibility relationships are open-world.
 *
 * [x] Compatibility metadata is open-world.
 *
 * [x] Profiles are open-ended.
 *
 * [x] Migration is declarative.
 *
 * [x] Replacement is declarative.
 *
 * [x] Supersession is declarative.
 *
 * [x] Deprecation is declarative.
 *
 * [x] No hardware limits are encoded.
 *
 * [x] No resource limits are encoded.
 *
 * [x] No quantum gate set is encoded.
 *
 * [x] No physical target is selected.
 *
 * [x] No IR is created.
 *
 * [x] No semantic compatibility algorithm is embedded.
 *
 * [x] No filesystem/network/runtime access exists.
 *
 * [x] No Rust or unsafe code is embedded.
 *
 * [x] Rust 1.97/1.97.1 generated-parser integration remains possible.
 *
 * [x] POCO-REAF is preserved.
 *
 * [x] Classical, quantum, HDL, hardware, AI, distributed, data, networking,
 *     security and future dialects can use the same compatibility mechanism.
 *
 * ============================================================================
 * FINAL RULE
 * ============================================================================
 *
 * Compatibility syntax describes RELATIONSHIPS BETWEEN LANGUAGE CONTRACTS.
 *
 * It does not describe machines.
 *
 * Therefore:
 *
 *     dialect compatibility
 *         !=
 *     hardware compatibility
 *
 *     language version
 *         !=
 *     machine version
 *
 *     capability requirement
 *         !=
 *     physical device selection
 *
 *     compatibility syntax
 *         !=
 *     compatibility algorithm
 *
 * The complete pipeline remains:
 *
 *     Zamani source
 *          |
 *          v
 *     lexer
 *          |
 *          v
 *     parser
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic compatibility analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          |
 *          v
 *     optimization / lowering
 *          |
 *          v
 *     routing / scheduling / resilience / QEC / ZQN
 *          |
 *          v
 *     HAL / runtime
 *          |
 *          v
 *     target realization
 *
 * This is the compatibility boundary required for scalable POCO-REAF.
 *
 * ============================================================================
 */