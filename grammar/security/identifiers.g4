/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/security/identities.g4
 *
 * Purpose:
 *     Canonical parser grammar for source-level security identities,
 *     principals, identity references, principal groups, identity bindings,
 *     identity attributes, authority references, and identity metadata.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains NO embedded Rust actions.
 *     This grammar contains NO semantic predicates.
 *     This grammar contains NO unsafe code.
 *     Generated/compiler integration MUST remain safe Rust.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * Source
 *   |
 *   v
 * Canonical Zamani Lexer
 *   |
 *   v
 * Parser
 *   |
 *   +--> Core names
 *   +--> Types
 *   +--> Expressions
 *   +--> Security identities       <-- THIS FILE
 *   |
 *   v
 * Frontend AST
 *   |
 *   +--> name resolution
 *   +--> identity analysis
 *   +--> principal analysis
 *   +--> capability analysis
 *   +--> authorization analysis
 *   +--> trust analysis
 *   +--> security policy analysis
 *   |
 *   v
 * Canonical semantic representation
 *   |
 *   +--> classical semantic representation
 *   +--> quantum::ir
 *   +--> HDL/hardware representation
 *   +--> distributed representation
 *   |
 *   v
 * Compilation / optimization / scheduling / routing
 *   |
 *   v
 * Runtime / deployment / hardware
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - identity declaration syntax;
 *   - principal declaration syntax;
 *   - principal-group syntax;
 *   - identity references;
 *   - principal references;
 *   - authority references;
 *   - identity aliases;
 *   - identity attributes;
 *   - principal membership declarations;
 *   - identity bindings;
 *   - identity metadata;
 *   - identity scope;
 *   - abstract identity lifecycle metadata;
 *   - identity provenance references.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical identifiers;
 *   - keyword definitions;
 *   - Unicode identifier rules;
 *   - authentication protocols;
 *   - passwords;
 *   - private keys;
 *   - certificates;
 *   - cryptographic algorithms;
 *   - credential storage;
 *   - authorization policies;
 *   - permissions;
 *   - capability declarations;
 *   - capability discovery;
 *   - trust evaluation;
 *   - cryptographic verification;
 *   - network identity-provider communication;
 *   - hardware identity discovery;
 *   - device discovery;
 *   - runtime authorization;
 *   - runtime authentication;
 *   - quantum::ir;
 *   - QEC;
 *   - ZQN;
 *   - scheduling;
 *   - routing;
 *   - optimization;
 *   - resource allocation.
 *
 * ============================================================================
 * IDENTITY MODEL
 * ============================================================================
 *
 * Identity:
 *
 *     A named semantic representation of an entity or subject identity.
 *
 * Principal:
 *
 *     A security subject that may participate in authorization decisions.
 *
 * Authority:
 *
 *     An external or internal semantic authority that may assert or govern
 *     identity information.
 *
 * Binding:
 *
 *     A source-level relationship between an identity and another identity
 *     namespace, authority, subject, or external reference.
 *
 * None of these constructs authenticate a real entity.
 *
 * Authentication belongs to downstream security/runtime infrastructure.
 *
 * ============================================================================
 * OPEN-WORLD IDENTITY MODEL
 * ============================================================================
 *
 * Identity kinds MUST remain open-ended.
 *
 * The grammar MUST NOT contain a closed enumeration such as:
 *
 *     USER
 *     ADMIN
 *     DEVICE
 *     SERVICE
 *     ROBOT
 *     QPU
 *     GPU
 *     NODE
 *     EMPLOYEE
 *
 * Instead:
 *
 *     identity application::user;
 *     identity infrastructure::device;
 *     identity quantum::operator;
 *     identity future::entity;
 *
 * are all represented using ordinary qualified names.
 *
 * Semantic analysis determines their meaning.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Identity syntax MUST remain independent of machine scale.
 *
 * It MUST NOT encode:
 *
 *     MAX_IDENTITIES
 *     MAX_PRINCIPALS
 *     MAX_GROUPS
 *     MAX_MEMBERS
 *     MAX_AUTHORITIES
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_USERS
 *
 * An identity may refer to:
 *
 *     one entity
 *     many entities
 *     one machine
 *     many machines
 *     one service
 *     many services
 *     one quantum system
 *     many quantum systems
 *     one distributed deployment
 *     a future execution environment
 *
 * without changing this grammar.
 *
 * ============================================================================
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * This grammar does NOT:
 *
 *     - authenticate identities;
 *     - validate credentials;
 *     - verify certificates;
 *     - verify signatures;
 *     - contact identity providers;
 *     - contact directories;
 *     - contact networks;
 *     - query hardware;
 *     - query secure enclaves;
 *     - access operating-system identity databases;
 *     - issue tokens;
 *     - revoke credentials;
 *     - evaluate authorization;
 *     - grant permissions;
 *     - enforce policies.
 *
 * It describes source-level identity intent only.
 *
 * ============================================================================
 * SECRET-MATERIAL RULE
 * ============================================================================
 *
 * Permanent source syntax MUST NOT contain:
 *
 *     passwords
 *     private keys
 *     secret keys
 *     bearer tokens
 *     authentication tokens
 *     session credentials
 *     provider secrets
 *     API secrets
 *     raw credential material
 *
 * Identity attributes are references/metadata only.
 *
 * Semantic validation MUST reject identity declarations that attempt to
 * embed secret or credential material.
 *
 * ============================================================================
 * CAPABILITY BOUNDARY
 * ============================================================================
 *
 * Capabilities are owned by:
 *
 *     grammar/core/capabilities.g4
 *
 * or, where appropriate, the security capability layer.
 *
 * This file may REFER to capabilities.
 *
 * It MUST NOT redefine capability declarations.
 *
 * ============================================================================
 * PERMISSION BOUNDARY
 * ============================================================================
 *
 * Permissions are owned by:
 *
 *     grammar/security/permissions.g4
 *
 * This file may REFER to permissions.
 *
 * It MUST NOT redefine permission policy syntax.
 *
 * ============================================================================
 * TRUST BOUNDARY
 * ============================================================================
 *
 * Trust relationships are owned by:
 *
 *     grammar/security/trust.g4
 *
 * Identity declarations may reference trust authorities or trust domains,
 * but MUST NOT implement trust evaluation.
 *
 * ============================================================================
 * CRYPTOGRAPHY BOUNDARY
 * ============================================================================
 *
 * Cryptographic syntax is owned by:
 *
 *     grammar/security/cryptography.g4
 *
 * This file may reference an abstract cryptographic or authentication
 * authority by name where required for identity binding.
 *
 * It MUST NOT define algorithms or cryptographic primitives.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Identities may be associated semantically with quantum computation.
 *
 * Examples:
 *
 *     identity quantum::operator;
 *     principal quantum::execution_service;
 *
 * However this grammar MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     topology
 *     calibration
 *     QEC
 *     ZQN
 *     backend selection
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 * HARDWARE BOUNDARY
 * ============================================================================
 *
 * Hardware identities may be referenced:
 *
 *     identity hardware::accelerator;
 *     principal infrastructure::node;
 *
 * but the grammar does not discover or select physical hardware.
 *
 * Hardware realization belongs to the hardware/target/resource layers.
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Identity syntax may represent distributed subjects, services, or nodes.
 *
 * The grammar MUST NOT impose a fixed number of:
 *
 *     nodes
 *     services
 *     replicas
 *     regions
 *     clusters
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser preserves:
 *
 *     - declaration kind;
 *     - source spelling;
 *     - qualified-name structure;
 *     - attributes;
 *     - membership expressions;
 *     - bindings;
 *     - authority references;
 *     - source ordering;
 *     - source spans.
 *
 * The parser MUST NOT construct runtime identity objects.
 *
 * The semantic layer may subsequently construct:
 *
 *     Identity
 *     Principal
 *     PrincipalGroup
 *     IdentityBinding
 *     IdentityReference
 *     AuthorityReference
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - whether an identity is unique;
 *     - whether a principal is unique;
 *     - whether a referenced identity exists;
 *     - whether a principal references a valid identity;
 *     - whether a group membership is valid;
 *     - whether bindings are valid;
 *     - whether authority references resolve;
 *     - whether attributes have valid types;
 *     - whether lifecycle expressions are valid;
 *     - whether an identity is permitted to participate in a policy;
 *     - whether a binding is trusted;
 *     - whether identity provenance is valid;
 *     - whether referenced credentials exist in secure runtime facilities.
 *
 * None of those decisions belong in this grammar.
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
 *     - no runtime calls;
 *     - no hardware discovery;
 *     - no randomness.
 *
 * The same token stream therefore produces the same parse structure.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * This grammar consumes canonical syntax from:
 *
 *     Core
 *     Types
 *     Expressions
 *
 * In particular it reuses:
 *
 *     identifier
 *     qualifiedName
 *     attributes
 *     visibility
 *     genericParameters
 *     expression
 *     argumentList
 *
 * Identifier syntax MUST NOT be recreated here.
 *
 * ============================================================================
 */

