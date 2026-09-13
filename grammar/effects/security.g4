/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/effects/security.g4
 *
 * Role:
 *     Canonical parser-level grammar for source-level security declarations,
 *     security requirements, security policies, authorization expressions,
 *     trust relationships, audit intent, data-classification intent, and
 *     security-sensitive effect composition.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Runtime/compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, semantic predicates,
 *     filesystem access, network access, runtime calls, or unsafe code.
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
 *     parser
 *       |
 *       +--> core names
 *       +--> types
 *       +--> expressions
 *       +--> effects
 *       +--> THIS FILE
 *       |
 *       v
 *     frontend AST
 *       |
 *       +--> name resolution
 *       +--> type checking
 *       +--> effect analysis
 *       +--> capability analysis
 *       +--> security analysis
 *       +--> policy analysis
 *       +--> resource analysis
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware representation
 *       +--> security/effect metadata
 *       |
 *       v
 *     optimization / routing / scheduling / resilience
 *       |
 *       v
 *     target lowering
 *       |
 *       v
 *     runtime / hardware
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns SOURCE SYNTAX only.
 *
 * It provides syntax for expressing:
 *
 *     - security domains;
 *     - principals;
 *     - capabilities;
 *     - permissions;
 *     - security policies;
 *     - policy rules;
 *     - grants;
 *     - denials;
 *     - revocations;
 *     - authentication intent;
 *     - authorization intent;
 *     - trust relationships;
 *     - audit requirements;
 *     - data-classification labels;
 *     - integrity requirements;
 *     - confidentiality requirements;
 *     - privacy requirements;
 *     - cryptographic operation intent;
 *     - security-sensitive effects;
 *     - security requirements on declarations;
 *     - security constraints;
 *     - security preferences;
 *     - security annotations;
 *     - policy composition.
 *
 * This file does NOT implement security.
 *
 * ============================================================================
 * OPEN-WORLD SECURITY MODEL
 * ============================================================================
 *
 * Security concepts are intentionally open-world.
 *
 * The grammar MUST NOT enumerate a closed list such as:
 *
 *     AES
 *     RSA
 *     ML-KEM
 *     TLS
 *     device_1
 *     user_1
 *     admin_1
 *
 * as language semantics.
 *
 * Algorithm names, identity names, policy names, authority names, provider
 * names, hardware names, cryptographic-suite names, trust anchors, and
 * security-domain names remain source-level names.
 *
 * Their meaning is determined downstream.
 *
 * This permits future security mechanisms without changing the grammar.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Security syntax describes WHAT security properties are required or what
 * security-sensitive behavior is requested.
 *
 * It does not permanently encode WHERE or HOW those requirements are
 * implemented.
 *
 * For example:
 *
 *     requires security::confidentiality;
 *
 * does NOT mean:
 *
 *     use algorithm X
 *     use device Y
 *     use provider Z
 *     use exactly N bits
 *     use exactly N nodes
 *
 * Target-specific realization belongs to semantic analysis, capability
 * negotiation, compilation, deployment, and runtime layers.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar imposes no language-level finite limits on:
 *
 *     - principals;
 *     - permissions;
 *     - capabilities;
 *     - policy rules;
 *     - policy expressions;
 *     - trust relationships;
 *     - security domains;
 *     - classification labels;
 *     - audit requirements;
 *     - cryptographic operations;
 *     - policy nesting;
 *     - generic parameters;
 *     - declaration count.
 *
 * Repetition uses grammar repetition operators rather than fixed bounds.
 *
 * Practical limits are implementation/resource-policy concerns and MUST NOT
 * become source-language semantics.
 *
 * ============================================================================
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * This grammar does not:
 *
 *     - authenticate anyone;
 *     - authorize an operation;
 *     - verify a signature;
 *     - decrypt data;
 *     - generate keys;
 *     - select cryptographic algorithms;
 *     - access credentials;
 *     - contact identity providers;
 *     - access files;
 *     - access networks;
 *     - inspect hardware;
 *     - discover devices;
 *     - access secure enclaves;
 *     - enforce policies;
 *     - perform runtime security checks.
 *
 * All of those are downstream responsibilities.
 *
 * ============================================================================
 * EFFECT BOUNDARY
 * ============================================================================
 *
 * Security-sensitive computation may be represented as effects.
 *
 * Examples:
 *
 *     security::authenticate
 *     security::authorize
 *     security::encrypt
 *     security::decrypt
 *     security::sign
 *     security::verify
 *     security::audit
 *
 * These remain names.
 *
 * This grammar does not redefine the effect system.
 *
 * The canonical effect system remains:
 *
 *     grammar/effects/effects.g4
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Security may apply to quantum computation.
 *
 * Examples include:
 *
 *     secure quantum execution
 *     protected quantum data
 *     authenticated control
 *     integrity requirements
 *     confidential quantum-classical communication
 *
 * However, this grammar MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     topology
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
 * Security requirements may refer to abstract capabilities.
 *
 * For example:
 *
 *     requires security::trusted_execution;
 *     requires security::isolated_memory;
 *
 * The grammar MUST NOT interpret those as:
 *
 *     use CPU X
 *     use GPU Y
 *     use FPGA Z
 *     use QPU N
 *
 * Hardware realization belongs to the hardware capability and target layers.
 *
 * ============================================================================
 * DATA BOUNDARY
 * ============================================================================
 *
 * Data-classification syntax expresses semantic intent.
 *
 * It does not define a storage format, memory address, database, filesystem,
 * network location, or physical storage medium.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve:
 *
 *     - source spelling;
 *     - declaration kind;
 *     - name structure;
 *     - qualified-name segments;
 *     - expression structure;
 *     - source ordering;
 *     - source spans.
 *
 * Semantic analysis is responsible for constructing:
 *
 *     - SecurityDomain
 *     - Principal
 *     - CapabilityRequirement
 *     - Permission
 *     - Policy
 *     - PolicyRule
 *     - TrustRelationship
 *     - Classification
 *     - SecurityRequirement
 *     - AuditRequirement
 *
 * The parser MUST NOT construct those semantic objects itself.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded actions;
 *     - no semantic predicates;
 *     - no I/O;
 *     - no random operations;
 *     - no runtime-dependent decisions;
 *     - no hardware discovery;
 *     - no policy evaluation.
 *
 * The same token stream must therefore produce the same parse structure.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * The grammar consumes canonical shared syntax from:
 *
 *     Core
 *     Types
 *     Expressions
 *
 * and composes with:
 *
 *     Effects
 *
 * Canonical shared rules MUST NOT be redefined here.
 *
 * In particular, this file reuses:
 *
 *     identifier
 *     qualifiedName
 *     attributes
 *     visibility
 *     genericParameters
 *     parameterList
 *     returnType
 *     whereClause
 *     typeExpression
 *     expression
 *     argumentList
 *     blockExpression
 *
 * ============================================================================
 */

