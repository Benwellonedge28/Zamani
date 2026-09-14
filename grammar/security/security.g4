/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/security/security.g4
 *
 * Role:
 *     SECURITY COMPOSITION ROOT.
 *
 *     This file is the parser-level integration boundary for Zamani security
 *     syntax. It composes the specialized security grammars without
 *     duplicating their ownership.
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
 *     - No filesystem access.
 *     - No network access.
 *     - No runtime calls.
 *     - No hardware discovery.
 *     - No capability discovery.
 *     - No policy evaluation.
 *     - No cryptographic execution.
 *     - No secret handling.
 *     - No unsafe code.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     grammar/antlr/ZamaniLexer.g4
 *          |
 *          v
 *     shared parser grammars
 *          |
 *          +-------------------------------+
 *          |                               |
 *          v                               v
 *     domain grammars              THIS FILE
 *                                      |
 *                                      v
 *                              Security composition
 *                                      |
 *                                      v
 *                                  frontend AST
 *                                      |
 *          +---------------------------+---------------------------+
 *          |                           |                           |
 *          v                           v                           v
 *     name resolution            type/effect analysis       security analysis
 *                                                                  |
 *                                                                  v
 *                                                         canonical semantics
 *                                                                  |
 *              +----------------------+----------------------------+--------+
 *              |                      |                            |        |
 *              v                      v                            v        v
 *        classical IR           quantum::ir                HDL/hardware   metadata
 *              |                      |                            |
 *              +----------------------+----------------------------+
 *                                     |
 *                                     v
 *                          optimization / routing
 *                                     |
 *                                     v
 *                            scheduling / resilience
 *                                     |
 *                                     v
 *                          target lowering / runtime
 *
 * ============================================================================
 * PRIMARY DESIGN RULE
 * ============================================================================
 *
 * THIS FILE IS A COMPOSITION ROOT.
 *
 * It owns:
 *
 *     - the security parser entry points;
 *     - security-wide composition;
 *     - security-only cross-domain attachment syntax;
 *     - security declaration aggregation;
 *     - integration of specialized security grammars;
 *     - security-wide source-level requirements/preferences;
 *     - security-domain declarations not owned by a specialized grammar.
 *
 * It DOES NOT duplicate:
 *
 *     identities
 *     capabilities
 *     permissions
 *     cryptography
 *     privacy
 *     trust
 *     security constraints
 *
 * Those concepts have specialized grammar owners.
 *
 * ============================================================================
 * SPECIALIZED OWNERSHIP
 * ============================================================================
 *
 * Identity / principal syntax:
 *
 *     grammar/security/identifiers.g4
 *
 * Actual file currently named `identifiers.g4` but declaring the parser
 * grammar `Identities`.
 *
 * Security capabilities:
 *
 *     grammar/security/capabilities.g4
 *
 * Permissions / authorization:
 *
 *     grammar/security/permissions.g4
 *
 * Cryptographic intent:
 *
 *     grammar/security/cryptography.g4
 *
 * Privacy:
 *
 *     grammar/security/privacy.g4
 *
 * Trust:
 *
 *     grammar/security/trust.g4
 *
 * Security-specific constraints:
 *
 *     grammar/security/security-constraints.g4
 *
 * Generic capabilities:
 *
 *     grammar/core/capabilities.g4
 *
 * Generic requirements:
 *
 *     grammar/core/requirements.g4
 *
 * Generic constraints:
 *
 *     grammar/core/constraints.g4
 *
 * Effects:
 *
 *     grammar/effects/*
 *
 * ============================================================================
 * NON-OWNERSHIP
 * ============================================================================
 *
 * This file MUST NOT own:
 *
 *     - lexical token spelling;
 *     - identifiers;
 *     - qualified-name syntax;
 *     - general expressions;
 *     - general types;
 *     - identity implementation;
 *     - authentication implementation;
 *     - authorization enforcement;
 *     - credential storage;
 *     - key storage;
 *     - cryptographic implementation;
 *     - trust evaluation;
 *     - privacy enforcement;
 *     - capability discovery;
 *     - hardware discovery;
 *     - resource allocation;
 *     - quantum IR;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - backend selection;
 *     - runtime execution;
 *     - deployment.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Security syntax describes portable security intent.
 *
 * It MUST NOT encode accidental properties of the machine currently used to
 * execute the program.
 *
 * Therefore this grammar imposes no language-level maximum on:
 *
 *     principals
 *     identities
 *     permissions
 *     capabilities
 *     policies
 *     resources
 *     devices
 *     nodes
 *     processors
 *     accelerators
 *     qubits
 *     memory
 *     storage
 *     security declarations
 *     policy rules
 *     trust relationships
 *     cryptographic objects
 *     privacy declarations
 *
 * No MAX_* constants are permitted in this grammar.
 *
 * No physical device identifier is a required part of portable security
 * semantics.
 *
 * No provider-specific security mechanism is a closed language enumeration.
 *
 * ============================================================================
 * OPEN-WORLD SECURITY
 * ============================================================================
 *
 * Security names are represented through canonical identifiers and qualified
 * names.
 *
 * This allows source programs to refer to future security concepts without
 * requiring a grammar redesign.
 *
 * Examples:
 *
 *     security::confidentiality
 *     security::integrity
 *     security::trusted_execution
 *     organization::policy
 *     future::security::mechanism
 *     quantum::security::property
 *     hardware::security::property
 *
 * The grammar does not permanently enumerate:
 *
 *     AES
 *     RSA
 *     ML-KEM
 *     ML-DSA
 *     TLS
 *     TPM
 *     HSM
 *     SGX
 *     SEV
 *     TrustZone
 *
 * as the only possible security mechanisms.
 *
 * ============================================================================
 * SECRET-MATERIAL BOUNDARY
 * ============================================================================
 *
 * This grammar MUST NOT create a language mechanism whose purpose is to embed
 * secret material.
 *
 * It must not introduce source-level primitives for:
 *
 *     passwords
 *     private keys
 *     secret keys
 *     bearer tokens
 *     session secrets
 *     API secrets
 *     authentication secrets
 *     recovery secrets
 *
 * Security objects may be referenced symbolically where another security
 * grammar permits references.
 *
 * Actual secret material belongs to secure runtime/key-management systems.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Security may protect quantum programs and quantum resources.
 *
 * This file MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     topology
 *     calibration
 *     QEC codes
 *     ZQN fault models
 *     backend selection
 *
 * Quantum semantics remain owned downstream by the canonical quantum semantic
 * boundary:
 *
 *     quantum::ir
 *
 * Security information may be propagated as semantic metadata or constraints
 * toward quantum compilation, but this parser does not construct quantum IR.
 *
 * ============================================================================
 * HARDWARE BOUNDARY
 * ============================================================================
 *
 * Security syntax may protect:
 *
 *     CPU computation
 *     GPU computation
 *     FPGA computation
 *     ASIC computation
 *     accelerator execution
 *     quantum execution
 *     distributed execution
 *     embedded execution
 *     future computational substrates
 *
 * It MUST NOT require a fixed physical machine.
 *
 * A source-level requirement such as:
 *
 *     security::trusted_execution
 *
 * is an abstract semantic requirement.
 *
 * Target selection belongs to:
 *
 *     capability analysis
 *     resource analysis
 *     target lowering
 *     deployment
 *     runtime
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no randomness;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no runtime state;
 *     - no policy evaluation.
 *
 * Therefore parsing depends only on the supplied token stream and grammar
 * version.
 *
 * ============================================================================
 * ERROR-RECOVERY PRINCIPLE
 * ============================================================================
 *
 * Error recovery belongs to the generated parser/frontend configuration.
 *
 * This grammar MUST NOT:
 *
 *     - execute user code;
 *     - silently reinterpret security syntax;
 *     - evaluate security policy;
 *     - query external state;
 *     - select a security mechanism;
 *     - resolve credentials.
 *
 * Invalid syntax must remain distinguishable from valid syntax.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser preserves syntax.
 *
 * The frontend AST is responsible for semantic objects such as:
 *
 *     SecurityDomain
 *     SecurityRequirement
 *     SecurityPreference
 *     SecurityAttachment
 *     SecurityClassification
 *     SecurityAudit
 *
 * Specialized grammars provide:
 *
 *     Identity
 *     Principal
 *     Capability
 *     Permission
 *     AuthorizationPolicy
 *     CryptographicIntent
 *     PrivacyPolicy
 *     TrustRelationship
 *     SecurityConstraint
 *
 * The parser does not instantiate runtime security objects.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis, not parsing, determines:
 *
 *     - whether names resolve;
 *     - whether references are legal;
 *     - whether requirements are satisfiable;
 *     - whether policies conflict;
 *     - whether trust relationships are valid;
 *     - whether permissions are authorized;
 *     - whether a capability is delegable;
 *     - whether an attenuation is safe;
 *     - whether cryptographic requirements are implementable;
 *     - whether privacy requirements can be satisfied;
 *     - whether hardware capabilities satisfy security requirements;
 *     - whether quantum execution can preserve security properties;
 *     - whether a target satisfies mandatory constraints.
 *
 * ============================================================================
 * COMPILATION CONTRACT
 * ============================================================================
 *
 * Compilation may consume the security AST and semantic model to produce
 * security metadata attached to canonical semantic representations.
 *
 * Security metadata may accompany:
 *
 *     classical IR
 *     quantum::ir
 *     HDL/hardware IR
 *     distributed representations
 *     deployment metadata
 *
 * The compiler MUST preserve mandatory security requirements through lowering.
 *
 * This grammar does not decide how the compiler satisfies those requirements.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime systems may evaluate security requirements against actual execution
 * state.
 *
 * Such evaluation is not parsing.
 *
 * Examples include:
 *
 *     credential verification
 *     trust verification
 *     policy evaluation
 *     capability validation
 *     secure-environment verification
 *     runtime authorization
 *
 * These operations remain outside this grammar.
 *
 * ============================================================================
 * RESILIENCE CONTRACT
 * ============================================================================
 *
 * Security metadata may be consumed by the resilience subsystem.
 *
 * Resilience may need to ensure that recovery actions preserve mandatory
 * security guarantees.
 *
 * This grammar MUST NOT implement:
 *
 *     retry
 *     rollback
 *     resume
 *     reroute
 *     reschedule
 *     recompile
 *     backend switching
 *     quarantine
 *     recovery
 *
 * ============================================================================
 * RESOURCE / SCALE CONTRACT
 * ============================================================================
 *
 * Security requirements may describe abstract resource properties.
 *
 * They must not turn resource capacity into fixed grammar constants.
 *
 * For example, a requirement may refer to:
 *
 *     security::protected_memory
 *     security::trusted_execution
 *     security::isolated_execution
 *     security::secure_channel
 *
 * without requiring:
 *
 *     N CPUs
 *     N GPUs
 *     N QPUs
 *     N nodes
 *     N qubits
 *     N bytes
 *
 * Resource feasibility is determined downstream.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * The complete dependency direction is:
 *
 *     ZamaniLexer
 *          |
 *          v
 *     Core / Types / Expressions
 *          |
 *          v
 *     specialized security grammars
 *          |
 *          v
 *     Security composition root
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic security analysis
 *          |
 *          +--> capability analysis
 *          +--> identity analysis
 *          +--> permission analysis
 *          +--> trust analysis
 *          +--> privacy analysis
 *          +--> cryptographic analysis
 *          +--> constraint analysis
 *          +--> resource analysis
 *          |
 *          v
 *     canonical semantic representations
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware IR
 *          |
 *          v
 *     optimization / routing / scheduling
 *          |
 *          v
 *     resilience / target lowering
 *          |
 *          v
 *     runtime / deployment
 *
 * The dependency direction MUST NOT be reversed.
 *
 * ============================================================================
 * SPECIALIZED GRAMMAR IMPORTS
 * ============================================================================
 *
 * Each imported grammar owns its own domain.
 *
 * IMPORTANT:
 *
 *     Do not recreate rules from these grammars in this file.
 *
 * The imports deliberately establish one composition boundary.
 *
 * ============================================================================
 */

