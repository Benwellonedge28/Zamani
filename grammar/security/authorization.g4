/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/security/authorization.g4
 *
 * GRAMMAR
 * -------
 * Authorization
 *
 * STATUS
 * ------
 * Production security authorization grammar
 *
 * PURPOSE
 * -------
 * This file defines the canonical SOURCE-LEVEL AUTHORIZATION syntax for
 * Zamani.
 *
 * Authorization describes:
 *
 *   - who/what may act;
 *   - what action is authorized;
 *   - upon which resource;
 *   - under which conditions;
 *   - for which scope;
 *   - with which obligations;
 *   - through which delegation relationship;
 *   - with which temporal/contextual restrictions.
 *
 * This file describes AUTHORIZATION INTENT.
 *
 * It does NOT perform authorization.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                       canonical lexer
 *                              |
 *                              v
 *                    canonical parser hierarchy
 *                              |
 *                              v
 *                 +----------------------------+
 *                 | authorization.g4           |
 *                 | THIS FILE                  |
 *                 +----------------------------+
 *                              |
 *                              v
 *                       frontend AST
 *                              |
 *                              v
 *                    security semantic analysis
 *                              |
 *                 +------------+-------------+
 *                 |            |             |
 *                 v            v             v
 *             identity     capability     resource
 *             analysis     analysis       analysis
 *                 |            |             |
 *                 +------------+-------------+
 *                              |
 *                              v
 *                    authorization semantics
 *                              |
 *                              v
 *                    canonical semantic model
 *                              |
 *              +---------------+----------------+
 *              |               |                |
 *              v               v                v
 *        classical         quantum::ir     HDL/hardware
 *                              |
 *                              v
 *                    optimization / lowering
 *                              |
 *                              v
 *                    runtime authorization
 *                              |
 *                              v
 *                       actual enforcement
 *
 * This grammar terminates at syntax.
 *
 * ============================================================================
 * RUST / SAFETY BASELINE
 * ============================================================================
 *
 * Target implementation:
 *
 *   Rust 1.97
 *   Rust 1.97.1
 *   Rust 2021
 *
 * Safety requirements:
 *
 *   - no embedded Rust;
 *   - no embedded target-language actions;
 *   - no semantic predicates;
 *   - no filesystem access;
 *   - no network access;
 *   - no environment inspection;
 *   - no hardware discovery;
 *   - no capability discovery;
 *   - no runtime execution;
 *   - no secret access;
 *   - no unsafe implementation requirement.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *   - authorization declarations;
 *   - authorization policies;
 *   - authorization rules;
 *   - allow decisions;
 *   - deny decisions;
 *   - grant relationships;
 *   - revoke relationships;
 *   - delegation relationships;
 *   - attenuation declarations;
 *   - authorization subjects;
 *   - authorization actions;
 *   - authorization resources;
 *   - authorization conditions;
 *   - authorization scopes;
 *   - authorization obligations;
 *   - authorization constraints;
 *   - authorization metadata;
 *   - source-level authorization requirements.
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *   - lexical token definitions;
 *   - identifiers;
 *   - qualified names;
 *   - authentication;
 *   - identity verification;
 *   - credential storage;
 *   - secret material;
 *   - cryptographic implementation;
 *   - trust evaluation;
 *   - generic capability declarations;
 *   - security capability declarations;
 *   - resource discovery;
 *   - resource allocation;
 *   - hardware discovery;
 *   - device selection;
 *   - quantum semantics;
 *   - quantum::ir;
 *   - QEC;
 *   - ZQN;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - runtime enforcement.
 *
 * ============================================================================
 * AUTHENTICATION / AUTHORIZATION SEPARATION
 * ============================================================================
 *
 * Authentication answers:
 *
 *     "Who or what is this?"
 *
 * Authorization answers:
 *
 *     "May this principal perform this action on this resource?"
 *
 * This grammar owns only the second question.
 *
 * Identity syntax belongs to:
 *
 *     grammar/security/identifiers.g4
 *
 * Authentication belongs to downstream security infrastructure.
 *
 * ============================================================================
 * PERMISSION / CAPABILITY / AUTHORIZATION SEPARATION
 * ============================================================================
 *
 * Permission:
 *
 *     an abstract authorization operation or authority.
 *
 * Capability:
 *
 *     an authority-bearing semantic capability.
 *
 * Authorization:
 *
 *     a relationship determining whether a subject may perform an action
 *     against a resource under specified conditions.
 *
 * These concepts MUST NOT collapse into one grammar object.
 *
 * ============================================================================
 * OPEN-WORLD SECURITY
 * ============================================================================
 *
 * Zamani authorization deliberately does not enumerate a finite set of:
 *
 *     users
 *     roles
 *     actions
 *     resources
 *     permissions
 *     policies
 *     providers
 *     devices
 *     algorithms
 *     security mechanisms
 *     domains
 *
 * Names are represented through the canonical name system.
 *
 * Examples:
 *
 *     data::read
 *     quantum::execute
 *     hardware::configure
 *     network::connect
 *     future::security::operation
 *
 * New authorization concepts therefore do not require modification of this
 * grammar merely because a new domain appears.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Authorization syntax MUST remain independent of target hardware.
 *
 * This grammar imposes no language-level limits on:
 *
 *     - subjects;
 *     - identities;
 *     - roles;
 *     - permissions;
 *     - capabilities;
 *     - actions;
 *     - resources;
 *     - policies;
 *     - rules;
 *     - conditions;
 *     - obligations;
 *     - delegations;
 *     - scopes;
 *     - domains;
 *     - machines;
 *     - nodes;
 *     - CPUs;
 *     - GPUs;
 *     - FPGAs;
 *     - QPUs;
 *     - qubits;
 *     - memory;
 *     - storage;
 *     - network endpoints.
 *
 * There are deliberately no constructs such as:
 *
 *     MAX_USERS
 *     MAX_ROLES
 *     MAX_PERMISSIONS
 *     MAX_POLICIES
 *     MAX_RULES
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_MEMORY
 *
 * Authorization remains scalable according to the resources and security
 * infrastructure available to the execution environment.
 *
 * ============================================================================
 * PORTABILITY RULE
 * ============================================================================
 *
 * Portable authorization expresses WHAT MUST BE AUTHORIZED.
 *
 * It must not unnecessarily encode:
 *
 *     WHICH MACHINE
 *     WHICH DEVICE
 *     WHICH CPU
 *     WHICH GPU
 *     WHICH QPU
 *     WHICH FPGA
 *     WHICH PROVIDER
 *     WHICH NETWORK
 *
 * Example:
 *
 *     allow subject principal::operator
 *           action quantum::execute
 *           resource quantum::program;
 *
 * expresses authorization intent.
 *
 * It does not select a physical QPU.
 *
 * ============================================================================
 * TARGET-SPECIFIC INFORMATION
 * ============================================================================
 *
 * Target-specific information may appear only when explicitly part of source
 * semantics and must remain represented as a symbolic expression.
 *
 * Authorization syntax must not create special grammar rules for:
 *
 *     AWS
 *     Azure
 *     GCP
 *     CUDA
 *     ROCm
 *     FPGA vendors
 *     QPU vendors
 *     CPU vendors
 *
 * Vendor-specific meaning is resolved downstream through ordinary names,
 * capabilities, dialects, or interoperability layers.
 *
 * ============================================================================
 * SECRET-MATERIAL BOUNDARY
 * ============================================================================
 *
 * This grammar MUST NOT contain syntax for embedding secret material.
 *
 * Forbidden as language-level authorization data:
 *
 *     passwords
 *     private keys
 *     secret keys
 *     bearer tokens
 *     session tokens
 *     API secrets
 *     recovery secrets
 *     authentication credentials
 *
 * Symbolic references are permitted:
 *
 *     credential::runtime_identity
 *     key::application_signing
 *     secret::deployment_reference
 *
 * Actual secret material belongs to secure runtime/key-management systems.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no semantic predicates;
 *     - no actions;
 *     - no runtime calls;
 *     - no environment queries;
 *     - no hardware queries;
 *     - no random behavior.
 *
 * Identical source/token streams must produce identical parse structures.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve:
 *
 *     - source spans;
 *     - declaration ordering;
 *     - authorization decision;
 *     - subject structure;
 *     - action structure;
 *     - resource structure;
 *     - condition structure;
 *     - scope structure;
 *     - obligation structure;
 *     - delegation structure;
 *     - attenuation structure;
 *     - explicit metadata.
 *
 * Recommended semantic AST concepts:
 *
 *     AuthorizationDeclaration
 *     AuthorizationPolicy
 *     AuthorizationRule
 *     AuthorizationDecision
 *     AuthorizationSubject
 *     AuthorizationAction
 *     AuthorizationResource
 *     AuthorizationCondition
 *     AuthorizationScope
 *     AuthorizationObligation
 *     AuthorizationGrant
 *     AuthorizationRevocation
 *     AuthorizationDelegation
 *     AuthorizationAttenuation
 *
 * These are semantic concepts.
 *
 * The grammar itself does not construct them.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving subjects;
 *     - resolving actions;
 *     - resolving resources;
 *     - resolving permissions;
 *     - resolving capabilities;
 *     - validating references;
 *     - detecting policy conflicts;
 *     - validating delegation;
 *     - validating attenuation;
 *     - checking scope compatibility;
 *     - checking condition types;
 *     - checking obligation compatibility;
 *     - checking security requirements;
 *     - checking target capability satisfaction.
 *
 * None of those decisions occur in this grammar.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Authorization may reference:
 *
 *     resources/*
 *     core/capabilities.g4
 *     security/capabilities.g4
 *
 * but it does not own resource allocation or capability discovery.
 *
 * Example:
 *
 *     requires capability quantum::measurement;
 *
 * is a requirement.
 *
 * It does not select a physical device.
 *
 * ============================================================================
 * IDENTITY INTEGRATION
 * ============================================================================
 *
 * Subject references may refer to entities declared by:
 *
 *     grammar/security/identifiers.g4
 *
 * The authorization grammar does not redefine identity syntax.
 *
 * ============================================================================
 * PERMISSION INTEGRATION
 * ============================================================================
 *
 * Permission declaration/reference syntax remains owned by:
 *
 *     grammar/security/permissions.g4
 *
 * This file references permissions but does not redefine permission
 * declaration syntax.
 *
 * ============================================================================
 * SECURITY CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Security authority capabilities remain owned by:
 *
 *     grammar/security/capabilities.g4
 *
 * Generic computational capabilities remain owned by:
 *
 *     grammar/core/capabilities.g4
 *
 * Authorization may reference either through canonical qualified names.
 *
 * ============================================================================
 * TRUST INTEGRATION
 * ============================================================================
 *
 * Trust relationships belong to:
 *
 *     grammar/security/trust.g4
 *
 * Authorization may require or reference trust conditions, but does not
 * evaluate trust.
 *
 * ============================================================================
 * CRYPTOGRAPHY INTEGRATION
 * ============================================================================
 *
 * Cryptographic requirements belong to:
 *
 *     grammar/security/cryptography.g4
 *
 * Authorization may reference a cryptographic requirement or property but
 * does not implement cryptography.
 *
 * ============================================================================
 * PRIVACY INTEGRATION
 * ============================================================================
 *
 * Privacy policy belongs to:
 *
 *     grammar/security/privacy.g4
 *
 * Authorization may attach privacy-related conditions or requirements without
 * duplicating privacy grammar.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Authorization can constrain operations whose semantic effects require
 * authorization.
 *
 * Effects remain owned by:
 *
 *     grammar/effects/
 *
 * Authorization does not become an effect system.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Authorization may protect:
 *
 *     quantum::execute
 *     quantum::measure
 *     quantum::compile
 *     quantum::deploy
 *
 * or future symbolic operations.
 *
 * It MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     topology
 *     calibration
 *     QEC
 *     ZQN
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * Authorization metadata may accompany canonical quantum semantics.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Authorization may protect abstract operations such as:
 *
 *     hardware::configure
 *     hardware::synthesize
 *     hardware::program
 *     accelerator::execute
 *
 * It must not encode physical hardware identities as the fundamental
 * authorization model.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Authorization may protect:
 *
 *     distributed::execute
 *     distributed::communicate
 *     distributed::deploy
 *     distributed::replicate
 *
 * without imposing any node-count limit.
 *
 * ============================================================================
 * NETWORKING INTEGRATION
 * ============================================================================
 *
 * Authorization may protect:
 *
 *     network::connect
 *     network::listen
 *     network::send
 *     network::receive
 *     network::route
 *
 * Endpoint resolution remains downstream.
 *
 * ============================================================================
 * CORE SYNTAX CONTRACT
 * ============================================================================
 *
 * The following are expected to be supplied by canonical shared grammars:
 *
 *     identifier
 *     qualifiedName
 *     attributes
 *     visibility
 *     genericParameters
 *     expression
 *     argumentList
 *
 * This file MUST NOT create competing definitions.
 *
 * ============================================================================
 * AUTHORIZATION DECLARATION
 * ============================================================================
 *
 * Canonical forms include:
 *
 *     authorization policy::data_access {
 *         ...
 *     }
 *
 *     authorization quantum::execution {
 *         ...
 *     }
 *
 * A declaration may contain metadata and rules.
 */