parser grammar Identities;

options {
    tokenVocab = ZamaniLexer;
}

import Core, Types, Expressions;


/*
 * ============================================================================
 * 1. IDENTITY DECLARATIONS
 * ============================================================================
 *
 * Examples:
 *
 *     identity application::alice;
 *
 *     identity application::alice {
 *         kind application::user;
 *         authority organization::example;
 *     }
 *
 *     identity hardware::device {
 *         kind infrastructure::device;
 *     }
 *
 * The declaration is source intent only.
 */

identityDeclaration
    : attributes?
      visibility?
      IDENTITY
      qualifiedName
      identityBody?
      SEMI?
    ;


identityBody
    : LBRACE
      identityMember*
      RBRACE
    ;


identityMember
    : identityKindClause
    | identityAuthorityClause
    | identityNamespaceClause
    | identityScopeClause
    | identityAliasClause
    | identityAttributeClause
    | identityBindingReference
    | identityProvenanceClause
    | identityLifecycleClause
    | identityMetadataClause
    ;


/*
 * ============================================================================
 * 2. IDENTITY KIND
 * ============================================================================
 *
 * Identity kinds are open-world qualified names.
 */

identityKindClause
    : KIND
      qualifiedName
      SEMI
    ;


/*
 * ============================================================================
 * 3. IDENTITY AUTHORITY
 * ============================================================================
 *
 * This is an abstract reference.
 *
 * It does not perform authentication or trust evaluation.
 */