parser grammar Security;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * IMPORTS
 * ============================================================================
 *
 * Core provides:
 *
 *     names
 *     paths
 *     attributes
 *     metadata
 *     visibility
 *     capabilities
 *     requirements
 *     constraints
 *
 * Types provides canonical type syntax.
 *
 * Expressions provides canonical expression syntax.
 *
 * Effects provides canonical effect syntax.
 *
 * Security-specific imports provide the specialized security domains.
 *
 * ============================================================================
 */

import
    Core,
    Types,
    Expressions,
    Effects,
    Identities,
    SecurityCapabilities,
    Permissions,
    Cryptography,
    Privacy,
    Trust,
    SecurityConstraints;


/*
 * ============================================================================
 * 1. SECURITY FILE ENTRY
 * ============================================================================
 *
 * This is the principal standalone entry point for security grammar tests,
 * parser integration tests, grammar tooling and future parser composition.
 *
 * It consumes zero or more security declarations followed by EOF.
 *
 * No fixed declaration count exists.
 *
 * ============================================================================
 */

securityFile
    : securityDeclaration*
      EOF
    ;


/*
 * ============================================================================
 * 2. SECURITY DECLARATION
 * ============================================================================
 *
 * This rule is the single aggregation point for security syntax.
 *
 * Specialized domains are referenced through their owning rules.
 *
 * ============================================================================
 */

