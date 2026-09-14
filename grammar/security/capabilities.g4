/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/security/capabilities.g4
 *
 * Purpose:
 *     Canonical parser grammar for SECURITY AUTHORITY CAPABILITIES.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Implementation baseline:
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
 *     - No cryptographic operations.
 *     - No unsafe code.
 *
 * ============================================================================
 * CRITICAL OWNERSHIP BOUNDARY
 * ============================================================================
 *
 * Zamani has several different meanings of "capability".
 *
 * 1. grammar/core/capabilities.g4
 *
 *    Owns the LANGUAGE-WIDE CAPABILITY MODEL.
 *
 *    It describes capabilities such as:
 *
 *        quantum::dynamic_control
 *        quantum::measurement
 *        accelerator::tensor
 *        distributed::consensus
 *
 *    These answer:
 *
 *        "What can an execution environment provide?"
 *
 *
 * 2. grammar/effects/capabilities.g4
 *
 *    Owns capability requirements attached to EFFECTS.
 *
 *
 * 3. THIS FILE
 *
 *    Owns SECURITY AUTHORITY CAPABILITIES.
 *
 *    A security capability represents an authority-bearing security
 *    abstraction that can be associated with:
 *
 *        - a principal;
 *        - one or more permissions;
 *        - actions;
 *        - resources;
 *        - scopes;
 *        - security requirements;
 *        - conditions;
 *        - issuers;
 *        - delegation properties;
 *        - attenuation properties.
 *
 *    It answers:
 *
 *        "What security authority is represented by this capability?"
 *
 * These three concepts MUST NOT be merged.
 *
 * ============================================================================
 * CAPABILITY != PERMISSION
 * ============================================================================
 *
 * A permission describes an authorization operation or authority.
 *
 * A security capability describes a bearer/delegable/attenuable authority
 * abstraction that may convey one or more permissions under explicit
 * conditions.
 *
 * A security capability is NOT itself:
 *
 *     - a password;
 *     - a private key;
 *     - a bearer token;
 *     - a secret;
 *     - a cryptographic key;
 *     - an authenticated identity;
 *     - proof of authorization;
 *     - runtime authorization state.
 *
 * Actual credentials and runtime authority belong downstream.
 *
 * ============================================================================
 * SECURITY MODEL
 * ============================================================================
 *
 * The conceptual relationship is:
 *
 *     Principal
 *         |
 *         v
 *     Security Capability
 *         |
 *         +----> Permission
 *         |
 *         +----> Action
 *         |
 *         +----> Resource
 *         |
 *         +----> Scope
 *         |
 *         +----> Conditions
 *         |
 *         +----> Requirements
 *         |
 *         v
 *     Security Analysis
 *         |
 *         v
 *     Authorization / Capability Evaluation
 *         |
 *         v
 *     Runtime Enforcement
 *
 * This grammar stops at syntax.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Security capabilities MUST remain independent of physical machine scale.
 *
 * This grammar MUST NOT encode:
 *
 *     MAX_CAPABILITIES
 *     MAX_PRINCIPALS
 *     MAX_PERMISSIONS
 *     MAX_RESOURCES
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * Nor may it encode:
 *
 *     device IDs
 *     CPU IDs
 *     GPU IDs
 *     QPU IDs
 *     hardware addresses
 *     fixed node counts
 *     fixed topology
 *     fixed provider
 *     fixed backend
 *
 * Security authority remains symbolic and resource-parametric.
 *
 * ============================================================================
 * UNIVERSAL COMPUTING BOUNDARY
 * ============================================================================
 *
 * A security capability may protect:
 *
 *     classical computation
 *     quantum computation
 *     HDL/hardware operations
 *     accelerator operations
 *     AI/ML operations
 *     data operations
 *     network operations
 *     distributed operations
 *     storage operations
 *     compilation
 *     deployment
 *     future computational domains
 *
 * The grammar MUST NOT create separate closed capability vocabularies for
 * CPU, GPU, FPGA, QPU, ASIC, cloud, etc.
 *
 * New domains are represented by ordinary qualified names.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Security capabilities may protect quantum operations.
 *
 * For example:
 *
 *     capability security::quantum_execution {
 *         permission quantum::execute;
 *         action quantum::execute;
 *         resource quantum::program;
 *     }
 *
 * This grammar MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     quantum topology
 *     calibration
 *     QEC codes
 *     ZQN fault models
 *
 * Quantum semantics remain downstream.
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 * HARDWARE BOUNDARY
 * ============================================================================
 *
 * A security capability may protect an abstract hardware operation:
 *
 *     capability security::hardware_configuration {
 *         permission hardware::configure;
 *         action hardware::configure;
 *         resource hardware::resource;
 *     }
 *
 * It MUST NOT select a physical device.
 *
 * ============================================================================
 * NO SECRET MATERIAL
 * ============================================================================
 *
 * This grammar MUST NOT introduce syntax whose purpose is to embed:
 *
 *     passwords
 *     private keys
 *     secret keys
 *     authentication tokens
 *     session tokens
 *     API keys
 *     bearer credentials
 *     recovery secrets
 *
 * Source may reference a credential or security object symbolically where
 * required by another security subsystem.
 *
 * Secret material belongs to secure credential/key-management systems.
 *
 * ============================================================================
 * OPEN-WORLD SECURITY
 * ============================================================================
 *
 * Capability names, permission names, action names, resource names,
 * principals, issuers and security domains are open-ended.
 *
 * Examples:
 *
 *     security::data::read
 *     security::quantum::execute
 *     security::hardware::configure
 *     security::network::admin
 *     future::security::new_authority
 *
 * No closed enumeration is permitted here.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - security capability declaration syntax;
 *     - security capability references;
 *     - capability authority subjects;
 *     - capability permission associations;
 *     - capability action associations;
 *     - capability resource associations;
 *     - capability scope associations;
 *     - capability issuer associations;
 *     - capability conditions;
 *     - capability security requirements;
 *     - delegation syntax;
 *     - attenuation syntax;
 *     - security capability metadata;
 *     - source-level security capability attributes.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - general capability declarations;
 *     - general capability versions;
 *     - effect capability requirements;
 *     - permission policy evaluation;
 *     - authentication;
 *     - identity verification;
 *     - credential verification;
 *     - cryptography;
 *     - key management;
 *     - trust establishment;
 *     - resource discovery;
 *     - hardware discovery;
 *     - target selection;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - quantum::ir;
 *     - runtime authorization;
 *     - security logging implementation.
 *
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 *
 * Canonical dependencies:
 *
 *     ZamaniLexer
 *     Core
 *     Types
 *     Expressions
 *
 * Semantic dependencies:
 *
 *     frontend AST
 *     security semantic analysis
 *     core capability registry
 *     permission registry
 *     identity/principal resolution
 *     trust analysis
 *     resource analysis
 *
 * This grammar MUST NOT depend directly on:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     scheduler
 *     router
 *     optimizer
 *     runtime
 *     hardware implementation
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     lexer
 *       |
 *       v
 *     Core / Types / Expressions
 *       |
 *       v
 *     SecurityCapabilities
 *       |
 *       v
 *     Security AST
 *       |
 *       v
 *     Name / identity / permission resolution
 *       |
 *       v
 *     Security semantic model
 *       |
 *       +----> capability analysis
 *       +----> permission analysis
 *       +----> trust analysis
 *       +----> resource analysis
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +----> classical IR
 *       +----> quantum::ir
 *       +----> HDL/hardware IR
 *       |
 *       v
 *     compiler / runtime / enforcement
 *
 * This direction MUST NOT be reversed.
 *
 * ============================================================================
 */

