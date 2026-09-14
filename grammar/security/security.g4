/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/security/security.g4
 *
 * Role:
 *     Canonical parser-level grammar for Zamani source-level security
 *     declarations and security intent.
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
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     Core / domain parser
 *       |
 *       +--> THIS GRAMMAR
 *       |
 *       v
 *     frontend AST
 *       |
 *       +--> name resolution
 *       +--> type analysis
 *       +--> effect analysis
 *       +--> capability analysis
 *       +--> security analysis
 *       +--> resource analysis
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL / hardware IR
 *       +--> security metadata
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing / scheduling / resilience / ZQN
 *       |
 *       v
 *     target lowering
 *       |
 *       v
 *     runtime / hardware
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - source-level security syntax;
 *   - security declarations;
 *   - abstract principals;
 *   - capabilities;
 *   - permissions;
 *   - policies;
 *   - policy rules;
 *   - trust declarations;
 *   - classifications;
 *   - security requirements;
 *   - security preferences;
 *   - audit declarations;
 *   - authentication intent;
 *   - authorization intent;
 *   - cryptographic intent;
 *   - security bindings;
 *   - security annotations;
 *   - security composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - authentication;
 *   - authorization execution;
 *   - identity verification;
 *   - cryptographic implementation;
 *   - key generation;
 *   - key storage;
 *   - encryption/decryption;
 *   - signature generation/verification;
 *   - TLS/network implementation;
 *   - hardware security;
 *   - enclave implementation;
 *   - secure-memory implementation;
 *   - access-control enforcement;
 *   - policy evaluation;
 *   - credential storage;
 *   - quantum IR;
 *   - QEC;
 *   - ZQN;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - backend selection.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Security syntax expresses portable security intent.
 *
 * It MUST NOT encode a temporary physical implementation as permanent
 * language semantics.
 *
 * For example:
 *
 *     requires security::confidentiality;
 *
 * does not mean:
 *
 *     use algorithm X;
 *     use provider Y;
 *     use device Z;
 *     use N nodes;
 *     use N qubits;
 *     use CPU X;
 *     use GPU Y;
 *     use QPU Z.
 *
 * Those decisions belong downstream to capability negotiation, compilation,
 * deployment and runtime layers.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are intentionally no finite grammar-level limits on:
 *
 *   - principals;
 *   - domains;
 *   - permissions;
 *   - capabilities;
 *   - policy rules;
 *   - policy nesting;
 *   - trust relationships;
 *   - classifications;
 *   - requirements;
 *   - audit requirements;
 *   - cryptographic operations;
 *   - policy metadata;
 *   - declaration count.
 *
 * Repetition is represented with ANTLR repetition operators.
 *
 * Physical/resource limits are NOT language limits.
 *
 * ============================================================================
 * OPEN-WORLD SECURITY
 * ============================================================================
 *
 * Security algorithms, mechanisms, providers, identity systems, trust
 * anchors, hardware facilities and future security mechanisms are represented
 * by qualified names and expressions.
 *
 * The grammar deliberately does not enumerate:
 *
 *   AES
 *   RSA
 *   ML-KEM
 *   ML-DSA
 *   TLS
 *   TPM
 *   HSM
 *   SGX
 *   SEV
 *   TrustZone
 *
 * as semantic language primitives.
 *
 * New mechanisms therefore do not require a grammar redesign.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Security syntax may apply to quantum programs.
 *
 * This grammar MUST NOT define:
 *
 *   QubitId
 *   PhysicalQubitId
 *   GateKind
 *   topology
 *   calibration
 *   QEC codes
 *   ZQN fault models
 *   backend selection
 *
 * Quantum semantics remain downstream and the canonical quantum semantic
 * boundary remains quantum::ir.
 *
 * ============================================================================
 * HARDWARE BOUNDARY
 * ============================================================================
 *
 * A security capability such as:
 *
 *     security::trusted_execution
 *
 * expresses an abstract requirement.
 *
 * It does not identify a CPU, GPU, FPGA, QPU, enclave, memory region,
 * machine or provider.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser preserves syntax.
 *
 * AST/semantic analysis constructs concepts such as:
 *
 *   SecurityDomain
 *   Principal
 *   Capability
 *   Permission
 *   Policy
 *   PolicyRule
 *   TrustRelationship
 *   Classification
 *   SecurityRequirement
 *   AuditRequirement
 *   SecurityBinding
 *
 * The parser itself MUST NOT instantiate semantic security objects.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *   - no actions;
 *   - no predicates;
 *   - no I/O;
 *   - no randomness;
 *   - no hardware discovery;
 *   - no runtime state;
 *   - no policy evaluation.
 *
 * Therefore parsing is deterministic for a deterministic token stream.
 *
 * ============================================================================
 * LEXER INTEGRATION CONTRACT
 * ============================================================================
 *
 * The canonical lexer must provide the security vocabulary used below.
 *
 * Required canonical tokens:
 *
 *   SECURITY
 *   DOMAIN
 *   PRINCIPAL
 *   CAPABILITY
 *   PERMISSION
 *   POLICY
 *   TRUST
 *   CLASSIFICATION
 *   REQUIREMENT
 *   AUDIT
 *   AUTHENTICATION
 *   AUTHORIZATION
 *   CRYPTOGRAPHY
 *   ENCRYPT
 *   DECRYPT
 *   SIGN
 *   VERIFY
 *   CONFIDENTIALITY
 *   INTEGRITY
 *   PRIVACY
 *   AVAILABILITY
 *   IDENTITY
 *   CREDENTIAL
 *   GRANT
 *   DENY
 *   ALLOW
 *   REJECT
 *   REVOKE
 *   PREFER
 *   ON
 *   TO
 *   FOR
 *   WHEN
 *   FROM
 *   WITH
 *   USING
 *   AS
 *   IN
 *   BY
 *   IF
 *   THEN
 *   ELSE
 *   NOT
 *   AND
 *   OR
 *   PUBLIC
 *   PRIVATE
 *   ISOLATED
 *   TRUSTED
 *   UNTRUSTED
 *   OPTIONAL
 *   MANDATORY
 *
 * Existing shared tokens such as IDENTIFIER, STRING, INTEGER, AT, COLON,
 * ASSIGN, COMMA, SEMI, LPAREN, RPAREN, LBRACE, RBRACE and DOUBLE_COLON
 * remain owned by ZamaniLexer.
 *
 * The lexer remains the single owner of token spelling.
 *
 * ============================================================================
 */