parser grammar Security;

options {
    tokenVocab = ZamaniLexer;
}

import Core, Types, Expressions, Effects;


/*
 * ============================================================================
 * 1. SECURITY DECLARATIONS
 * ============================================================================
 *
 * A security declaration introduces source-level security intent.
 *
 * Security declarations remain open-world.
 */

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


/*
 * ============================================================================
 * 2. SECURITY DOMAIN
 * ============================================================================
 *
 * Examples:
 *
 *     security domain application;
 *     security domain application {
 *         ...
 *     }
 *
 * A domain is a semantic namespace/context, not a physical security device.
 */

securityDomainDeclaration
    : attributes?
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
    ;


/*
 * ============================================================================
 * 3. PRINCIPALS
 * ============================================================================
 *
 * A principal represents an abstract security identity.
 *
 * It does not itself authenticate the identity.
 */

securityPrincipalDeclaration
    : attributes?
      visibility?
      PRINCIPAL
      identifier
      principalTypeClause?
      principalBody?
      SEMI?
    ;


principalTypeClause
    : COLON
      qualifiedName
    ;


principalBody
    : LBRACE
      principalMember*
      RBRACE
    ;


principalMember
    : principalAttribute
    | securityCapabilityReference
    | securityPermissionReference
    | securityTrustReference
    ;