parser grammar SecurityCapabilities;

options {
    tokenVocab = ZamaniLexer;
}

import Core, Types, Expressions;


/* ============================================================================
 * 1. SECURITY CAPABILITY DOCUMENT ENTRY
 * ============================================================================
 *
 * Used by grammar-level tests and standalone parser validation.
 *
 * The enclosing Security grammar MUST consume `securityCapabilityDeclaration`
 * rather than this EOF-bearing rule.
 */

securityCapabilitiesFile
    : securityCapabilityDeclaration+
      EOF
    ;


/* ============================================================================
 * 2. SECURITY CAPABILITY DECLARATION
 * ============================================================================
 *
 * Examples:
 *
 *     capability security::data::read;
 *
 *     capability security::data::read {
 *         permission data::read;
 *     }
 *
 *     capability security::quantum::execute {
 *         permission quantum::execute;
 *         principal service::quantum_runner;
 *         action quantum::execute;
 *         resource quantum::program;
 *     }
 *
 * The declaration introduces source-level security authority semantics.
 *
 * It does NOT create a runtime credential.
 */

securityCapabilityDeclaration
    : attributes*
      visibility?
      CAPABILITY
      qualifiedName
      genericParameters?
      securityCapabilityInheritanceClause?
      securityCapabilityBody?
      SEMI?
    ;