securityDeclaration
    : securityDomainDeclaration

    /*
     * Identity / principal domain.
     */
    | identityDeclaration
    | principalDeclaration
    | principalGroupDeclaration
    | identityBindingDeclaration

    /*
     * Security authority domain.
     */
    | securityCapabilityDeclaration

    /*
     * Authorization domain.
     */
    | permissionDeclaration
    | authorizationPolicyDeclaration
    | permissionBinding
    | permissionGrantBinding
    | permissionDenyBinding
    | permissionDelegation
    | scopedPermissionDeclaration
    | principalPermissionBinding

    /*
     * Cryptographic domain.
     */
    | cryptographicDeclaration
    | cryptographicRequirementDeclaration
    | cryptographicConstraintDeclaration
    | cryptographicPreferenceDeclaration

    /*
     * Privacy domain.
     */
    | privacyDeclaration
    | privacyPolicyDeclaration
    | privacyRequirementDeclaration
    | privacyConstraintDeclaration
    | privacyPreferenceDeclaration
    | privacyPurposeDeclaration
    | privacyClassificationDeclaration
    | privacyObligationDeclaration

    /*
     * Trust domain.
     */
    | trustRelationshipDeclaration
    | trustRequirementDeclaration
    | trustPreferenceDeclaration
    | trustAssertionDeclaration

    /*
     * Security constraint domain.
     */
    | securityConstraintDeclaration

    /*
     * Security-wide intent.
     */
    | securityRequirementDeclaration
    | securityPreferenceDeclaration
    | securityClassificationDeclaration
    | securityAuditDeclaration
    | securityBindingDeclaration
    | authenticationIntent
    | authorizationIntent
    | securityCryptographicIntent
    ;