authorizationDeclaration
    : attributes?
      visibility?
      AUTHORIZATION
      qualifiedName
      genericParameters?
      authorizationBody
      SEMI?
    ;


/*
 * ============================================================================
 * AUTHORIZATION BODY
 * ============================================================================
 */

authorizationBody
    : LBRACE
      authorizationMember*
      RBRACE
    ;


authorizationMember
    : authorizationRule
    | authorizationGrant
    | authorizationRevocation
    | authorizationDelegation
    | authorizationAttenuation
    | authorizationRequirement
    | authorizationMetadata
    ;


/*
 * ============================================================================
 * AUTHORIZATION POLICY
 * ============================================================================
 *
 * A policy is a named collection of authorization semantics.
 *
 * Policy evaluation is downstream.
 */

authorizationPolicyDeclaration
    : attributes?
      visibility?
      POLICY
      qualifiedName
      genericParameters?
      policyBody
      SEMI?
    ;


policyBody
    : LBRACE
      policyMember*
      RBRACE
    ;


policyMember
    : authorizationRule
    | authorizationGrant
    | authorizationRevocation
    | authorizationDelegation
    | authorizationAttenuation
    | authorizationRequirement
    | authorizationMetadata
    ;


/*
 * ============================================================================
 * AUTHORIZATION RULE
 * ============================================================================
 *
 * Canonical form:
 *
 *     allow subject <subject>
 *          action <action>
 *          resource <resource>;
 *
 *     deny subject <subject>
 *         action <action>
 *         resource <resource>
 *         when <condition>;
 *
 * The clauses are deliberately named rather than positionally fixed.
 *
 * This permits future authorization dimensions without encoding a closed
 * security model.
 */