parser grammar Security;

options {
    tokenVocab = ZamaniLexer;
}

import Core, Types, Expressions, Effects;


/* ============================================================================
 * 1. SECURITY ROOT
 * ========================================================================== */

securityDeclaration
    : securityDomainDeclaration
    | securityPrincipalDeclaration
    | securityCapabilityDeclaration
    | securityPermissionDeclaration
    | securityPolicyDeclaration
    | securityTrustDeclaration
    | securityClassificationDeclaration
    | securityRequirementDeclaration
    | securityAuditDeclaration
    | securityBindingDeclaration
    ;


/* ============================================================================
 * 2. SECURITY DOMAIN
 * ========================================================================== */

securityDomainDeclaration
    : attributes*
      visibility?
      SECURITY
      DOMAIN
      identifier
      genericParameters?
      securityDomainBody?
      SEMI?
    ;

securityDomainBody
    : LBRACE
      securityDomainMember*
      RBRACE
    ;

securityDomainMember
    : securityPrincipalDeclaration
    | securityCapabilityDeclaration
    | securityPermissionDeclaration
    | securityPolicyDeclaration
    | securityTrustDeclaration
    | securityClassificationDeclaration
    | securityRequirementDeclaration
    | securityAuditDeclaration
    | securityBindingDeclaration
    ;


/* ============================================================================
 * 3. PRINCIPALS
 * ========================================================================== */

securityPrincipalDeclaration
    : attributes*
      visibility?
      PRINCIPAL
      identifier
      principalTypeClause?
      genericParameters?
      principalBody?
      SEMI?
    ;

principalTypeClause
    : COLON
      typeExpression
    ;

principalBody
    : LBRACE
      principalMember*
      RBRACE
    ;