principalAttribute
    : identifier
      (ASSIGN expression)?
      SEMI
    ;


/*
 * ============================================================================
 * 4. CAPABILITIES
 * ============================================================================
 *
 * A capability describes an abstract authority or security property that an
 * execution environment or principal may possess.
 *
 * It does NOT perform capability discovery.
 */

securityCapabilityDeclaration
    : attributes?
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
    : capabilityAttribute
    | securityPermissionReference
    | securityRequirement
    ;


capabilityAttribute
    : identifier
      (ASSIGN expression)?
      SEMI
    ;


securityCapabilityReference
    : CAPABILITY
      qualifiedName
    ;


/*
 * ============================================================================
 * 5. PERMISSIONS
 * ============================================================================
 *
 * A permission names an abstract authority.
 *
 * Permission meaning is determined semantically.
 */

securityPermissionDeclaration
    : attributes?
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
    : permissionAttribute
    | securityRequirement
    ;


permissionAttribute
    : identifier
      (ASSIGN expression)?
      SEMI
    ;


securityPermissionReference
    : PERMISSION
      qualifiedName
    ;


/*
 * ============================================================================
 * 6. POLICIES
 * ============================================================================
 *
 * A policy is a source-level declaration of security rules.
 *
 * Policy evaluation is NOT performed by the parser.
 */

securityPolicyDeclaration
    : attributes?
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
    | securityPolicyMetadata
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
    | REQUIRE
    | REJECT
    | ALLOW
    ;


policyAction
    : ON
      securityReference
    ;


securityPolicyMetadata
    : identifier
      (ASSIGN expression)?
      SEMI
    ;


/*
 * ============================================================================
 * 7. SECURITY REFERENCES
 * ============================================================================
 *
 * A security reference is an open-world qualified name.
 */

securityReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 8. REQUIREMENTS
 * ============================================================================
 *
 * Security requirements express semantic obligations.
 *
 * They do not directly select implementations.
 */

securityRequirementDeclaration
    : attributes?
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
    : securityReference
    | securityRequirementSet
    | securityRequirementCall
    | expression
    ;


securityRequirementSet
    : LBRACE
      securityRequirementItem*
      RBRACE
    ;


securityRequirementItem
    : securityRequirementExpression
      COMMA?
    ;


securityRequirementCall
    : qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 9. SECURITY PREFERENCES
 * ============================================================================
 *
 * A preference expresses a preferred valid realization.
 *
 * It is not a hard requirement.
 */

securityPreference
    : PREFER
      securityRequirementExpression
    ;


/*
 * ============================================================================
 * 10. TRUST RELATIONSHIPS
 * ============================================================================
 *
 * Trust is represented declaratively.
 *
 * The grammar does not decide whether trust is justified.
 */

securityTrustDeclaration
    : attributes?
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
    ;


securityTrustProperty
    : qualifiedName
    | qualifiedName
      ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 11. CLASSIFICATION
 * ============================================================================
 *
 * Classification is semantic metadata.
 *
 * The grammar does not impose a finite classification vocabulary.
 */

securityClassificationDeclaration
    : attributes?
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
    : classificationAttribute
    | securityRequirement
    ;


classificationAttribute
    : identifier
      (ASSIGN expression)?
      SEMI
    ;


/*
 * ============================================================================
 * 12. SECURITY LABELS
 * ============================================================================
 *
 * Labels are source-level names.
 *
 * They must not be interpreted as a closed enumeration.
 */