authorizationRule
    : authorizationDecision
      authorizationSubjectClause?
      authorizationActionClause?
      authorizationResourceClause?
      authorizationPermissionClause?
      authorizationScopeClause?
      authorizationConditionClause?
      authorizationObligationClause*
      SEMI
    ;


authorizationDecision
    : ALLOW
    | DENY
    | REQUIRE
    | REJECT
    ;


/*
 * ============================================================================
 * SUBJECT
 * ============================================================================
 */

authorizationSubjectClause
    : SUBJECT
      authorizationSubject
    ;


authorizationSubject
    : authorizationSubjectReference
    | authorizationSubjectExpression
    ;


authorizationSubjectReference
    : qualifiedName
    ;


authorizationSubjectExpression
    : authorizationSubjectReference
    | authorizationSubjectCall
    | authorizationSubjectSet
    | expression
    ;


authorizationSubjectCall
    : qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


authorizationSubjectSet
    : LBRACE
      authorizationSubjectItem*
      RBRACE
    ;


authorizationSubjectItem
    : authorizationSubjectExpression
      COMMA?
    ;


/*
 * ============================================================================
 * ACTION
 * ============================================================================
 *
 * Actions are open-world symbolic names.
 *
 * No finite action enumeration exists.
 */

authorizationActionClause
    : ACTION
      authorizationAction
    ;