/* ============================================================================
 * 3. INHERITANCE / COMPOSITION
 * ============================================================================
 *
 * Capability inheritance describes source-level composition.
 *
 * It does not imply runtime credential inheritance.
 */

securityCapabilityInheritanceClause
    : EXTENDS
      securityCapabilityReferenceList
    ;


securityCapabilityReferenceList
    : securityCapabilityReference
      (COMMA securityCapabilityReference)*
    ;


/* ============================================================================
 * 4. CAPABILITY BODY
 * ============================================================================
 */

securityCapabilityBody
    : LBRACE
      securityCapabilityMember*
      RBRACE
    ;


securityCapabilityMember
    : securityCapabilityPermission
    | securityCapabilityPrincipal
    | securityCapabilityAction
    | securityCapabilityResource
    | securityCapabilityScope
    | securityCapabilityIssuer
    | securityCapabilityCondition
    | securityCapabilityRequirement
    | securityCapabilityDelegation
    | securityCapabilityAttenuation
    | securityCapabilityProperty
    ;


/* ============================================================================
 * 5. SECURITY CAPABILITY REFERENCES
 * ============================================================================
 *
 * This is deliberately distinct from the language-wide capabilityReference
 * owned by grammar/core/capabilities.g4.
 *
 * This reference identifies a SECURITY AUTHORITY capability.
 */

securityCapabilityReference
    : qualifiedName
    ;


/* ============================================================================
 * 6. PERMISSION ASSOCIATION
 * ============================================================================
 *
 * A security capability may convey one or more named permissions.
 *
 * The permission itself remains owned by grammar/security/permissions.g4.
 *
 * No permission semantics are duplicated here.
 */

securityCapabilityPermission
    : PERMISSION
      permissionReferenceList
      SEMI
    ;


permissionReferenceList
    : permissionReference
      (COMMA permissionReference)*
    ;


permissionReference
    : qualifiedName
    ;


/* ============================================================================
 * 7. PRINCIPAL ASSOCIATION
 * ============================================================================
 *
 * Identifies the subject/principal to which the authority applies.
 *
 * Principal identity semantics remain owned by the security identity
 * subsystem.
 */

securityCapabilityPrincipal
    : PRINCIPAL
      securityPrincipalSelectorList
      SEMI
    ;


securityPrincipalSelectorList
    : securityPrincipalSelector
      (COMMA securityPrincipalSelector)*
    ;


securityPrincipalSelector
    : qualifiedName
    | expression
    ;


/* ============================================================================
 * 8. ACTION ASSOCIATION
 * ============================================================================
 *
 * Actions are open-world semantic names.
 *
 * No CPU/GPU/QPU/FPGA action list is hard-coded.
 */