securityLabel
    : qualifiedName
    ;


securityLabelList
    : securityLabel
      (COMMA securityLabel)*
      COMMA?
    ;


securityLabelSet
    : LBRACE
      securityLabelList?
      RBRACE
    ;


/*
 * ============================================================================
 * 13. DATA SECURITY BINDINGS
 * ============================================================================
 *
 * A binding attaches security metadata to an expression or declaration.
 */

securityBindingDeclaration
    : attributes?
      visibility?
      SECURITY
      BIND
      securityLabelSet
      TO
      expression
      SEMI?
    ;


/*
 * ============================================================================
 * 14. AUTHENTICATION INTENT
 * ============================================================================
 *
 * Authentication is a semantic/runtime operation.
 *
 * The grammar expresses intent only.
 */

authenticationExpression
    : AUTHENTICATE
      LPAREN
      argumentList?
      RPAREN
    ;


authenticationStatement
    : authenticationExpression
      SEMI?
    ;


/*
 * ============================================================================
 * 15. AUTHORIZATION INTENT
 * ============================================================================
 *
 * Authorization asks whether an abstract principal/capability/operation
 * relationship is permitted.
 */

authorizationExpression
    : AUTHORIZE
      LPAREN
      argumentList?
      RPAREN
    ;


authorizationStatement
    : authorizationExpression
      SEMI?
    ;


/*
 * ============================================================================
 * 16. CRYPTOGRAPHIC OPERATION INTENT
 * ============================================================================
 *
 * These are operation forms, not implementations.
 *
 * Algorithm/provider/key selection remains semantic.
 */

securityCryptoExpression
    : ENCRYPT
      LPAREN
      argumentList?
      RPAREN

    | DECRYPT
      LPAREN
      argumentList?
      RPAREN

    | SIGN
      LPAREN
      argumentList?
      RPAREN

    | VERIFY
      LPAREN
      argumentList?
      RPAREN
    ;


securityCryptoStatement
    : securityCryptoExpression
      SEMI?
    ;


/*
 * ============================================================================
 * 17. REDACTION
 * ============================================================================
 *
 * Redaction expresses data-transformation intent.
 */

redactExpression
    : REDACT
      LPAREN
      argumentList?
      RPAREN
    ;


redactStatement
    : redactExpression
      SEMI?
    ;


/*
 * ============================================================================
 * 18. AUDIT
 * ============================================================================
 *
 * Audit syntax expresses observability/compliance intent.
 */

securityAuditDeclaration
    : attributes?
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
    : securityAuditRequirement
    | auditAttribute
    ;


securityAuditRequirement
    : AUDIT
      securityAuditExpression
      SEMI?
    ;


securityAuditExpression
    : securityReference
    | securityRequirementExpression
    | expression
    ;


auditAttribute
    : identifier
      (ASSIGN expression)?
      SEMI
    ;


/*
 * ============================================================================
 * 19. AUDIT STATEMENT
 * ============================================================================
 */

auditStatement
    : AUDIT
      expression?
      SEMI?
    ;


/*
 * ============================================================================
 * 20. SECURITY REQUIREMENT CLAUSE
 * ============================================================================
 *
 * This form is intended for declaration-level composition.
 *
 * Example:
 *
 *     fn transfer(...)
 *         requires security::authorization;
 *
 * It remains separate from the general function grammar so the semantic
 * frontend can distinguish ordinary preconditions from security metadata.
 */

securityRequirementClause
    : REQUIRES
      securityRequirementExpression
    ;


/*
 * ============================================================================
 * 21. SECURITY EFFECT CLAUSE
 * ============================================================================
 *
 * Security-specific effect names are ordinary qualified names.
 *
 * Example:
 *
 *     with security::authenticate,
 *          security::audit
 *
 * The actual effect-set semantics remain owned by effects.g4 and downstream
 * effect analysis.
 */