authorizationAction
    : authorizationActionReference
    | authorizationActionExpression
    ;


authorizationActionReference
    : qualifiedName
    ;


authorizationActionExpression
    : authorizationActionReference
    | authorizationActionCall
    | expression
    ;


authorizationActionCall
    : qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * RESOURCE
 * ============================================================================
 *
 * Resources are semantic resources, not necessarily physical resources.
 */

authorizationResourceClause
    : RESOURCE
      authorizationResource
    ;


authorizationResource
    : authorizationResourceReference
    | authorizationResourceExpression
    ;


authorizationResourceReference
    : qualifiedName
    ;


authorizationResourceExpression
    : authorizationResourceReference
    | authorizationResourceCall
    | authorizationResourceSet
    | expression
    ;


authorizationResourceCall
    : qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


authorizationResourceSet
    : LBRACE
      authorizationResourceItem*
      RBRACE
    ;


authorizationResourceItem
    : authorizationResourceExpression
      COMMA?
    ;


/*
 * ============================================================================
 * PERMISSION
 * ============================================================================
 *
 * Permission declarations belong to permissions.g4.
 *
 * This rule only references them.
 */

authorizationPermissionClause
    : PERMISSION
      permissionReference
    ;


permissionReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * SCOPE
 * ============================================================================
 *
 * Scope is semantic authorization scope.
 *
 * It is not a machine topology.
 */