identityAuthorityClause
    : AUTHORITY
      qualifiedName
      SEMI
    ;


identityAuthorityReference
    : AUTHORITY
      qualifiedName
    ;


/*
 * ============================================================================
 * 4. IDENTITY NAMESPACE
 * ============================================================================
 */

identityNamespaceClause
    : NAMESPACE
      qualifiedName
      SEMI
    ;


/*
 * ============================================================================
 * 5. IDENTITY SCOPE
 * ============================================================================
 *
 * Scope is an expression rather than a fixed enumeration.
 */

identityScopeClause
    : SCOPE
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 6. IDENTITY ALIASES
 * ============================================================================
 *
 * Aliases are semantic identifiers, not credentials.
 */

identityAliasClause
    : ALIAS
      identityAliasValue
      SEMI
    ;


identityAliasValue
    : qualifiedName
    | expression
    ;


/*
 * ============================================================================
 * 7. IDENTITY ATTRIBUTES
 * ============================================================================
 *
 * Attribute names are open-ended.
 *
 * Secret-bearing values MUST be rejected by semantic security validation.
 */

identityAttributeClause
    : ATTRIBUTE
      qualifiedName
      ASSIGN
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 8. IDENTITY BINDING REFERENCE
 * ============================================================================
 *
 * A binding connects an identity to another semantic identity namespace.
 *
 * It does NOT store credentials.
 */

identityBindingReference
    : BINDING
      qualifiedName
      SEMI
    ;


/*
 * ============================================================================
 * 9. IDENTITY PROVENANCE
 * ============================================================================
 *
 * Provenance identifies the source/authority of an identity declaration.
 *
 * It does not establish trust.
 */

