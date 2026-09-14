/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/security/privacy.g4
 *
 * Role:
 *     Canonical parser-level grammar for source-level privacy declarations,
 *     privacy requirements, privacy constraints, privacy preferences,
 *     data-classification intent, processing-purpose declarations, disclosure
 *     rules, retention intent, minimization intent, consent references,
 *     jurisdiction references, privacy obligations, and privacy metadata.
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No filesystem access.
 *     - No network access.
 *     - No runtime calls.
 *     - No unsafe code.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical Zamani lexer
 *          |
 *          v
 *     parser
 *          |
 *          v
 *     Privacy syntax
 *          |
 *          v
 *     frontend AST
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> privacy analysis
 *          +--> security analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> policy analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL / hardware IR
 *          +--> distributed representation
 *          +--> security/privacy metadata
 *          |
 *          v
 *     optimization / scheduling / routing / resilience
 *          |
 *          v
 *     target lowering
 *          |
 *          v
 *     runtime / hardware / deployment
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - privacy declarations;
 *   - privacy requirements;
 *   - privacy constraints;
 *   - privacy preferences;
 *   - privacy policies;
 *   - processing-purpose declarations;
 *   - data-use intent;
 *   - disclosure intent;
 *   - retention intent;
 *   - minimization intent;
 *   - consent references;
 *   - privacy obligation declarations;
 *   - privacy classification references;
 *   - jurisdiction references;
 *   - privacy metadata.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - identity declarations;
 *   - principal declarations;
 *   - authorization;
 *   - permissions;
 *   - capability definitions;
 *   - authentication;
 *   - cryptographic algorithms;
 *   - cryptographic key material;
 *   - trust evaluation;
 *   - network implementation;
 *   - storage implementation;
 *   - database implementation;
 *   - data serialization;
 *   - runtime enforcement;
 *   - resource discovery;
 *   - hardware discovery;
 *   - quantum semantics;
 *   - quantum::ir;
 *   - QEC;
 *   - ZQN;
 *   - scheduling;
 *   - routing;
 *   - optimization;
 *   - backend selection.
 *
 * ============================================================================
 * PRIVACY PRINCIPLE
 * ============================================================================
 *
 * Privacy syntax describes WHAT privacy properties a program requires,
 * constrains, prefers, or declares.
 *
 * It does not implement privacy mechanisms.
 *
 * For example:
 *
 *     require privacy.minimization;
 *
 * describes semantic intent.
 *
 * It does NOT mean:
 *
 *     use database X;
 *     use storage device Y;
 *     use encryption provider Z;
 *     use machine N;
 *     use node N;
 *     use a particular operating system;
 *     use a particular cloud provider.
 *
 * Those implementation decisions belong to downstream semantic, resource,
 * compilation, deployment, and runtime systems.
 *
 * ============================================================================
 * DATA PRINCIPLE
 * ============================================================================
 *
 * Privacy declarations may refer to data semantically.
 *
 * They MUST NOT become a second data model.
 *
 * Canonical data structures remain owned by:
 *
 *     grammar/data/*
 *
 * Privacy may classify or constrain the use of data references.
 *
 * ============================================================================
 * IDENTITY PRINCIPLE
 * ============================================================================
 *
 * Identity and principal declarations belong to:
 *
 *     grammar/security/identities.g4
 *
 * This grammar may reference those identities/principals but MUST NOT
 * redefine them.
 *
 * ============================================================================
 * PERMISSION PRINCIPLE
 * ============================================================================
 *
 * Authorization belongs to:
 *
 *     grammar/security/permissions.g4
 *
 * Privacy syntax may express privacy obligations or conditions that are later
 * considered by authorization analysis, but it must not redefine allow/deny
 * semantics.
 *
 * ============================================================================
 * CRYPTOGRAPHY PRINCIPLE
 * ============================================================================
 *
 * Cryptographic intent belongs to:
 *
 *     grammar/security/cryptography.g4
 *
 * Privacy may require a property such as confidentiality or protected
 * disclosure, but algorithm selection and cryptographic implementation remain
 * owned by the cryptography subsystem.
 *
 * No secret, private key, password, credential material, or raw key material
 * belongs in this grammar.
 *
 * ============================================================================
 * TRUST PRINCIPLE
 * ============================================================================
 *
 * Trust relationships belong to:
 *
 *     grammar/security/trust.g4
 *
 * Privacy can reference trust-related requirements but does not evaluate trust.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Privacy classifications, purposes, jurisdictions, obligations, policies,
 * mechanisms, and future privacy concepts are represented using identifiers,
 * qualified names, and expressions.
 *
 * This grammar intentionally does NOT enumerate a finite list of privacy
 * regimes or jurisdictions.
 *
 * For example, it must not permanently encode a fixed enumeration of:
 *
 *     GDPR
 *     HIPAA
 *     CCPA
 *     LGPD
 *     POPIA
 *
 * as the only possible privacy models.
 *
 * Such regimes may be represented through namespaces and semantic registries.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * The same privacy-aware Zamani program must remain semantically meaningful
 * when executed on:
 *
 *     embedded systems
 *     CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     quantum systems
 *     distributed systems
 *     clusters
 *     clouds
 *     future architectures
 *
 * The grammar therefore contains no:
 *
 *     fixed node count
 *     fixed storage size
 *     fixed database size
 *     fixed memory size
 *     fixed data volume
 *     fixed number of principals
 *     fixed number of data subjects
 *     fixed number of policies
 *     fixed number of jurisdictions
 *     fixed number of processors
 *     fixed deployment topology
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no predicates;
 *     - no randomness;
 *     - no runtime state;
 *     - no filesystem access;
 *     - no network access;
 *     - no policy evaluation.
 *
 * Parsing therefore depends only on the supplied token stream.
 *
 * ============================================================================
 */

