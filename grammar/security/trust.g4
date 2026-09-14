/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/security/trust.g4
 *
 * Role:
 *     Canonical parser-level grammar for source-level trust declarations,
 *     trust relationships, trust requirements, trust preferences, trust
 *     assertions, trust references, and trust metadata.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Language/runtime baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No unsafe code.
 *     - No filesystem access.
 *     - No network access.
 *     - No runtime calls.
 *     - No hardware discovery.
 *     - No cryptographic execution.
 *     - No identity-provider access.
 *     - No policy evaluation.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     Core / Types / Expressions
 *          |
 *          v
 *     Trust parser
 *          |
 *          v
 *     frontend AST
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> security analysis
 *          +--> trust analysis
 *          +--> capability analysis
 *          +--> policy analysis
 *          |
 *          v
 *     canonical semantic representations
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL / hardware representation
 *          +--> security metadata
 *          |
 *          v
 *     compilation / deployment / runtime
 *
 * Trust syntax is therefore ABOVE semantic analysis and BELOW the canonical
 * lexer. It does not become an IR and does not call runtime systems.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - trust declaration syntax;
 *   - trust relationship syntax;
 *   - trust requirement syntax;
 *   - trust preference syntax;
 *   - trust assertion syntax;
 *   - trust references;
 *   - trust source/subject references;
 *   - trust target/object references;
 *   - trust metadata;
 *   - trust conditions;
 *   - trust qualifiers;
 *   - trust evidence references;
 *   - trust scope;
 *   - trust validity expressions;
 *   - trust lifecycle intent;
 *   - trust composition syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - identity declaration;
 *   - principal declaration;
 *   - identity verification;
 *   - authentication;
 *   - authorization;
 *   - permission declaration;
 *   - capability declaration;
 *   - cryptographic algorithms;
 *   - key material;
 *   - certificates;
 *   - credential storage;
 *   - trust-store implementation;
 *   - certificate-chain validation;
 *   - signature verification;
 *   - policy evaluation;
 *   - privacy policy;
 *   - security enforcement;
 *   - hardware trust anchors;
 *   - TPM/HSM/enclave implementation;
 *   - network transport;
 *   - quantum semantics;
 *   - quantum::ir;
 *   - QEC;
 *   - ZQN;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - backend selection;
 *   - runtime execution.
 *
 * Identity syntax belongs to:
 *
 *     security/identities.g4
 *
 * Authorization belongs to:
 *
 *     security/permissions.g4
 *
 * Security capabilities belong to:
 *
 *     security/capabilities.g4
 *
 * Cryptographic intent belongs to:
 *
 *     security/cryptography.g4
 *
 * Privacy intent belongs to:
 *
 *     security/privacy.g4
 *
 * Security-wide composition belongs to:
 *
 *     security/security.g4
 *
 * ============================================================================
 * TRUST SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Trust describes a declared relationship or requirement concerning whether
 * one abstract entity may be relied upon by another for a stated purpose,
 * scope, condition, or evidence set.
 *
 * A trust declaration does NOT establish that the relationship is actually
 * trustworthy.
 *
 * For example:
 *
 *     trust application from identity::alice to service::compute;
 *
 * expresses source-level intent.
 *
 * It does NOT mean that:
 *
 *     alice has been authenticated;
 *     a certificate has been verified;
 *     a signature has been checked;
 *     a trust anchor exists;
 *     a provider has been contacted;
 *     a hardware root of trust exists.
 *
 * Those are downstream semantic/runtime responsibilities.
 *
 * ============================================================================
 * OPEN-WORLD TRUST MODEL
 * ============================================================================
 *
 * Trust relationships are deliberately open-world.
 *
 * The grammar MUST NOT enumerate finite sets of:
 *
 *   - trust providers;
 *   - identity providers;
 *   - certificate authorities;
 *   - algorithms;
 *   - trust anchors;
 *   - hardware roots of trust;
 *   - enclaves;
 *   - vendors;
 *   - cloud providers;
 *   - machines;
 *   - devices;
 *   - principals;
 *   - domains;
 *   - future trust mechanisms.
 *
 * Qualified names and general expressions represent these concepts.
 *
 * Therefore introducing a new trust mechanism does not require changing this
 * grammar.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Trust syntax expresses PORTABLE TRUST INTENT.
 *
 * It MUST NOT encode a temporary physical realization as permanent program
 * semantics.
 *
 * A declaration such as:
 *
 *     trust execution from security::authority to compute::service;
 *
 * does not imply:
 *
 *     use machine X;
 *     use device Y;
 *     use provider Z;
 *     use N nodes;
 *     use N CPUs;
 *     use N GPUs;
 *     use N qubits;
 *     use a particular enclave;
 *     use a particular network;
 *
 * Target selection, resource selection, trust-anchor resolution, verification,
 * and enforcement occur downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO grammar-level finite limits on:
 *
 *   - trust declarations;
 *   - trust relationships;
 *   - trust conditions;
 *   - evidence references;
 *   - metadata entries;
 *   - scopes;
 *   - validity expressions;
 *   - trust requirements;
 *   - trust preferences;
 *   - nested expressions;
 *   - declaration count.
 *
 * Repetition uses ANTLR repetition operators.
 *
 * The grammar MUST NOT contain:
 *
 *     MAX_TRUST_RELATIONSHIPS
 *     MAX_PRINCIPALS
 *     MAX_TRUST_ANCHORS
 *     MAX_DOMAINS
 *     MAX_DEVICES
 *     MAX_NODES
 *
 * or equivalent artificial limits.
 *
 * Practical limits are parser/compiler/resource-policy concerns and are not
 * language semantics.
 *
 * ============================================================================
 * IDENTITY BOUNDARY
 * ============================================================================
 *
 * Trust may refer to identities and principals, but does not declare them.
 *
 * For example:
 *
 *     FROM identity::alice
 *
 * refers to an identity owned by the identity grammar/semantic layer.
 *
 * Trust does not redefine:
 *
 *     Principal
 *     Identity
 *     IdentityBinding
 *     Credential
 *
 * ============================================================================
 * AUTHENTICATION BOUNDARY
 * ============================================================================
 *
 * Trust may express requirements concerning authentication, but it does not
 * perform authentication.
 *
 * Authentication mechanisms remain downstream.
 *
 * ============================================================================
 * AUTHORIZATION BOUNDARY
 * ============================================================================
 *
 * Trust may be consumed during authorization analysis, but trust does not
 * define allow/deny policy semantics.
 *
 * Authorization remains owned by:
 *
 *     security/permissions.g4
 *
 * ============================================================================
 * CRYPTOGRAPHY BOUNDARY
 * ============================================================================
 *
 * Trust may reference cryptographic evidence or cryptographic requirements
 * using open-world names and expressions.
 *
 * It does NOT define:
 *
 *     encryption;
 *     decryption;
 *     signing;
 *     verification;
 *     key generation;
 *     key storage;
 *     certificate implementation;
 *     algorithm implementations.
 *
 * No secret material may appear as a trust-language primitive.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Trust syntax can apply to quantum computation.
 *
 * Examples include:
 *
 *     trusted quantum execution;
 *     trusted quantum-classical control;
 *     trusted QPU capability;
 *     trust requirements for quantum services.
 *
 * However, this grammar MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     quantum topology
 *     calibration
 *     QEC codes
 *     ZQN faults
 *     backend selection
 *
 * Quantum semantics remain downstream and the canonical quantum semantic
 * boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 * HARDWARE BOUNDARY
 * ============================================================================
 *
 * Trust may refer to abstract hardware capabilities.
 *
 * Example:
 *
 *     requires trust::trusted_execution;
 *
 * does not mean:
 *
 *     use CPU X;
 *     use GPU Y;
 *     use FPGA Z;
 *     use QPU N;
 *     use enclave E.
 *
 * Hardware realization belongs to hardware capability, resource, target,
 * compilation, deployment, and runtime layers.
 *
 * ============================================================================
 * DATA BOUNDARY
 * ============================================================================
 *
 * Trust references data or services abstractly.
 *
 * This grammar does not define:
 *
 *     data schemas;
 *     database structures;
 *     storage formats;
 *     memory layouts;
 *     filesystem paths;
 *     network addresses.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser preserves:
 *
 *   - declaration kind;
 *   - source ordering;
 *   - source spelling;
 *   - qualified-name structure;
 *   - expressions;
 *   - conditions;
 *   - metadata;
 *   - source spans supplied by the frontend.
 *
 * Semantic analysis may construct:
 *
 *   TrustRelationship
 *   TrustRequirement
 *   TrustPreference
 *   TrustAssertion
 *   TrustScope
 *   TrustCondition
 *   TrustEvidence
 *   TrustMetadata
 *
 * The parser MUST NOT construct those semantic objects.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic for a deterministic token stream.
 *
 * This grammar contains:
 *
 *   - no actions;
 *   - no semantic predicates;
 *   - no I/O;
 *   - no randomness;
 *   - no hardware discovery;
 *   - no runtime state;
 *   - no trust evaluation.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * This parser consumes canonical shared syntax from:
 *
 *     Core
 *     Types
 *     Expressions
 *
 * It MUST NOT define another lexer.
 *
 * Canonical shared rules consumed here include:
 *
 *     identifier
 *     qualifiedName
 *     attributes
 *     visibility
 *     genericParameters
 *     typeExpression
 *     expression
 *     literal
 *     argumentList
 *
 * If a repository version names one of these rules differently, the canonical
 * rule must be reconciled at the shared grammar layer rather than creating a
 * trust-local duplicate.
 *
 * ============================================================================
 */