identityProvenanceClause
    : PROVENANCE
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 10. IDENTITY LIFECYCLE
 * ============================================================================
 *
 * Lifecycle state is intentionally represented as an open expression.
 *
 * The grammar therefore does not hard-code:
 *
 *     ACTIVE
 *     DISABLED
 *     REVOKED
 *     EXPIRED
 *
 * as the only possible future states.
 */

identityLifecycleClause
    : LIFECYCLE
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 11. IDENTITY METADATA
 * ============================================================================
 */

identityMetadataClause
    : METADATA
      qualifiedName
      ASSIGN
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 12. IDENTITY REFERENCES
 * ============================================================================
 */

identityReference
    : qualifiedName
    ;


identityReferenceList
    : identityReference
      (COMMA identityReference)*
    ;


/*
 * ============================================================================
 * 13. PRINCIPAL DECLARATIONS
 * ============================================================================
 *
 * A principal is an authorization subject.
 *
 * The principal may refer to an identity but does not contain credentials.
 */

principalDeclaration
    : attributes?
      visibility?
      PRINCIPAL
      qualifiedName
      principalBody?
      SEMI?
    ;


principalBody
    : LBRACE
      principalMember*
      RBRACE
    ;


principalMember
    : principalIdentityClause
    | principalKindClause
    | principalAuthorityClause
    | principalScopeClause
    | principalAttributeClause
    | principalMembershipClause
    | principalBindingClause
    | principalMetadataClause
    ;


/*
 * ============================================================================
 * 14. PRINCIPAL -> IDENTITY
 * ============================================================================
 */

principalIdentityClause
    : IDENTITY
      identityReference
      SEMI
    ;


/*
 * ============================================================================
 * 15. PRINCIPAL KIND
 * ============================================================================
 *
 * Open-world.
 */

principalKindClause
    : KIND
      qualifiedName
      SEMI
    ;


/*
 * ============================================================================
 * 16. PRINCIPAL AUTHORITY
 * ============================================================================
 */

principalAuthorityClause
    : AUTHORITY
      qualifiedName
      SEMI
    ;


/*
 * ============================================================================
 * 17. PRINCIPAL SCOPE
 * ============================================================================
 */

principalScopeClause
    : SCOPE
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 18. PRINCIPAL ATTRIBUTES
 * ============================================================================
 */

principalAttributeClause
    : ATTRIBUTE
      qualifiedName
      ASSIGN
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 19. PRINCIPAL MEMBERSHIP
 * ============================================================================
 *
 * Membership is a semantic relation.
 *
 * The grammar imposes no finite member count.
 */

principalMembershipClause
    : MEMBER
      principalReference
      SEMI
    ;


principalMembershipList
    : principalReference
      (COMMA principalReference)*
    ;


principalReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 20. PRINCIPAL BINDINGS
 * ============================================================================
 */

principalBindingClause
    : BINDING
      qualifiedName
      SEMI
    ;


/*
 * ============================================================================
 * 21. PRINCIPAL METADATA
 * ============================================================================
 */

principalMetadataClause
    : METADATA
      qualifiedName
      ASSIGN
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 22. PRINCIPAL GROUPS
 * ============================================================================
 *
 * Groups are identity-domain structures.
 *
 * They are not permission sets.
 * They are not capability declarations.
 */

principalGroupDeclaration
    : attributes?
      visibility?
      GROUP
      qualifiedName
      principalGroupBody?
      SEMI?
    ;


principalGroupBody
    : LBRACE
      principalGroupMember*
      RBRACE
    ;


principalGroupMember
    : groupMemberClause
    | groupIdentityClause
    | groupAttributeClause
    | groupScopeClause
    | groupMetadataClause
    ;


groupMemberClause
    : MEMBER
      principalReference
      SEMI
    ;


groupIdentityClause
    : IDENTITY
      identityReference
      SEMI
    ;


groupAttributeClause
    : ATTRIBUTE
      qualifiedName
      ASSIGN
      expression
      SEMI
    ;


groupScopeClause
    : SCOPE
      expression
      SEMI
    ;