securityCapabilityAction
    : ACTION
      securityActionSelectorList
      SEMI
    ;


securityActionSelectorList
    : securityActionSelector
      (COMMA securityActionSelector)*
    ;


securityActionSelector
    : qualifiedName
    | expression
    ;


/* ============================================================================
 * 9. RESOURCE ASSOCIATION
 * ============================================================================
 *
 * Resources remain abstract.
 *
 * This grammar does not allocate or discover resources.
 */

securityCapabilityResource
    : RESOURCE
      securityResourceSelectorList
      SEMI
    ;


securityResourceSelectorList
    : securityResourceSelector
      (COMMA securityResourceSelector)*
    ;


securityResourceSelector
    : qualifiedName
    | expression
    ;


/* ============================================================================
 * 10. SCOPE
 * ============================================================================
 *
 * Scope limits where or under which circumstances the capability applies.
 *
 * Scope is an expression, not a physical deployment topology.
 */

securityCapabilityScope
    : SCOPE
      expression
      SEMI
    ;


/* ============================================================================
 * 11. ISSUER
 * ============================================================================
 *
 * Issuer is a symbolic security principal/authority reference.
 *
 * It does not perform authentication.
 */

securityCapabilityIssuer
    : ISSUER
      qualifiedName
      SEMI
    ;


/* ============================================================================
 * 12. CONDITIONS
 * ============================================================================
 *
 * Conditions are parsed, not evaluated.
 *
 * Examples:
 *
 *     when context::environment == "production";
 *
 *     when context::trust >= required::level;
 *
 *     when request::purpose == "research";
 */

securityCapabilityCondition
    : WHEN
      expression
      SEMI
    ;


/* ============================================================================
 * 13. SECURITY REQUIREMENTS
 * ============================================================================
 *
 * A security capability may require another security property or a
 * language-wide capability.
 *
 * IMPORTANT:
 *
 *     requirement != capability declaration
 *     requirement != authorization decision
 *     requirement != runtime verification
 */

securityCapabilityRequirement
    : REQUIRES
      securityCapabilityRequirementExpression
      SEMI
    ;


securityCapabilityRequirementExpression
    : securityCapabilityRequirementTerm
    | securityCapabilityRequirementAll
    | securityCapabilityRequirementAny
    | securityCapabilityRequirementNot
    ;


securityCapabilityRequirementTerm
    : securityCapabilityReference
    | capabilityReference
    | permissionReference
    | expression
    ;


securityCapabilityRequirementAll
    : LBRACE
      securityCapabilityRequirementExpression
      (COMMA securityCapabilityRequirementExpression)*
      COMMA?
      RBRACE
    ;


securityCapabilityRequirementAny
    : LPAREN
      securityCapabilityRequirementExpression
      (OR securityCapabilityRequirementExpression)+
      RPAREN
    ;


securityCapabilityRequirementNot
    : NOT
      securityCapabilityRequirementExpression
    ;


/* ============================================================================
 * 14. DELEGATION
 * ============================================================================
 *
 * Delegation describes whether the authority represented by a capability may
 * be delegated.
 *
 * This is declarative syntax only.
 *
 * It does not create a credential or perform delegation.
 */

securityCapabilityDelegation
    : DELEGATE
      securityDelegationSpecification
      SEMI
    ;


securityDelegationSpecification
    : securityDelegationFlag
    | securityDelegationBody
    ;


securityDelegationFlag
    : identifier
    ;


securityDelegationBody
    : LBRACE
      securityDelegationMember*
      RBRACE
    ;


securityDelegationMember
    : securityDelegationProperty
    | securityDelegationConstraint
    ;


securityDelegationProperty
    : identifier
      (ASSIGN expression)?
      SEMI
    ;


securityDelegationConstraint
    : CONSTRAINT
      expression
      SEMI
    ;