parser grammar Trust;

options {
    tokenVocab = ZamaniLexer;
}

import Core, Types, Expressions;


/* ============================================================================
 * 1. TRUST ROOT
 * ========================================================================== */

/*
 * Entry point for trust syntax when this grammar is composed independently.
 *
 * The security aggregator may compose this rule into its broader security
 * declaration grammar.
 */
trustFile
    : trustDeclaration* EOF
    ;


/*
 * A trust declaration is one source-level trust construct.
 */
trustDeclaration
    : trustRelationshipDeclaration
    | trustRequirementDeclaration
    | trustPreferenceDeclaration
    | trustAssertionDeclaration
    ;


/* ============================================================================
 * 2. TRUST RELATIONSHIPS
 * ========================================================================== */

/*
 * Declares a relationship in which one abstract entity is treated as a
 * potential source of trust for another abstract entity.
 *
 * Example:
 *
 *     trust serviceTrust
 *         from identity::authority
 *         to service::compute;
 *
 * The grammar does not determine whether the relationship is valid.
 */
trustRelationshipDeclaration
    : attributes?
      visibility?
      TRUST
      identifier?
      trustSourceClause
      trustTargetClause
      trustRelationshipBody?
      SEMI?
    ;


trustSourceClause
    : FROM
      trustReference
    ;