authorizationScopeClause
    : SCOPE
      authorizationScope
    ;


authorizationScope
    : authorizationScopeReference
    | authorizationScopeExpression
    ;


authorizationScopeReference
    : qualifiedName
    ;


authorizationScopeExpression
    : authorizationScopeReference
    | authorizationScopeCall
    | authorizationScopeSet
    | expression
    ;


authorizationScopeCall
    : qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


authorizationScopeSet
    : LBRACE
      authorizationScopeItem*
      RBRACE
    ;


authorizationScopeItem
    : authorizationScopeExpression
      COMMA?
    ;


/*
 * ============================================================================
 * CONDITION
 * ============================================================================
 *
 * Conditions are source expressions.
 *
 * They are not evaluated by the parser.
 */

authorizationConditionClause
    : WHEN
      expression
    ;


/*
 * ============================================================================
 * OBLIGATIONS
 * ============================================================================
 *
 * Obligations describe semantic actions that may accompany authorization.
 *
 * Runtime enforcement remains downstream.
 */

authorizationObligationClause
    : OBLIGE
      authorizationObligation
    ;


authorizationObligation
    : authorizationObligationReference
    | authorizationObligationExpression
    ;


authorizationObligationReference
    : qualifiedName
    ;


authorizationObligationExpression
    : authorizationObligationReference
    | authorizationObligationCall
    | expression
    ;


authorizationObligationCall
    : qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


authorizationObligation
    : authorizationObligationReference
    | authorizationObligationExpression
    ;


/*
 * ============================================================================
 * GRANTS
 * ============================================================================
 *
 * A grant creates an explicit authorization relationship.
 */