parser grammar Privacy;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. ENTRY POINT
 * ========================================================================== */

/*
 * Standalone entry point for grammar tests and tooling.
 *
 * The complete Zamani parser may import/reuse the individual rules instead
 * of using this EOF-consuming rule.
 */
privacyFile
    : privacyEntry* EOF
    ;

privacyEntry
    : privacyDeclaration
    | privacyPolicyDeclaration
    | privacyRequirementDeclaration
    | privacyConstraintDeclaration
    | privacyPreferenceDeclaration
    | privacyPurposeDeclaration
    | privacyClassificationDeclaration
    | privacyObligationDeclaration
    ;


/* ============================================================================
 * 2. PRIVACY DECLARATION
 * ========================================================================== */

privacyDeclaration
    : attributes*
      visibility?
      PRIVACY
      identifier
      genericParameters?
      privacyTargetClause?
      privacyBody?
      SEMI?
    ;

privacyTargetClause
    : FOR
      privacyReference
    ;

privacyBody
    : LBRACE
      privacyMember*
      RBRACE
    ;

privacyMember
    : privacyRequirement
    | privacyConstraint
    | privacyPreference
    | privacyPurposeReference
    | privacyClassificationReference
    | privacyDataUseDeclaration
    | privacyDisclosureDeclaration
    | privacyRetentionDeclaration
    | privacyMinimizationDeclaration
    | privacyConsentReference
    | privacyJurisdictionReference
    | privacyObligationReference
    | privacyProperty
    | privacyMetadata
    ;


/* ============================================================================
 * 3. PRIVACY POLICIES
 * ========================================================================== */

privacyPolicyDeclaration
    : attributes*
      visibility?
      POLICY
      identifier
      genericParameters?
      policyTargetClause?
      privacyPolicyBody?
      SEMI?
    ;

privacyPolicyBody
    : LBRACE
      privacyPolicyMember*
      RBRACE
    ;

privacyPolicyMember
    : privacyRule
    | privacyRequirement
    | privacyConstraint
    | privacyPreference
    | privacyPurposeReference
    | privacyClassificationReference
    | privacyDataUseDeclaration
    | privacyDisclosureDeclaration
    | privacyRetentionDeclaration
    | privacyMinimizationDeclaration
    | privacyConsentReference
    | privacyJurisdictionReference
    | privacyObligationReference
    | privacyProperty
    | privacyMetadata
    ;


/* ============================================================================
 * 4. PRIVACY RULES
 * ========================================================================== */

/*
 * A privacy rule is intentionally abstract.
 *
 * Authorization enforcement remains owned by permissions/security analysis.
 *
 * Privacy rules describe privacy intent and obligations.
 */
privacyRule
    : privacyCondition?
      privacyDecision
      privacyAction?
      SEMI?
    ;

privacyCondition
    : WHEN
      expression
    ;

privacyDecision
    : ALLOW
    | DENY
    | REQUIREMENT
    | REJECT
    ;

privacyAction
    : ON
      privacyReference
    ;