trustTargetClause
    : TO
      trustReference
    ;


trustRelationshipBody
    : LBRACE
      trustMember*
      RBRACE
    ;


trustMember
    : trustProperty
    | trustCondition
    | trustScope
    | trustEvidence
    | trustValidity
    | trustRequirement
    | trustPreference
    | trustMetadata
    ;


/* ============================================================================
 * 3. TRUST PROPERTIES
 * ========================================================================== */

trustProperty
    : qualifiedName
      (ASSIGN expression)?
      SEMI
    ;


trustMetadata
    : identifier
      (COLON typeExpression)?
      (ASSIGN expression)?
      SEMI
    ;


/* ============================================================================
 * 4. TRUST CONDITIONS
 * ========================================================================== */

/*
 * Conditions constrain when a declared trust relationship is applicable.
 *
 * This is declarative syntax only.
 */
trustCondition
    : WHEN
      expression
      SEMI?
    ;


/*
 * Explicit conditional trust form.
 *
 * Example:
 *
 *     if <expression>
 */
trustConditionClause
    : IF
      expression
    ;


/* ============================================================================
 * 5. TRUST SCOPE
 * ========================================================================== */

/*
 * Scope identifies the abstract semantic area in which trust applies.
 *
 * Scope does not identify a physical machine or fixed deployment topology.
 */