principalMember
    : principalProperty
    | securityCapabilityReference
    | securityPermissionReference
    | securityTrustReference
    ;

principalProperty
    : identifier
      (COLON typeExpression)?
      (ASSIGN expression)?
      SEMI
    ;


/* ============================================================================
 * 4. CAPABILITIES
 * ========================================================================== */

securityCapabilityDeclaration
    : attributes*
      visibility?
      CAPABILITY
      identifier
      genericParameters?
      capabilityBody?
      SEMI?
    ;

capabilityBody
    : LBRACE
      capabilityMember*
      RBRACE
    ;

capabilityMember
    : capabilityProperty
    | securityPermissionReference
    | securityRequirement
    ;

capabilityProperty
    : identifier
      (COLON typeExpression)?
      (ASSIGN expression)?
      SEMI
    ;

securityCapabilityReference
    : CAPABILITY
      qualifiedName
    ;


/* ============================================================================
 * 5. PERMISSIONS
 * ========================================================================== */

securityPermissionDeclaration
    : attributes*
      visibility?
      PERMISSION
      identifier
      genericParameters?
      permissionBody?
      SEMI?
    ;

permissionBody
    : LBRACE
      permissionMember*
      RBRACE
    ;

permissionMember
    : permissionProperty
    | securityRequirement
    ;

permissionProperty
    : identifier
      (COLON typeExpression)?
      (ASSIGN expression)?
      SEMI
    ;

securityPermissionReference
    : PERMISSION
      qualifiedName
    ;


/* ============================================================================
 * 6. POLICY DECLARATIONS
 * ========================================================================== */

securityPolicyDeclaration
    : attributes*
      visibility?
      POLICY
      identifier
      genericParameters?
      policyTargetClause?
      policyBody?
      SEMI?
    ;

policyTargetClause
    : FOR
      securityReference
    ;

policyBody
    : LBRACE
      securityPolicyMember*
      RBRACE
    ;

securityPolicyMember
    : securityPolicyRule
    | securityRequirement
    | securityPreference
    | securityAuditRequirement
    | securityPolicyProperty
    ;

securityPolicyRule
    : policyCondition?
      policyDecision
      policyAction?
      SEMI?
    ;

policyCondition
    : WHEN
      expression
    ;

policyDecision
    : GRANT
    | DENY
    | ALLOW
    | REJECT
    | REQUIREMENT
    ;

policyAction
    : ON
      securityReference
    ;

securityPolicyProperty
    : identifier
      (COLON typeExpression)?
      (ASSIGN expression)?
      SEMI
    ;


/* ============================================================================
 * 7. SECURITY REFERENCES
 * ========================================================================== */

securityReference
    : qualifiedName
    ;

securityReferenceList
    : securityReference
      (COMMA securityReference)*
      COMMA?
    ;


/* ============================================================================
 * 8. REQUIREMENTS
 * ========================================================================== */

securityRequirementDeclaration
    : attributes*
      visibility?
      SECURITY
      REQUIREMENT
      identifier?
      securityRequirement
      SEMI?
    ;

securityRequirement
    : REQUIRES
      securityRequirementExpression
    ;

securityRequirementExpression
    : securityRequirementAll
    | securityRequirementAny
    | securityRequirementNot
    | securityRequirementAtom
    ;

securityRequirementAll
    : LBRACE
      securityRequirementExpression
      (COMMA securityRequirementExpression)*
      COMMA?
      RBRACE
    ;

securityRequirementAny
    : LPAREN
      securityRequirementExpression
      (OR securityRequirementExpression)+
      RPAREN
    ;

securityRequirementNot
    : NOT
      securityRequirementExpression
    ;

securityRequirementAtom
    : securityRequirementReference
    | securityRequirementCall
    | securityPropertyExpression
    | expression
    ;

securityRequirementReference
    : securityReference
    ;

securityRequirementCall
    : qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;

securityPropertyExpression
    : securityReference
      (IS | COLON)
      securityPropertyValue
    ;

securityPropertyValue
    : securityReference
    | literal
    | expression
    ;


/* ============================================================================
 * 9. PREFERENCES
 * ========================================================================== */