/* ============================================================================
 * 5. REQUIREMENTS
 * ========================================================================== */

privacyRequirementDeclaration
    : attributes*
      visibility?
      PRIVACY
      REQUIREMENT
      identifier?
      privacyRequirement
      SEMI?
    ;

privacyRequirement
    : REQUIRE
      privacyRequirementExpression
    ;

privacyRequirementExpression
    : privacyRequirementAll
    | privacyRequirementAny
    | privacyRequirementNot
    | privacyRequirementAtom
    ;

privacyRequirementAll
    : LBRACE
      privacyRequirementExpression
      (COMMA privacyRequirementExpression)*
      COMMA?
      RBRACE
    ;

privacyRequirementAny
    : LPAREN
      privacyRequirementExpression
      (OR privacyRequirementExpression)+
      RPAREN
    ;

privacyRequirementNot
    : NOT
      privacyRequirementExpression
    ;

privacyRequirementAtom
    : privacyReference
    | privacyPropertyExpression
    | privacyCallExpression
    | expression
    ;

privacyCallExpression
    : qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;

privacyPropertyExpression
    : privacyReference
      (IS | COLON)
      privacyValue
    ;

privacyValue
    : privacyReference
    | literal
    | expression
    ;


/* ============================================================================
 * 6. CONSTRAINTS
 * ========================================================================== */

privacyConstraintDeclaration
    : attributes*
      visibility?
      CONSTRAINT
      identifier
      privacyConstraintBody?
      SEMI?
    ;

privacyConstraintBody
    : LBRACE
      privacyConstraintMember*
      RBRACE
    ;

privacyConstraintMember
    : privacyConstraint
    | privacyRequirement
    | privacyPreference
    | privacyProperty
    | privacyMetadata
    ;

privacyConstraint
    : CONSTRAIN
      privacyConstraintExpression
    ;

privacyConstraintExpression
    : privacyReference
    | privacyPropertyExpression
    | privacyCallExpression
    | expression
    ;


/* ============================================================================
 * 7. PREFERENCES
 * ========================================================================== */

privacyPreferenceDeclaration
    : attributes*
      visibility?
      PREFERENCE
      identifier
      privacyPreferenceBody?
      SEMI?
    ;

privacyPreferenceBody
    : LBRACE
      privacyPreferenceMember*
      RBRACE
    ;

privacyPreferenceMember
    : privacyPreference
    | privacyRequirement
    | privacyConstraint
    | privacyProperty
    | privacyMetadata
    ;

privacyPreference
    : PREFER
      privacyPreferenceExpression
    ;

privacyPreferenceExpression
    : privacyReference
    | privacyPropertyExpression
    | privacyCallExpression
    | expression
    ;


/* ============================================================================
 * 8. DATA-USE INTENT
 * ========================================================================== */

/*
 * Privacy describes intended use of data.
 *
 * The data model itself remains in grammar/data/*.
 */
privacyDataUseDeclaration
    : DATA
      USE
      privacyDataReference
      privacyPurposeClause?
      privacyConditionClause?
      SEMI?
    ;

privacyDataReference
    : privacyReference
    ;

privacyPurposeClause
    : FOR
      privacyPurposeExpression
    ;

privacyPurposeExpression
    : privacyReference
    | expression
    ;

privacyConditionClause
    : WHEN
      expression
    ;


/* ============================================================================
 * 9. DISCLOSURE
 * ========================================================================== */

privacyDisclosureDeclaration
    : DISCLOSE
      privacyDataReference
      disclosureTargetClause?
      disclosurePurposeClause?
      disclosureConditionClause?
      SEMI?
    ;

disclosureTargetClause
    : TO
      privacyReference
    ;

disclosurePurposeClause
    : FOR
      privacyPurposeExpression
    ;

disclosureConditionClause
    : WHEN
      expression
    ;


/* ============================================================================
 * 10. RETENTION
 * ========================================================================== */

/*
 * Retention is expressed semantically.
 *
 * A duration/value may be represented by the shared expression/literal
 * infrastructure. This grammar does not impose a maximum retention period.
 */
privacyRetentionDeclaration
    : RETAIN
      privacyDataReference
      privacyRetentionValue?
      privacyConditionClause?
      SEMI?
    ;

privacyRetentionValue
    : FOR
      expression
    | UNTIL
      expression
    ;


/* ============================================================================
 * 11. MINIMIZATION
 * ========================================================================== */