/* ============================================================================
 * 15. ATTENUATION
 * ============================================================================
 *
 * Attenuation allows a derived authority to be restricted relative to the
 * source authority.
 *
 * This grammar only records the source-level relationship.
 *
 * Semantic analysis MUST verify that attenuation never expands authority.
 */

securityCapabilityAttenuation
    : ATTENUATE
      securityAttenuationSpecification
      SEMI
    ;


securityAttenuationSpecification
    : securityAttenuationBody
    | expression
    ;


securityAttenuationBody
    : LBRACE
      securityAttenuationMember*
      RBRACE
    ;


securityAttenuationMember
    : securityAttenuationPermission
    | securityAttenuationAction
    | securityAttenuationResource
    | securityAttenuationCondition
    | securityAttenuationProperty
    ;


securityAttenuationPermission
    : PERMISSION
      permissionReferenceList
      SEMI
    ;


securityAttenuationAction
    : ACTION
      securityActionSelectorList
      SEMI
    ;


securityAttenuationResource
    : RESOURCE
      securityResourceSelectorList
      SEMI
    ;


securityAttenuationCondition
    : WHEN
      expression
      SEMI
    ;


securityAttenuationProperty
    : identifier
      (ASSIGN expression)?
      SEMI
    ;


/* ============================================================================
 * 16. GENERIC SECURITY CAPABILITY PROPERTY
 * ============================================================================
 *
 * Extension points must remain open.
 *
 * New security metadata must not require a grammar rewrite when it can be
 * represented as an ordinary typed property.
 */

securityCapabilityProperty
    : identifier
      (COLON typeExpression)?
      (ASSIGN expression)?
      SEMI
    ;


/* ============================================================================
 * 17. CAPABILITY LIST
 * ============================================================================
 *
 * No finite maximum.
 */

securityCapabilityReferenceListExpression
    : securityCapabilityReference
      (COMMA securityCapabilityReference)*
    ;


/* ============================================================================
 * 18. SECURITY CAPABILITY SET
 * ============================================================================
 *
 * Used by future semantic composition layers and extension grammars.
 */

securityCapabilitySet
    : LBRACE
      securityCapabilitySetMember*
      RBRACE
    ;


securityCapabilitySetMember
    : securityCapabilityReference
      COMMA?
    ;


/* ============================================================================
 * 19. SECURITY CAPABILITY EXPRESSION
 * ============================================================================
 *
 * Generic compositional expression.
 *
 * The semantic layer determines:
 *
 *     ALL
 *     ANY
 *     NOT
 *     equivalence
 *     conflict
 *     attenuation
 *     inheritance
 *
 * No authorization decision occurs here.
 */

securityCapabilityExpression
    : securityCapabilityReference
    | securityCapabilityExpressionGroup
    | securityCapabilityExpressionNot
    | securityCapabilityExpressionBinary
    ;


securityCapabilityExpressionGroup
    : LPAREN
      securityCapabilityExpression
      RPAREN
    ;


securityCapabilityExpressionNot
    : NOT
      securityCapabilityExpression
    ;


securityCapabilityExpressionBinary
    : securityCapabilityExpression
      capabilityExpressionOperator
      securityCapabilityExpression
    ;


capabilityExpressionOperator
    : AND
    | OR
    ;


/* ============================================================================
 * 20. VALIDATION BOUNDARY
 * ============================================================================
 *
 * The grammar intentionally does NOT enforce the following semantic rules:
 *
 *     - referenced principal exists;
 *     - referenced permission exists;
 *     - referenced resource exists;
 *     - referenced action exists;
 *     - issuer is trusted;
 *     - issuer is authorized to delegate;
 *     - capability exists in the registry;
 *     - capability version is compatible;
 *     - delegation is legal;
 *     - attenuation is monotonic;
 *     - capability is non-escalating;
 *     - capability satisfies policy;
 *     - capability satisfies runtime requirements;
 *     - capability is available on the selected target.
 *
 * Those belong to semantic/security analysis.
 *
 * ============================================================================
 */