securityEffectClause
    : WITH
      EFFECTS
      securityEffectReferenceList
    ;


securityEffectReferenceList
    : securityEffectReference
      (COMMA securityEffectReference)*
      COMMA?
    ;


securityEffectReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 22. SECURITY EFFECT INVOCATION
 * ============================================================================
 *
 * This composes with the canonical effect system.
 *
 * No independent effect model is created here.
 */

securityPerformStatement
    : PERFORM
      securityEffectReference
      (
          LPAREN
          argumentList?
          RPAREN
      )?
      SEMI?
    ;


/*
 * ============================================================================
 * 23. SECURITY POLICY COMPOSITION
 * ============================================================================
 *
 * Policy composition is syntactic.
 *
 * Evaluation and conflict resolution belong to semantic policy analysis.
 */

securityPolicyComposition
    : COMPOSE
      securityReferenceList
    ;


securityReferenceList
    : securityReference
      (COMMA securityReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * 24. POLICY INHERITANCE
 * ============================================================================
 *
 * Policy inheritance does not imply implementation inheritance.
 */

securityPolicyInheritance
    : EXTENDS
      securityReferenceList
    ;


/*
 * ============================================================================
 * 25. GRANTS
 * ============================================================================
 *
 * A grant declaration describes an authorization relationship.
 */

securityGrantDeclaration
    : attributes?
      visibility?
      GRANT
      securityReference
      TO
      securityReferenceList
      grantCondition?
      SEMI?
    ;


grantCondition
    : WHEN
      expression
    ;


/*
 * ============================================================================
 * 26. DENIALS
 * ============================================================================
 */

securityDenyDeclaration
    : attributes?
      visibility?
      DENY
      securityReference
      TO
      securityReferenceList
      denyCondition?
      SEMI?
    ;


denyCondition
    : WHEN
      expression
    ;


/*
 * ============================================================================
 * 27. REVOCATIONS
 * ============================================================================
 */

securityRevokeDeclaration
    : attributes?
      visibility?
      REVOKE
      securityReference
      FROM
      securityReferenceList
      revokeCondition?
      SEMI?
    ;


revokeCondition
    : WHEN
      expression
    ;


/*
 * ============================================================================
 * 28. SECURITY OPERATIONS
 * ============================================================================
 *
 * These rules provide parser-level composition points for statement grammar.
 */

securityOperation
    : authenticationStatement
    | authorizationStatement
    | securityCryptoStatement
    | redactStatement
    | auditStatement
    | securityPerformStatement
    | securityGrantDeclaration
    | securityDenyDeclaration
    | securityRevokeDeclaration
    ;


/*
 * ============================================================================
 * 29. SECURITY ASSERTIONS
 * ============================================================================
 *
 * Assertions are semantic checks performed downstream.
 */

securityAssertion
    : ASSERT
      LPAREN
      securityRequirementExpression
      RPAREN
      SEMI?
    ;


/*
 * ============================================================================
 * 30. SECURITY CONSTRAINT
 * ============================================================================
 *
 * A constraint is not necessarily a requirement.
 *
 * Semantic analysis determines whether it is hard, soft, static, dynamic,
 * compile-time, deployment-time, or runtime.
 */

securityConstraint
    : CONSTRAIN
      securityRequirementExpression
    ;


securityConstraintList
    : securityConstraint
      (COMMA securityConstraint)*
    ;


/*
 * ============================================================================
 * 31. SECURITY CONTEXT
 * ============================================================================
 *
 * Security context groups source-level security declarations and requirements.
 */

securityContext
    : SECURITY
      CONTEXT
      identifier?
      LBRACE
      securityContextMember*
      RBRACE
    ;


securityContextMember
    : securityRequirement
    | securityConstraint
    | securityPreference
    | securityCapabilityReference
    | securityPermissionReference
    | securityTrustReference
    | securityAuditRequirement
    | securityOperation
    ;


securityTrustReference
    : TRUST
      securityReference
    ;


/*
 * ============================================================================
 * 32. SECURITY REQUIREMENT SET
 * ============================================================================
 *
 * Explicit logical composition remains expression-oriented.
 */

securityRequirementSetExpression
    : ALL
      securityRequirementSetItem+
    | ANY
      securityRequirementSetItem+
    | NOT
      securityRequirementSetItem
    ;


securityRequirementSetItem
    : LPAREN
      securityRequirementExpression
      RPAREN
    | securityRequirementExpression
    ;


/*
 * ============================================================================
 * 33. SECURITY POLICY CONDITION
 * ============================================================================
 *
 * Conditions are ordinary expressions.
 *
 * Security grammar does not create a second expression language.
 */

securityCondition
    : expression
    ;


/*
 * ============================================================================
 * 34. SECURITY PRINCIPAL SET
 * ============================================================================
 */

securityPrincipalSet
    : LBRACE
      securityReferenceList?
      RBRACE
    ;


/*
 * ============================================================================
 * 35. SECURITY CAPABILITY SET
 * ============================================================================
 */

securityCapabilitySet
    : LBRACE
      securityReferenceList?
      RBRACE
    ;


/*
 * ============================================================================
 * 36. SECURITY PERMISSION SET
 * ============================================================================
 */

securityPermissionSet
    : LBRACE
      securityReferenceList?
      RBRACE
    ;


/*
 * ============================================================================
 * 37. SECURITY AUDIT REQUIREMENT
 * ============================================================================
 */

securityAuditRequirement
    : AUDIT
      securityRequirementExpression
    ;


/*
 * ============================================================================
 * 38. SECURITY DECLARATION MEMBER
 * ============================================================================
 *
 * Aggregate grammar composition point.
 */

securityMember
    : securityDeclaration
    | securityOperation
    | securityAssertion
    | securityContext
    ;


/*
 * ============================================================================
 * 39. TOP-LEVEL SECURITY UNIT
 * ============================================================================
 *
 * This rule intentionally does not define a complete program.
 *
 * The canonical source-unit grammar remains responsible for determining where
 * security members may occur.
 */

securityUnit
    : securityMember*
    ;


/*
 * ============================================================================
 * 40. SEMANTIC BOUNDARY SUMMARY
 * ============================================================================
 *
 * SECURITY GRAMMAR OWNS:
 *
 *     source syntax
 *     declaration syntax
 *     policy syntax
 *     principal syntax
 *     capability syntax
 *     permission syntax
 *     trust syntax
 *     classification syntax
 *     audit syntax
 *     security operation syntax
 *     security requirement syntax
 *     security effect references
 *
 * SECURITY GRAMMAR DOES NOT OWN:
 *
 *     authentication
 *     authorization
 *     cryptography
 *     key management
 *     identity storage
 *     credential storage
 *     certificate validation
 *     trust evaluation
 *     policy evaluation
 *     capability discovery
 *     resource allocation
 *     hardware selection
 *     network access
 *     filesystem access
 *     runtime execution
 *     quantum semantics
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *     optimization
 *     resilience
 *     canonical IR
 *
 * ============================================================================
 * 41. NO HARD-CODED SECURITY LIMITS
 * ============================================================================
 *
 * Forbidden examples:
 *
 *     MAX_PRINCIPALS = ...
 *     MAX_PERMISSIONS = ...
 *     MAX_CAPABILITIES = ...
 *     MAX_POLICY_RULES = ...
 *     MAX_TRUST_RELATIONSHIPS = ...
 *     MAX_SECURITY_LABELS = ...
 *     MAX_KEYS = ...
 *     MAX_ALGORITHMS = ...
 *     MAX_DEVICES = ...
 *
 * None are represented in this grammar.
 *
 * ============================================================================
 */