securityPreference
    : PREFER
      securityRequirementExpression
    ;


/* ============================================================================
 * 10. TRUST
 * ========================================================================== */

securityTrustDeclaration
    : attributes*
      visibility?
      TRUST
      identifier?
      trustSourceClause
      trustTargetClause
      trustPropertyClause*
      SEMI?
    ;

trustSourceClause
    : FROM
      securityReference
    ;

trustTargetClause
    : TO
      securityReference
    ;

trustPropertyClause
    : WITH
      securityTrustPropertyList
    ;

securityTrustPropertyList
    : securityTrustProperty
      (COMMA securityTrustProperty)*
      COMMA?
    ;

securityTrustProperty
    : qualifiedName
    | qualifiedName
      ASSIGN
      expression
    ;

securityTrustReference
    : TRUST
      securityReference
    ;


/* ============================================================================
 * 11. CLASSIFICATION
 * ========================================================================== */

securityClassificationDeclaration
    : attributes*
      visibility?
      CLASSIFICATION
      identifier
      classificationValue?
      classificationBody?
      SEMI?
    ;

classificationValue
    : ASSIGN
      expression
    ;

classificationBody
    : LBRACE
      classificationMember*
      RBRACE
    ;

classificationMember
    : classificationLabel
    | classificationProperty
    | securityRequirement
    ;

classificationLabel
    : identifier
      (ASSIGN expression)?
      SEMI?
    ;

classificationProperty
    : identifier
      COLON
      expression
      SEMI?
    ;


/* ============================================================================
 * 12. AUDIT
 * ========================================================================== */

securityAuditDeclaration
    : attributes*
      visibility?
      AUDIT
      identifier?
      auditBody?
      SEMI?
    ;

auditBody
    : LBRACE
      auditMember*
      RBRACE
    ;

auditMember
    : auditRequirement
    | auditProperty
    | auditEvent
    ;

securityAuditRequirement
    : AUDIT
      securityAuditRequirementExpression
      SEMI?
    ;

auditRequirement
    : securityAuditRequirement
    ;

securityAuditRequirementExpression
    : securityReference
    | securityRequirementExpression
    | expression
    ;

auditProperty
    : identifier
      (COLON typeExpression)?
      (ASSIGN expression)?
      SEMI
    ;

auditEvent
    : identifier
      (LPAREN argumentList? RPAREN)?
      SEMI?
    ;


/* ============================================================================
 * 13. SECURITY BINDINGS
 *
 * A binding attaches security intent to a source-level subject.
 *
 * It does not force a target implementation.
 * ========================================================================== */

securityBindingDeclaration
    : attributes*
      visibility?
      SECURITY
      identifier
      securityBindingTarget
      securityBindingClause*
      SEMI?
    ;

securityBindingTarget
    : ON
      securityReference
    ;

securityBindingClause
    : securityRequirement
    | securityPreference
    | securityPropertyClause
    | securityAuditRequirement
    ;

securityPropertyClause
    : WITH
      securityPropertyList
    ;

securityPropertyList
    : securityProperty
      (COMMA securityProperty)*
      COMMA?
    ;

securityProperty
    : qualifiedName
      (ASSIGN expression)?
    ;


/* ============================================================================
 * 14. AUTHENTICATION
 * ========================================================================== */

authenticationIntent
    : AUTHENTICATION
      authenticationTarget?
      authenticationClause*
    ;

authenticationTarget
    : FOR
      securityReference
    ;

authenticationClause
    : USING
      securityReferenceList
    | WITH
      securityPropertyList
    | AS
      securityReference
    | IF
      expression
    ;


/* ============================================================================
 * 15. AUTHORIZATION
 * ========================================================================== */

authorizationIntent
    : AUTHORIZATION
      authorizationAction
      authorizationTarget?
      authorizationClause*
    ;

authorizationAction
    : GRANT
    | DENY
    | ALLOW
    | REJECT
    | REVOKE
    ;

authorizationTarget
    : TO
      securityReference
    ;

authorizationClause
    : WHEN
      expression
    | WITH
      securityPropertyList
    | FOR
      securityReference
    ;