authorizationGrant
    : GRANT
      authorizationGrantSubjectClause?
      authorizationGrantPermissionClause?
      authorizationGrantActionClause?
      authorizationGrantResourceClause?
      authorizationScopeClause?
      authorizationConditionClause?
      authorizationObligationClause*
      SEMI
    ;


authorizationGrantSubjectClause
    : TO
      authorizationSubject
    ;


authorizationGrantPermissionClause
    : PERMISSION
      permissionReference
    ;


authorizationGrantActionClause
    : ACTION
      authorizationAction
    ;


authorizationGrantResourceClause
    : ON
      authorizationResource
    ;


/*
 * ============================================================================
 * REVOCATION
 * ============================================================================
 *
 * Revocation is declarative source intent.
 *
 * Runtime authority revocation is downstream.
 */

authorizationRevocation
    : REVOKE
      authorizationRevokeSubjectClause?
      authorizationRevokePermissionClause?
      authorizationRevokeActionClause?
      authorizationRevokeResourceClause?
      authorizationScopeClause?
      authorizationConditionClause?
      SEMI
    ;


authorizationRevokeSubjectClause
    : FROM
      authorizationSubject
    ;


authorizationRevokePermissionClause
    : PERMISSION
      permissionReference
    ;


authorizationRevokeActionClause
    : ACTION
      authorizationAction
    ;


authorizationRevokeResourceClause
    : ON
      authorizationResource
    ;


/*
 * ============================================================================
 * DELEGATION
 * ============================================================================
 *
 * Delegation transfers a bounded authority relationship.
 *
 * The semantic analyzer must enforce:
 *
 *     - delegation authority;
 *     - scope;
 *     - attenuation;
 *     - lifetime;
 *     - revocation semantics;
 *     - trust requirements.
 *
 * None of those are performed by this grammar.
 */

authorizationDelegation
    : DELEGATE
      authorizationDelegatorClause?
      authorizationDelegateeClause
      authorizationPermissionClause?
      authorizationActionClause?
      authorizationResourceClause?
      authorizationScopeClause?
      authorizationConditionClause?
      authorizationObligationClause*
      SEMI
    ;


authorizationDelegatorClause
    : FROM
      authorizationSubject
    ;


authorizationDelegateeClause
    : TO
      authorizationSubject
    ;


/*
 * ============================================================================
 * ATTENUATION
 * ============================================================================
 *
 * Attenuation narrows an existing authorization authority.
 *
 * It must never silently expand authority.
 *
 * Semantic validation owns that invariant.
 */

authorizationAttenuation
    : ATTENUATE
      authorizationAuthorityReference
      authorizationAttenuationClause*
      SEMI
    ;


authorizationAuthorityReference
    : qualifiedName
    ;


authorizationAttenuationClause
    : authorizationPermissionClause
    | authorizationActionClause
    | authorizationResourceClause
    | authorizationScopeClause
    | authorizationConditionClause
    | authorizationObligationClause
    | authorizationMetadata
    ;


/*
 * ============================================================================
 * REQUIREMENTS
 * ============================================================================
 *
 * Authorization requirements describe mandatory security intent.
 *
 * They are not authorization decisions.
 */

authorizationRequirement
    : REQUIRES
      authorizationRequirementExpression
      SEMI
    ;


authorizationRequirementExpression
    : authorizationPermissionRequirement
    | authorizationCapabilityRequirement
    | authorizationTrustRequirement
    | authorizationConditionRequirement
    | expression
    ;


authorizationPermissionRequirement
    : PERMISSION
      permissionReference
    ;


authorizationCapabilityRequirement
    : CAPABILITY
      qualifiedName
    ;


authorizationTrustRequirement
    : TRUST
      qualifiedName
    ;


authorizationConditionRequirement
    : WHEN
      expression
    ;


/*
 * ============================================================================
 * METADATA
 * ============================================================================
 *
 * Metadata is source data.
 *
 * It is not interpreted by the parser.
 */