/*
 * ============================================================================
 * 3. SECURITY DOMAIN
 * ============================================================================
 *
 * A security domain groups source-level security declarations.
 *
 * A security domain is a language construct, not a runtime security domain.
 *
 * It does not:
 *
 *     - establish trust;
 *     - authenticate principals;
 *     - allocate resources;
 *     - create credentials;
 *     - select hardware.
 *
 * ============================================================================
 */

securityDomainDeclaration
    : attributes*
      visibility?
      SECURITY
      DOMAIN
      qualifiedName
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
    : securityDeclaration
    ;


/*
 * ============================================================================
 * 4. SECURITY REQUIREMENTS
 * ============================================================================
 *
 * A security requirement is stronger than a preference.
 *
 * It expresses a condition that the semantic/compilation pipeline must
 * preserve and satisfy when the program declares it as mandatory.
 *
 * This grammar does not determine whether the requirement is satisfiable.
 *
 * ============================================================================
 */

securityRequirementDeclaration
    : attributes*
      visibility?
      SECURITY
      REQUIREMENT
      identifier?
      REQUIRES
      securityRequirementExpression
      SEMI?
    ;


securityRequirementExpression
    : securityRequirementDisjunction
    ;


securityRequirementDisjunction
    : securityRequirementConjunction
      (
          OR
          securityRequirementConjunction
      )*
    ;


securityRequirementConjunction
    : securityRequirementUnary
      (
          AND
          securityRequirementUnary
      )*
    ;


securityRequirementUnary
    : NOT
      securityRequirementUnary

    | securityRequirementPrimary
    ;


securityRequirementPrimary
    : securityRequirementReference
    | securityRequirementCall
    | LPAREN
      securityRequirementExpression
      RPAREN
    | securityRequirementComparison
    ;


securityRequirementReference
    : qualifiedName
    ;


securityRequirementCall
    : qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


securityRequirementComparison
    : qualifiedName
      securityComparisonOperator
      expression
    ;


securityComparisonOperator
    : EQ
    | NEQ
    | LT
    | LE
    | GT
    | GE
    ;


/*
 * ============================================================================
 * 5. SECURITY PREFERENCES
 * ============================================================================
 *
 * Preferences express desirable implementation/security properties.
 *
 * They are intentionally weaker than requirements.
 *
 * A preference MUST NOT be interpreted as a mandatory guarantee by the parser.
 *
 * ============================================================================
 */

securityPreferenceDeclaration
    : attributes*
      visibility?
      PREFERENCE
      identifier?
      PREFER
      securityPreferenceExpression
      SEMI?
    ;


securityPreferenceExpression
    : securityPreferenceDisjunction
    ;


securityPreferenceDisjunction
    : securityPreferenceConjunction
      (
          OR
          securityPreferenceConjunction
      )*
    ;


securityPreferenceConjunction
    : securityPreferenceUnary
      (
          AND
          securityPreferenceUnary
      )*
    ;


securityPreferenceUnary
    : NOT
      securityPreferenceUnary

    | securityPreferencePrimary
    ;


securityPreferencePrimary
    : qualifiedName
    | qualifiedName
      LPAREN
      argumentList?
      RPAREN
    | LPAREN
      securityPreferenceExpression
      RPAREN
    | expression
    ;


/*
 * ============================================================================
 * 6. SECURITY CLASSIFICATION
 * ============================================================================
 *
 * Classification is intentionally open-world.
 *
 * The grammar does not encode a finite regulatory classification vocabulary.
 *
 * Examples may be represented as:
 *
 *     security::classification::public
 *     organization::classification::restricted
 *     future::classification::level
 *
 * Actual classification semantics are resolved downstream.
 *
 * ============================================================================
 */

securityClassificationDeclaration
    : attributes*
      visibility?
      CLASSIFICATION
      qualifiedName
      securityClassificationBody?
      SEMI?
    ;


securityClassificationBody
    : LBRACE
      securityClassificationMember*
      RBRACE
    ;


securityClassificationMember
    : securityClassificationLabel
    | securityClassificationProperty
    | securityRequirementDeclaration
    | securityPreferenceDeclaration
    ;


securityClassificationLabel
    : qualifiedName
      (ASSIGN expression)?
      SEMI?
    ;


securityClassificationProperty
    : identifier
      (COLON typeExpression)?
      (ASSIGN expression)?
      SEMI
    ;


/*
 * ============================================================================
 * 7. SECURITY AUDIT
 * ============================================================================
 *
 * Audit syntax describes source-level audit intent.
 *
 * It does not implement logging or storage.
 *
 * ============================================================================
 */