/* ============================================================================
 * 16. CRYPTOGRAPHIC INTENT
 *
 * Algorithm and provider names remain open-world qualified names.
 * ========================================================================== */

cryptographicIntent
    : CRYPTOGRAPHY
      cryptographicOperation
      cryptographicTarget?
      cryptographicClause*
    ;

cryptographicOperation
    : ENCRYPT
    | DECRYPT
    | SIGN
    | VERIFY
    ;

cryptographicTarget
    : ON
      securityReference
    ;

cryptographicClause
    : USING
      securityReferenceList
    | WITH
      securityPropertyList
    | AS
      securityReference
    | FOR
      securityReference
    ;


/* ============================================================================
 * 17. SECURITY PROPERTY EXPRESSIONS
 * ========================================================================== */

securityPropertyDeclaration
    : attributes*
      visibility?
      SECURITY
      identifier
      ASSIGN
      securityPropertyExpression
      SEMI?
    ;

securityProperty
    : securityReference
    | securityPropertyExpression
    ;

securityPropertySet
    : LBRACE
      securityProperty
      (COMMA securityProperty)*
      COMMA?
      RBRACE
    ;

securityPropertyExpression
    : securityPropertyAtom
      (AND securityPropertyAtom)*
    ;

securityPropertyAtom
    : securityReference
    | securityPropertySet
    | LPAREN securityPropertyExpression RPAREN
    | securityPropertyComparison
    ;

securityPropertyComparison
    : securityReference
      comparisonOperator
      expression
    ;

comparisonOperator
    : EQ
    | NEQ
    | LT
    | LE
    | GT
    | GE
    ;


/* ============================================================================
 * 18. STANDARD SECURITY DIMENSIONS
 *
 * These are syntax-level names, not implementations.
 * ========================================================================== */

securityDimension
    : CONFIDENTIALITY
    | INTEGRITY
    | PRIVACY
    | AVAILABILITY
    ;

securityDimensionRequirement
    : REQUIRES
      securityDimension
    ;


/* ============================================================================
 * 19. IDENTITY / CREDENTIAL INTENT
 * ========================================================================== */

identityReference
    : IDENTITY
      securityReference
    ;

credentialReference
    : CREDENTIAL
      securityReference
    ;


/* ============================================================================
 * 20. SECURITY COMPOSITION
 *
 * These rules permit security constructs to be attached to source-level
 * declarations without creating a second declaration system.
 * ========================================================================== */

securityClause
    : securityRequirement
    | securityPreference
    | authenticationIntent
    | authorizationIntent
    | cryptographicIntent
    | securityAuditRequirement
    ;

securityClauseList
    : securityClause+
    ;


/* ============================================================================
 * 21. SECURITY EFFECT REFERENCES
 *
 * Effect ownership remains with grammar/effects/effects.g4.
 * ========================================================================== */

securityEffectReference
    : qualifiedName
    ;

securityEffectList
    : securityEffectReference
      (COMMA securityEffectReference)*
      COMMA?
    ;


/* ============================================================================
 * 22. SECURITY-QUALIFIED TARGETS
 *
 * This permits security to refer to arbitrary source-level entities without
 * introducing machine-specific identifiers.
 * ========================================================================== */

securityTarget
    : securityReference
    ;

securityTargetList
    : securityTarget
      (COMMA securityTarget)*
      COMMA?
    ;


/* ============================================================================
 * 23. GENERAL SECURITY SPECIFICATION
 *
 * Useful as an attachment point for future security constructs while keeping
 * the grammar open-world.
 * ========================================================================== */

securitySpecification
    : securityClauseList
    | securityPropertyExpression
    | securityPropertySet
    ;


/* ============================================================================
 * 24. DOMAIN-AGNOSTIC SECURITY ATTACHMENT
 *
 * Security may protect classical, quantum, HDL, distributed, AI, networking
 * and future computation. No domain-specific resource representation is
 * created here.
 * ========================================================================== */

securityAttachment
    : SECURITY
      ON
      securityTarget
      securitySpecification
      SEMI?
    ;


/* ============================================================================
 * END
 * ============================================================================
 */