authorizationMetadata
    : identifier
      ASSIGN
      expression
      SEMI
    ;


/*
 * ============================================================================
 * COMPACT AUTHORIZATION FORM
 * ============================================================================
 *
 * The compact form allows:
 *
 *     authorize subject::operator
 *       action quantum::execute
 *       resource quantum::program;
 *
 * This is syntactic sugar for an authorization rule.
 *
 * Semantic normalization must produce the same canonical authorization model
 * as the expanded rule form.
 */

authorizationStatement
    : AUTHORIZE
      authorizationSubject
      authorizationActionClause?
      authorizationResourceClause?
      authorizationPermissionClause?
      authorizationScopeClause?
      authorizationConditionClause?
      authorizationObligationClause*
      SEMI
    ;


/*
 * ============================================================================
 * SOURCE-LEVEL POLICY COMPOSITION
 * ============================================================================
 *
 * Named policies may be composed without imposing a fixed policy hierarchy.
 */

authorizationPolicyUse
    : USE
      POLICY
      qualifiedName
      authorizationPolicyArgumentList?
      SEMI
    ;


authorizationPolicyArgumentList
    : LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * SECURITY DOMAIN ATTACHMENT
 * ============================================================================
 *
 * Authorization can be attached to a named security domain.
 */

authorizationDomainDeclaration
    : attributes?
      visibility?
      SECURITY
      qualifiedName
      authorizationDomainBody
      SEMI?
    ;


authorizationDomainBody
    : LBRACE
      authorizationDomainMember*
      RBRACE
    ;


authorizationDomainMember
    : authorizationPolicyDeclaration
    | authorizationDeclaration
    | authorizationRule
    | authorizationGrant
    | authorizationRevocation
    | authorizationDelegation
    | authorizationAttenuation
    | authorizationRequirement
    | authorizationMetadata
    ;


/*
 * ============================================================================
 * CONFORMANCE INVARIANTS
 * ============================================================================
 *
 * The following MUST remain true:
 *
 * 1. No authorization keyword enumerates users, devices, providers or
 *    hardware.
 *
 * 2. No authorization rule introduces physical resource limits.
 *
 * 3. No rule performs authorization.
 *
 * 4. No rule performs authentication.
 *
 * 5. No rule contains secret material.
 *
 * 6. No rule creates a quantum IR.
 *
 * 7. No rule creates a hardware IR.
 *
 * 8. No rule performs capability discovery.
 *
 * 9. No rule performs resource allocation.
 *
 * 10. No rule depends on runtime state.
 *
 * 11. No rule depends on hardware state.
 *
 * 12. No finite authorization vocabulary is required.
 *
 * 13. All source spans must remain recoverable through the parser/AST
 *     integration.
 *
 * 14. Authorization metadata must survive lowering.
 *
 * 15. Mandatory authorization requirements must not be silently discarded
 *     by optimization.
 *
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] authorization has one dedicated grammar owner;
 * [x] authentication remains separate;
 * [x] identity remains separate;
 * [x] permissions remain separate;
 * [x] capabilities remain separate;
 * [x] trust remains separate;
 * [x] resources remain separate;
 * [x] conditions remain expressions;
 * [x] obligations remain semantic intent;
 * [x] authorization is open-world;
 * [x] no machine limits exist;
 * [x] no vendor enumeration exists;
 * [x] no physical hardware identity is required;
 * [x] no secrets are embedded;
 * [x] no semantic predicates exist;
 * [x] no embedded Rust exists;
 * [x] no unsafe requirement exists;
 * [x] deterministic parsing is preserved;
 * [x] AST integration is predetermined;
 * [x] semantic integration is predetermined;
 * [x] quantum integration remains through quantum::ir;
 * [x] hardware integration remains downstream;
 * [x] POCO-REAF remains preserved.
 *
 * Remaining repository-level integration is intentionally performed by the
 * composition/lexer layers rather than duplicated here.
 *
 * ============================================================================
 */