securityAuditDeclaration
    : attributes*
      visibility?
      AUDIT
      identifier?
      securityAuditBody?
      SEMI?
    ;


securityAuditBody
    : LBRACE
      securityAuditMember*
      RBRACE
    ;


securityAuditMember
    : securityAuditRequirement
    | securityAuditProperty
    | securityAuditEvent
    ;


securityAuditRequirement
    : REQUIRES
      securityAuditExpression
      SEMI?
    ;


securityAuditExpression
    : qualifiedName
    | qualifiedName
      LPAREN
      argumentList?
      RPAREN
    | expression
    ;


securityAuditProperty
    : identifier
      (COLON typeExpression)?
      (ASSIGN expression)?
      SEMI
    ;


securityAuditEvent
    : qualifiedName
      LPAREN
      argumentList?
      RPAREN
      SEMI?
    ;


/*
 * ============================================================================
 * 8. SECURITY BINDINGS
 * ============================================================================
 *
 * A security binding attaches security intent to an already existing source
 * entity.
 *
 * It does not redefine that entity.
 *
 * It does not select hardware.
 *
 * ============================================================================
 */

securityBindingDeclaration
    : attributes*
      visibility?
      SECURITY
      BINDING
      qualifiedName
      securityBindingBody?
      SEMI?
    ;


securityBindingBody
    : LBRACE
      securityBindingMember*
      RBRACE
    ;


securityBindingMember
    : securityBindingRequirement
    | securityBindingPreference
    | securityBindingProperty
    | securityBindingAudit
    ;


securityBindingRequirement
    : REQUIRES
      securityBindingExpression
      SEMI?
    ;


securityBindingPreference
    : PREFER
      securityBindingExpression
      SEMI?
    ;


securityBindingExpression
    : qualifiedName
    | qualifiedName
      LPAREN
      argumentList?
      RPAREN
    | expression
    ;


securityBindingProperty
    : identifier
      (COLON typeExpression)?
      (ASSIGN expression)?
      SEMI
    ;


securityBindingAudit
    : AUDIT
      securityAuditExpression
      SEMI?
    ;


/*
 * ============================================================================
 * 9. AUTHENTICATION INTENT
 * ============================================================================
 *
 * Authentication is represented only as source-level intent here.
 *
 * Actual authentication is downstream.
 *
 * The grammar deliberately uses symbolic mechanism references rather than
 * defining a closed authentication mechanism vocabulary.
 *
 * ============================================================================
 */

authenticationIntent
    : AUTHENTICATION
      authenticationTarget?
      authenticationClause*
      SEMI?
    ;


authenticationTarget
    : FOR
      qualifiedName
    ;


authenticationClause
    : USING
      qualifiedNameList
    | WITH
      securityPropertyList
    | AS
      qualifiedName
    | IF
      expression
    ;