trustScope
    : IN
      trustReferenceList
      SEMI?
    ;


trustReferenceList
    : trustReference
      (COMMA trustReference)*
      COMMA?
    ;


/* ============================================================================
 * 6. TRUST EVIDENCE
 * ========================================================================== */

/*
 * Evidence is referenced abstractly.
 *
 * The grammar does not validate certificates, signatures, measurements,
 * credentials, attestations, logs, or other evidence.
 */
trustEvidence
    : WITH
      EVIDENCE
      trustEvidenceList
      SEMI?
    ;


trustEvidenceList
    : trustEvidenceReference
      (COMMA trustEvidenceReference)*
      COMMA?
    ;


trustEvidenceReference
    : trustReference
    | trustEvidenceCall
    ;


trustEvidenceCall
    : qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


/* ============================================================================
 * 7. TRUST VALIDITY
 * ========================================================================== */

/*
 * Validity is expressed as an abstract expression.
 *
 * The grammar does not impose a particular clock, timestamp representation,
 * calendar, duration system, or deployment environment.
 */
trustValidity
    : VALID
      trustValidityExpression
      SEMI?
    ;


trustValidityExpression
    : expression
    ;


/* ============================================================================
 * 8. TRUST REQUIREMENTS
 * ========================================================================== */

/*
 * A trust requirement expresses a condition that a downstream semantic
 * analysis must establish or satisfy.
 */
trustRequirementDeclaration
    : attributes?
      visibility?
      TRUST
      REQUIREMENT
      identifier?
      trustRequirement
      SEMI?
    ;


trustRequirement
    : REQUIRES
      trustRequirementExpression
    ;


trustRequirementExpression
    : trustRequirementAll
    | trustRequirementAny
    | trustRequirementNot
    | trustRequirementAtom
    ;


trustRequirementAll
    : LBRACE
      trustRequirementExpression
      (COMMA trustRequirementExpression)*
      COMMA?
      RBRACE
    ;


trustRequirementAny
    : LPAREN
      trustRequirementExpression
      (OR trustRequirementExpression)+
      RPAREN
    ;


trustRequirementNot
    : NOT
      trustRequirementExpression
    ;


trustRequirementAtom
    : trustReference
    | trustCall
    | trustPropertyExpression
    | expression
    ;


trustPropertyExpression
    : trustReference
      (IS | COLON)
      trustValue
    ;


trustValue
    : trustReference
    | literal
    | expression
    ;


trustCall
    : qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


/* ============================================================================
 * 9. TRUST PREFERENCES
 * ========================================================================== */

/*
 * A preference is weaker than a requirement.
 *
 * It expresses a desirable trust property without making it intrinsically
 * mandatory.
 */
trustPreferenceDeclaration
    : attributes?
      visibility?
      TRUST
      PREFERENCE
      identifier?
      trustPreferenceExpression
      SEMI?
    ;


trustPreference
    : PREFER
      trustPreferenceExpression
    ;


trustPreferenceExpression
    : trustRequirementExpression
    ;


/* ============================================================================
 * 10. TRUST ASSERTIONS
 * ========================================================================== */

/*
 * An assertion records source-level intent that a trust relationship or
 * property is asserted.
 *
 * It does not verify the assertion.
 */
trustAssertionDeclaration
    : attributes?
      visibility?
      TRUST
      ASSERTION
      identifier?
      trustAssertionTarget
      trustAssertionBody?
      SEMI?
    ;


trustAssertionTarget
    : FOR
      trustReference
    ;


trustAssertionBody
    : LBRACE
      trustAssertionMember*
      RBRACE
    ;


trustAssertionMember
    : trustProperty
    | trustCondition
    | trustEvidence
    | trustValidity
    | trustMetadata
    ;


/* ============================================================================
 * 11. TRUST REFERENCES
 * ========================================================================== */