privacyMinimizationDeclaration
    : MINIMIZE
      privacyDataReference?
      privacyMinimizationBody?
      SEMI?
    ;

privacyMinimizationBody
    : LBRACE
      privacyMinimizationMember*
      RBRACE
    ;

privacyMinimizationMember
    : privacyRequirement
    | privacyConstraint
    | privacyPreference
    | privacyProperty
    | privacyMetadata
    ;


/* ============================================================================
 * 12. CONSENT
 * ========================================================================== */

privacyConsentReference
    : CONSENT
      privacyReference
      privacyConditionClause?
      SEMI?
    ;


/* ============================================================================
 * 13. JURISDICTION
 * ========================================================================== */

/*
 * Jurisdictions are open-world references.
 *
 * The grammar does not hard-code a legal regime or geographic inventory.
 */
privacyJurisdictionReference
    : JURISDICTION
      qualifiedName
      SEMI?
    ;


/* ============================================================================
 * 14. PROCESSING PURPOSE
 * ========================================================================== */

privacyPurposeDeclaration
    : PURPOSE
      identifier
      privacyPurposeBody?
      SEMI?
    ;

privacyPurposeBody
    : LBRACE
      privacyPurposeMember*
      RBRACE
    ;

privacyPurposeMember
    : privacyRequirement
    | privacyConstraint
    | privacyPreference
    | privacyProperty
    | privacyMetadata
    ;

privacyPurposeReference
    : PURPOSE
      qualifiedName
    ;


/* ============================================================================
 * 15. DATA CLASSIFICATION
 * ========================================================================== */

/*
 * Classification labels are open-world.
 *
 * This allows future classification systems without grammar redesign.
 */
privacyClassificationDeclaration
    : CLASSIFICATION
      identifier
      privacyClassificationBody?
      SEMI?
    ;

privacyClassificationBody
    : LBRACE
      privacyClassificationMember*
      RBRACE
    ;

privacyClassificationMember
    : privacyClassificationLabel
    | privacyRequirement
    | privacyConstraint
    | privacyProperty
    | privacyMetadata
    ;

privacyClassificationLabel
    : identifier
      (ASSIGN expression)?
      SEMI?
    ;

privacyClassificationReference
    : CLASSIFICATION
      qualifiedName
    ;


/* ============================================================================
 * 16. OBLIGATIONS
 * ========================================================================== */

privacyObligationDeclaration
    : OBLIGATION
      identifier
      privacyObligationBody?
      SEMI?
    ;

privacyObligationBody
    : LBRACE
      privacyObligationMember*
      RBRACE
    ;

privacyObligationMember
    : privacyRequirement
    | privacyConstraint
    | privacyPreference
    | privacyActionDeclaration
    | privacyProperty
    | privacyMetadata
    ;

privacyObligationReference
    : OBLIGATION
      qualifiedName
    ;

privacyActionDeclaration
    : ACTION
      privacyReference
      SEMI?
    ;


/* ============================================================================
 * 17. PRIVACY PROPERTIES
 * ========================================================================== */

privacyProperty
    : identifier
      (COLON typeExpression)?
      (ASSIGN expression)?
      SEMI
    ;

privacyPropertyList
    : privacyProperty*
    ;


/* ============================================================================
 * 18. PRIVACY METADATA
 * ========================================================================== */

privacyMetadata
    : METADATA
      LBRACE
      privacyMetadataEntry*
      RBRACE
    ;

privacyMetadataEntry
    : identifier
      ASSIGN
      expression
      SEMI?
    ;


/* ============================================================================
 * 19. COMMON REFERENCES
 * ========================================================================== */

privacyReference
    : qualifiedName
    ;

privacyReferenceList
    : privacyReference
      (COMMA privacyReference)*
      COMMA?
    ;


/* ============================================================================
 * 20. COMMON DECLARATION LISTS
 * ========================================================================== */

privacyDeclarationList
    : privacyDeclaration*
    ;

privacyPolicyList
    : privacyPolicyDeclaration*
    ;

privacyRequirementList
    : privacyRequirementDeclaration*
    ;

privacyConstraintList
    : privacyConstraintDeclaration*
    ;

privacyPreferenceList
    : privacyPreferenceDeclaration*
    ;

privacyPurposeList
    : privacyPurposeDeclaration*
    ;

privacyClassificationList
    : privacyClassificationDeclaration*
    ;

privacyObligationList
    : privacyObligationDeclaration*
    ;