groupMetadataClause
    : METADATA
      qualifiedName
      ASSIGN
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 23. IDENTITY BINDINGS
 * ============================================================================
 *
 * Bindings describe relationships between identity namespaces.
 *
 * Examples:
 *
 *     binding application_to_federated {
 *         identity application::alice;
 *         authority organization::example;
 *         subject external::alice;
 *     }
 *
 * No credentials are permitted by this grammar.
 */

identityBindingDeclaration
    : attributes?
      visibility?
      BINDING
      qualifiedName
      identityBindingBody?
      SEMI?
    ;


identityBindingBody
    : LBRACE
      identityBindingMember*
      RBRACE
    ;


identityBindingMember
    : bindingIdentityClause
    | bindingAuthorityClause
    | bindingSubjectClause
    | bindingNamespaceClause
    | bindingMethodClause
    | bindingConditionClause
    | bindingAttributeClause
    | bindingMetadataClause
    ;


bindingIdentityClause
    : IDENTITY
      identityReference
      SEMI
    ;


bindingAuthorityClause
    : AUTHORITY
      qualifiedName
      SEMI
    ;


bindingSubjectClause
    : SUBJECT
      principalReference
      SEMI
    ;


bindingNamespaceClause
    : NAMESPACE
      qualifiedName
      SEMI
    ;


bindingMethodClause
    : METHOD
      qualifiedName
      SEMI
    ;


bindingConditionClause
    : WHEN
      expression
      SEMI
    ;


bindingAttributeClause
    : ATTRIBUTE
      qualifiedName
      ASSIGN
      expression
      SEMI
    ;


bindingMetadataClause
    : METADATA
      qualifiedName
      ASSIGN
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 24. AUTHORITY REFERENCES
 * ============================================================================
 *
 * An authority is referenced, not implemented.
 */

authorityReference
    : qualifiedName
    ;


authorityReferenceList
    : authorityReference
      (COMMA authorityReference)*
    ;


/*
 * ============================================================================
 * 25. IDENTITY SETS
 * ============================================================================
 *
 * This is deliberately structural rather than numerically bounded.
 */

identitySet
    : LBRACE
      identityReferenceList?
      RBRACE
    ;


principalSet
    : LBRACE
      principalReferenceList?
      RBRACE
    ;


principalReferenceList
    : principalReference
      (COMMA principalReference)*
    ;


/*
 * ============================================================================
 * 26. SECURITY SUBJECT REFERENCE
 * ============================================================================
 *
 * A security subject may be represented by a principal, identity, group, or
 * another open-world security reference.
 *
 * Semantic analysis determines whether the reference is valid in context.
 */

securitySubjectReference
    : IDENTITY identityReference
    | PRINCIPAL principalReference
    | GROUP qualifiedName
    | qualifiedName
    ;


/*
 * ============================================================================
 * 27. IDENTITY ATTRIBUTE LIST
 * ============================================================================
 */

identityAttributeList
    : identityAttributeEntry
      (COMMA identityAttributeEntry)*
    ;


identityAttributeEntry
    : qualifiedName
      ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 28. IDENTITY DECLARATION LIST
 * ============================================================================
 *
 * No fixed maximum.
 */

identityDeclarationList
    : identityDeclaration+
    ;


/*
 * ============================================================================
 * 29. PRINCIPAL DECLARATION LIST
 * ============================================================================
 */

principalDeclarationList
    : principalDeclaration+
    ;


/*
 * ============================================================================
 * 30. PRINCIPAL GROUP DECLARATION LIST
 * ============================================================================
 */

principalGroupDeclarationList
    : principalGroupDeclaration+
    ;


/*
 * ============================================================================
 * 31. IDENTITY FILE ENTRY
 * ============================================================================
 *
 * This rule exists for isolated grammar tests.
 *
 * The larger Security grammar MUST compose individual declaration rules
 * rather than using this EOF-consuming entry point.
 */

identitiesFile
    : identityFileEntry+
      EOF
    ;


identityFileEntry
    : identityDeclaration
    | principalDeclaration
    | principalGroupDeclaration
    | identityBindingDeclaration
    ;