/*
 * Trust references are intentionally open-world.
 *
 * They can refer to:
 *
 *     identity
 *     principal
 *     service
 *     domain
 *     capability
 *     authority
 *     execution context
 *     hardware abstraction
 *     future trust entities
 *
 * The semantic layer resolves their meaning.
 */
trustReference
    : qualifiedName
    ;


trustReferenceListExpression
    : trustReference
      (COMMA trustReference)*
    ;


/* ============================================================================
 * 12. TRUST RELATIONSHIP QUALIFIERS
 * ========================================================================== */

/*
 * Optional semantic qualifiers are represented as open-world names rather
 * than a closed enumeration.
 */
trustQualifier
    : qualifiedName
    ;


trustQualifierList
    : trustQualifier
      (COMMA trustQualifier)*
      COMMA?
    ;


trustQualifierClause
    : WITH
      trustQualifierList
    ;


/* ============================================================================
 * 13. TRUST MEMBER FORMS
 * ========================================================================== */

/*
 * Allows a trust relationship body to express a requirement without creating
 * a second security requirement language.
 */
trustRequirementMember
    : trustRequirement
    ;


trustPreferenceMember
    : trustPreference
    ;


/* ============================================================================
 * 14. TRUST COMPOSITION
 * ========================================================================== */

/*
 * A trust composition references other trust declarations.
 */
trustComposition
    : USING
      trustReferenceList
    ;


trustCompositionClause
    : trustComposition
    ;


/* ============================================================================
 * 15. TRUST POLICY-FACING INTENT
 * ========================================================================== */

/*
 * These rules allow security policy layers to refer to trust without
 * redefining trust declarations.
 */
trustConditionReference
    : TRUST
      trustReference
    ;


trustRequirementReference
    : TRUST
      trustReference
    ;


/* ============================================================================
 * 16. OPTIONAL TRUST ACTION INTENT
 * ========================================================================== */

/*
 * Trust may be consumed by downstream policy systems that need an abstract
 * operation over a trust relationship.
 *
 * This rule does NOT implement the operation.
 */
trustAction
    : qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


/* ============================================================================
 * 17. GENERAL TRUST PROPERTY
 * ========================================================================== */

trustPropertyClause
    : WITH
      trustPropertyList
    ;


trustPropertyList
    : trustPropertyItem
      (COMMA trustPropertyItem)*
      COMMA?
    ;


trustPropertyItem
    : qualifiedName
    | qualifiedName
      ASSIGN
      expression
    ;


/* ============================================================================
 * 18. TRUST EVALUATION INTENT
 * ========================================================================== */

/*
 * Evaluation is an INTENT, not evaluation itself.
 *
 * The semantic/runtime layers decide how trust is evaluated.
 */
trustEvaluation
    : EVALUATE
      trustReference
      (WITH trustEvaluationOptions)?
    ;


trustEvaluationOptions
    : trustPropertyList
    ;


trustEvaluationClause
    : trustEvaluation
    ;


/* ============================================================================
 * 19. TRUST STATUS REFERENCES
 * ========================================================================== */

/*
 * Status values remain open-world expressions rather than hard-coded semantic
 * states. This permits future trust models.
 */
trustStatus
    : expression
    ;


trustStatusClause
    : AS
      trustStatus
    ;


/* ============================================================================
 * 20. TRUST RELATIONSHIP EXTENSION
 * ========================================================================== */

/*
 * Existing trust declarations may be semantically extended by another
 * declaration. The meaning is resolved downstream.
 */
trustExtension
    : EXTENDS
      trustReferenceList
    ;


trustExtensionClause
    : trustExtension
    ;


/* ============================================================================
 * 21. TRUST DECLARATION OPTIONS
 * ========================================================================== */

trustDeclarationOption
    : trustScope
    | trustEvidence
    | trustValidity
    | trustQualifierClause
    | trustCompositionClause
    | trustExtensionClause
    | trustStatusClause
    | trustPropertyClause
    ;


trustDeclarationOptionList
    : trustDeclarationOption*
    ;