qualifiedNameList
    : qualifiedName
      (
          COMMA
          qualifiedName
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 10. AUTHORIZATION INTENT
 * ============================================================================
 *
 * Authorization declarations are aggregated from the permissions grammar
 * wherever a complete policy/rule construct is needed.
 *
 * This rule exists only for compact source-level intent attachments.
 *
 * Actual authorization remains downstream.
 *
 * ============================================================================
 */

authorizationIntent
    : AUTHORIZATION
      authorizationDecision
      authorizationTarget?
      authorizationClause*
      SEMI?
    ;


authorizationDecision
    : GRANT
    | DENY
    | ALLOW
    | REJECT
    | REVOKE
    ;


authorizationTarget
    : TO
      qualifiedName
    ;


authorizationClause
    : WHEN
      expression
    | WITH
      securityPropertyList
    | FOR
      qualifiedName
    ;


securityPropertyList
    : securityPropertyItem
      (
          COMMA
          securityPropertyItem
      )*
      COMMA?
    ;


securityPropertyItem
    : qualifiedName
      (
          ASSIGN
          expression
      )?
    ;


/*
 * ============================================================================
 * 11. CRYPTOGRAPHIC INTENT
 * ============================================================================
 *
 * Cryptography itself belongs to Cryptography.
 *
 * This rule is merely the security-wide compact attachment form.
 *
 * Algorithm names remain open-world qualified names.
 *
 * ============================================================================
 */

securityCryptographicIntent
    : CRYPTOGRAPHY
      securityCryptographicOperation
      securityCryptographicTarget?
      securityCryptographicClause*
      SEMI?
    ;


securityCryptographicOperation
    : ENCRYPT
    | DECRYPT
    | SIGN
    | VERIFY
    ;


securityCryptographicTarget
    : ON
      qualifiedName
    ;


securityCryptographicClause
    : USING
      qualifiedNameList
    | WITH
      securityPropertyList
    | AS
      qualifiedName
    | FOR
      qualifiedName
    ;


/*
 * ============================================================================
 * 12. SECURITY TARGET
 * ============================================================================
 *
 * Security can attach to arbitrary source-level entities.
 *
 * A target is symbolic.
 *
 * It does not identify a physical machine.
 *
 * ============================================================================
 */

securityTarget
    : qualifiedName
    ;


securityTargetList
    : securityTarget
      (
          COMMA
          securityTarget
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 13. SECURITY ATTACHMENT
 * ============================================================================
 *
 * General-purpose source-level security attachment.
 *
 * Example conceptual forms:
 *
 *     security on module::component {
 *         ...
 *     }
 *
 * The target is symbolic and resolved downstream.
 *
 * ============================================================================
 */

securityAttachment
    : SECURITY
      ON
      securityTarget
      securityAttachmentBody?
      SEMI?
    ;


securityAttachmentBody
    : LBRACE
      securityAttachmentMember*
      RBRACE
    ;


securityAttachmentMember
    : securityAttachmentRequirement
    | securityAttachmentPreference
    | securityAttachmentProperty
    | securityAttachmentAudit
    ;


securityAttachmentRequirement
    : REQUIRES
      securityAttachmentExpression
      SEMI?
    ;


securityAttachmentPreference
    : PREFER
      securityAttachmentExpression
      SEMI?
    ;


securityAttachmentExpression
    : qualifiedName
    | qualifiedName
      LPAREN
      argumentList?
      RPAREN
    | expression
    ;


securityAttachmentProperty
    : qualifiedName
      (
          ASSIGN
          expression
      )?
      SEMI?
    ;


securityAttachmentAudit
    : AUDIT
      securityAuditExpression
      SEMI?
    ;


/*
 * ============================================================================
 * 14. SECURITY CLAUSE
 * ============================================================================
 *
 * Common composition point for parent grammars.
 *
 * These rules do not execute or evaluate security semantics.
 *
 * ============================================================================
 */

securityClause
    : securityRequirementClause
    | securityPreferenceClause
    | securityAuditClause
    | securityAuthenticationClause
    | securityAuthorizationClause
    | securityCryptographyClause
    ;


securityRequirementClause
    : REQUIRES
      securityRequirementExpression
      SEMI?
    ;


securityPreferenceClause
    : PREFER
      securityPreferenceExpression
      SEMI?
    ;


securityAuditClause
    : AUDIT
      securityAuditExpression
      SEMI?
    ;


securityAuthenticationClause
    : authenticationIntent
    ;


securityAuthorizationClause
    : authorizationIntent
    ;


securityCryptographyClause
    : securityCryptographicIntent
    ;


securityClauseList
    : securityClause+
    ;


/*
 * ============================================================================
 * 15. SECURITY PROPERTY
 * ============================================================================
 *
 * A property is a symbolic semantic property.
 *
 * This file deliberately uses a unique rule name:
 *
 *     securityPropertyItem
 *
 * rather than defining a second generic `securityProperty` rule that can
 * collide with specialized security grammars.
 *
 * ============================================================================
 */

securityProperty
    : securityPropertyReference
    | securityPropertyComparison
    | LPAREN
      securityPropertyExpression
      RPAREN
    ;


securityPropertyReference
    : qualifiedName
    ;


securityPropertyComparison
    : qualifiedName
      securityComparisonOperator
      expression
    ;


securityPropertyExpression
    : securityPropertyConjunction
    ;


securityPropertyConjunction
    : securityPropertyUnary
      (
          AND
          securityPropertyUnary
      )*
    ;


securityPropertyUnary
    : NOT
      securityPropertyUnary

    | securityProperty
    ;


/*
 * ============================================================================
 * 16. SECURITY PROPERTY SET
 * ============================================================================
 */

securityPropertySet
    : LBRACE
      securityPropertySetMember*
      RBRACE
    ;


securityPropertySetMember
    : qualifiedName
      (
          ASSIGN
          expression
      )?
      COMMA?
    ;


/*
 * ============================================================================
 * 17. SECURITY SPECIFICATION
 * ============================================================================
 *
 * General composition point for tooling and parent grammars.
 * ============================================================================
 */

securitySpecification
    : securityClauseList
    | securityPropertyExpression
    | securityPropertySet
    | securityTarget
    ;


/*
 * ============================================================================
 * 18. SECURITY REQUIREMENT LIST
 * ============================================================================
 */

securityRequirementList
    : securityRequirementClause+
    ;


/*
 * ============================================================================
 * 19. SECURITY PREFERENCE LIST
 * ============================================================================
 */

securityPreferenceList
    : securityPreferenceClause+
    ;


/*
 * ============================================================================
 * 20. SECURITY AUDIT LIST
 * ============================================================================
 */

securityAuditList
    : securityAuditClause+
    ;


/*
 * ============================================================================
 * 21. SECURITY TARGET LIST
 * ============================================================================
 */

securityTargets
    : securityTargetList
    ;


/*
 * ============================================================================
 * 22. STANDALONE SECURITY CONSTRUCT
 * ============================================================================
 *
 * Useful for grammar tooling that wants one construct without requiring a
 * complete file.
 * ============================================================================
 */

securityConstruct
    : securityDeclaration
    | securityClause
    | securityAttachment
    | securitySpecification
    ;


/*
 * ============================================================================
 * 23. SECURITY COMPOSITION CONTRACT
 * ============================================================================
 *
 * The following concepts are intentionally references to imported grammars:
 *
 *     identityDeclaration
 *     principalDeclaration
 *     principalGroupDeclaration
 *     identityBindingDeclaration
 *
 *     securityCapabilityDeclaration
 *
 *     permissionDeclaration
 *     authorizationPolicyDeclaration
 *     permissionBinding
 *     permissionGrantBinding
 *     permissionDenyBinding
 *     permissionDelegation
 *     scopedPermissionDeclaration
 *     principalPermissionBinding
 *
 *     cryptographicDeclaration
 *     cryptographicRequirementDeclaration
 *     cryptographicConstraintDeclaration
 *     cryptographicPreferenceDeclaration
 *
 *     privacyDeclaration
 *     privacyPolicyDeclaration
 *     privacyRequirementDeclaration
 *     privacyConstraintDeclaration
 *     privacyPreferenceDeclaration
 *     privacyPurposeDeclaration
 *     privacyClassificationDeclaration
 *     privacyObligationDeclaration
 *
 *     trustRelationshipDeclaration
 *     trustRequirementDeclaration
 *     trustPreferenceDeclaration
 *     trustAssertionDeclaration
 *
 *     securityConstraintDeclaration
 *
 * These rules MUST remain owned by their respective specialized grammars.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 24. SEMANTIC PRESERVATION CONTRACT
 * ============================================================================
 *
 * The security frontend MUST preserve:
 *
 *     source order
 *     source spans
 *     declaration kind
 *     qualified-name structure
 *     explicit requirements
 *     explicit preferences
 *     explicit policy decisions
 *     explicit trust relationships
 *     explicit classifications
 *     explicit audit intent
 *     explicit cryptographic intent
 *     explicit privacy intent
 *     explicit capability references
 *     explicit permission references
 *
 * No security semantics may be silently discarded during parser-to-AST
 * conversion.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 25. HARD-CODING AUDIT CONTRACT
 * ============================================================================
 *
 * This file MUST remain free of:
 *
 *     MAX_SECURITY_DOMAINS
 *     MAX_PRINCIPALS
 *     MAX_CAPABILITIES
 *     MAX_PERMISSIONS
 *     MAX_POLICIES
 *     MAX_TRUST_RELATIONSHIPS
 *     MAX_KEYS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_MEMORY
 *     MAX_STORAGE
 *
 * and equivalent finite grammar-level restrictions.
 *
 * Repetition is expressed with ANTLR repetition operators.
 *
 * Practical limits are implementation/resource limits and belong outside the
 * language grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 26. SECURITY / QUANTUM INTEGRATION CONTRACT
 * ============================================================================
 *
 * Security syntax may constrain quantum computation through symbolic
 * requirements.
 *
 * Example semantic references:
 *
 *     quantum::execution_integrity
 *     quantum::confidential_execution
 *     quantum::measurement_provenance
 *     quantum::trusted_execution
 *
 * The grammar does not know:
 *
 *     qubit count
 *     physical qubit identifiers
 *     topology
 *     calibration
 *     gate durations
 *     QEC code
 *     ZQN model
 *     backend
 *
 * Those remain downstream concerns.
 *
 * Security metadata may accompany quantum::ir without making this grammar
 * depend directly on quantum IR implementation types.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 27. SECURITY / HDL INTEGRATION CONTRACT
 * ============================================================================
 *
 * Security may attach to:
 *
 *     hardware modules
 *     interfaces
 *     signals
 *     memories
 *     processes
 *     execution regions
 *     accelerator boundaries
 *
 * through symbolic references.
 *
 * This grammar does not define HDL semantics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 28. SECURITY / DISTRIBUTED INTEGRATION CONTRACT
 * ============================================================================
 *
 * Security may attach to:
 *
 *     services
 *     nodes
 *     messages
 *     communication
 *     replication
 *     deployment
 *
 * through symbolic references.
 *
 * No node count, cluster size, topology or provider is hard-coded here.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 29. SECURITY / RESOURCE INTEGRATION CONTRACT
 * ============================================================================
 *
 * Security requirements may be consumed by resource analysis.
 *
 * Example conceptual requirement:
 *
 *     requires security::trusted_execution;
 *
 * The resource system determines whether an available execution environment
 * satisfies the requirement.
 *
 * This grammar does not discover resources.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 30. SECURITY / COMPILATION INTEGRATION CONTRACT
 * ============================================================================
 *
 * Security intent flows:
 *
 *     parser
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic security model
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware IR
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing/scheduling
 *       |
 *       v
 *     target lowering
 *
 * Mandatory security requirements MUST survive every lowering boundary.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 31. SECURITY / RUNTIME INTEGRATION CONTRACT
 * ============================================================================
 *
 * Runtime systems may consume compiled security metadata.
 *
 * Runtime implementation may perform:
 *
 *     authentication
 *     authorization
 *     policy evaluation
 *     credential resolution
 *     trust verification
 *     capability verification
 *     security monitoring
 *
 * None of these operations occur in the grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 32. SECURITY / RESILIENCE INTEGRATION CONTRACT
 * ============================================================================
 *
 * Resilience may adapt execution while preserving mandatory security
 * guarantees.
 *
 * A recovery strategy that would violate a mandatory security requirement must
 * be rejected by downstream semantic/resilience analysis.
 *
 * The grammar remains unaware of the actual recovery strategy.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 33. VERSIONING CONTRACT
 * ============================================================================
 *
 * Grammar evolution is governed by the language-version and compatibility
 * specifications.
 *
 * This file MUST NOT infer language versions from external state.
 *
 * Changes to security syntax require:
 *
 *     grammar/specification/language-version.md
 *     grammar/specification/compatibility.md
 *     grammar/compatibility/*
 *
 * to be updated as appropriate.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 34. TOOLING CONTRACT
 * ============================================================================
 *
 * Tooling may use:
 *
 *     securityFile
 *     securityDeclaration
 *     securityConstruct
 *
 * for:
 *
 *     parsing
 *     formatting
 *     syntax highlighting
 *     diagnostics
 *     source indexing
 *     documentation extraction
 *     IDE integration
 *
 * Tooling must not treat parsing as policy evaluation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 35. TESTING CONTRACT
 * ============================================================================
 *
 * This grammar requires:
 *
 * POSITIVE TESTS
 *
 *     security domains
 *     identity declarations
 *     principals
 *     capabilities
 *     permissions
 *     authorization policies
 *     cryptographic declarations
 *     privacy declarations
 *     trust declarations
 *     security constraints
 *     requirements
 *     preferences
 *     classifications
 *     audits
 *     bindings
 *     authentication intent
 *     authorization intent
 *     cryptographic intent
 *
 * NEGATIVE TESTS
 *
 *     malformed security declarations
 *     missing targets
 *     malformed qualified names
 *     malformed requirement expressions
 *     malformed policy syntax
 *     malformed trust syntax
 *     malformed cryptographic syntax
 *     malformed privacy syntax
 *
 * BOUNDARY TESTS
 *
 *     zero security declarations
 *     one declaration
 *     deeply nested declarations
 *     large declaration sets
 *     long qualified names
 *     large policy expressions
 *     large security metadata sets
 *
 * CROSS-DOMAIN TESTS
 *
 *     classical + security
 *     quantum + security
 *     HDL + security
 *     hardware + security
 *     distributed + security
 *     AI + security
 *     data + security
 *     networking + security
 *     quantum + classical + security
 *     quantum + HDL + security
 *     quantum + distributed + security
 *
 * SCALABILITY TESTS
 *
 *     No test may assume a fixed machine size.
 *
 * DETERMINISM TESTS
 *
 *     Identical token streams must produce identical parse structures.
 *
 * ROUND-TRIP TESTS
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> formatter/serializer
 *       -> parser
 *
 * must preserve intended security semantics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 36. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 * [ ] ANTLR parser generation succeeds.
 *
 * [ ] The canonical ZamaniLexer is the only lexer vocabulary authority.
 *
 * [ ] All imported parser grammars resolve.
 *
 * [ ] No duplicate parser rule names exist within the composed grammar.
 *
 * [ ] No security domain duplicates a specialized security grammar's
 *     ownership.
 *
 * [ ] No embedded Rust actions exist.
 *
 * [ ] No semantic predicates exist.
 *
 * [ ] No unsafe code exists in the grammar or required generated integration.
 *
 * [ ] No filesystem/network/runtime behavior exists in parsing.
 *
 * [ ] No security secrets are represented as grammar primitives.
 *
 * [ ] No machine-size constants exist.
 *
 * [ ] No fixed qubit/device/node/CPU/GPU/FPGA counts exist.
 *
 * [ ] Security names remain open-world.
 *
 * [ ] Quantum semantics remain downstream at quantum::ir.
 *
 * [ ] Security metadata can accompany classical, quantum, HDL, hardware and
 *     distributed semantic representations.
 *
 * [ ] Mandatory requirements are distinguishable from preferences.
 *
 * [ ] Authentication is distinct from authorization.
 *
 * [ ] Permission is distinct from capability.
 *
 * [ ] Identity is distinct from principal authorization.
 *
 * [ ] Trust is distinct from authentication.
 *
 * [ ] Security constraint is distinct from permission.
 *
 * [ ] Privacy is distinct from cryptographic implementation.
 *
 * [ ] Parsing is deterministic.
 *
 * [ ] Positive, negative, boundary, cross-domain and round-trip tests exist.
 *
 * [ ] Grammar version/compatibility documentation is synchronized.
 *
 * [ ] Parent parser integration is tested.
 *
 * [ ] Frontend AST integration is tested.
 *
 * [ ] Security semantic analysis receives all security constructs without
 *     loss of source information.
 *
 * ============================================================================
 */