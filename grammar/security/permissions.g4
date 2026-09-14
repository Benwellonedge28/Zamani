/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/security/permissions.g4
 *
 * Role:
 *     Canonical source-level grammar for declarative permissions,
 *     authorization rules, grants, denials, revocations, subjects,
 *     actions, resources, conditions, obligations, and permission
 *     composition.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler/runtime baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     - No embedded Rust.
 *     - No semantic predicates.
 *     - No filesystem access.
 *     - No network access.
 *     - No runtime calls.
 *     - No hardware discovery.
 *     - No unsafe code.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                           lexer
 *                              |
 *                              v
 *                           parser
 *                              |
 *              +---------------+---------------+
 *              |                               |
 *              v                               v
 *       shared language rules          security/permissions
 *                                              |
 *                                              v
 *                                        frontend AST
 *                                              |
 *                  +---------------------------+----------------------+
 *                  |                           |                      |
 *                  v                           v                      v
 *             name resolution            type analysis        security analysis
 *                                                                  |
 *                                                                  v
 *                                                         policy analysis
 *                                                                  |
 *                                                                  v
 *                                                         capability analysis
 *                                                                  |
 *                                                                  v
 *                                                    canonical semantic IR
 *                                                                  |
 *             +----------------------+------------------------------+---------+
 *             |                      |                              |
 *             v                      v                              v
 *        classical IR           quantum::ir                  HDL/hardware IR
 *             |                      |                              |
 *             +----------------------+------------------------------+
 *                                    |
 *                                    v
 *                         optimization / scheduling
 *                                    |
 *                                    v
 *                           target lowering
 *                                    |
 *                                    v
 *                            runtime / hardware
 *
 * This file is syntax only.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - permission declarations;
 *   - permission references;
 *   - authorization rules;
 *   - allow/deny decisions;
 *   - grant declarations;
 *   - revoke declarations;
 *   - subject expressions;
 *   - action expressions;
 *   - resource expressions;
 *   - authorization conditions;
 *   - policy obligations;
 *   - permission inheritance/composition syntax;
 *   - permission requirements;
 *   - authorization metadata;
 *   - source-level permission annotations.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - authentication;
 *   - identity verification;
 *   - credential validation;
 *   - cryptographic algorithms;
 *   - key management;
 *   - trust establishment;
 *   - capability discovery;
 *   - hardware discovery;
 *   - device selection;
 *   - resource allocation;
 *   - network access;
 *   - runtime policy enforcement;
 *   - authorization decisions;
 *   - security logging implementation;
 *   - audit storage;
 *   - quantum semantics;
 *   - QEC;
 *   - ZQN;
 *   - routing;
 *   - scheduling;
 *   - canonical IR.
 *
 * Those responsibilities belong to downstream semantic, security,
 * capability, runtime, and hardware subsystems.
 *
 * ============================================================================
 * OPEN-WORLD SECURITY MODEL
 * ============================================================================
 *
 * This grammar intentionally does NOT enumerate a fixed permission vocabulary.
 *
 * Valid:
 *
 *     permission read_data;
 *     permission quantum_execution;
 *     permission fpga_reconfiguration;
 *     permission future_security_capability;
 *
 * The meaning of those names is resolved downstream.
 *
 * This prevents the grammar from becoming a scalability or evolution
 * bottleneck.
 *
 * No fixed list of:
 *
 *     users
 *     devices
 *     resources
 *     providers
 *     algorithms
 *     hardware
 *     nodes
 *     permissions
 *     actions
 *     roles
 *
 * is encoded here.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Permission syntax describes security intent.
 *
 * It does not permanently select:
 *
 *     - a machine;
 *     - a processor;
 *     - a QPU;
 *     - a GPU;
 *     - an FPGA;
 *     - a provider;
 *     - a network;
 *     - a physical security mechanism.
 *
 * Therefore:
 *
 *     permission quantum_execution;
 *
 * means that the program expresses a security authority named
 * "quantum_execution".
 *
 * It does NOT mean:
 *
 *     use QPU X
 *     use N qubits
 *     use topology Y
 *     use provider Z
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No finite language-level limits are imposed on:
 *
 *     - permissions;
 *     - subjects;
 *     - actions;
 *     - resources;
 *     - rules;
 *     - conditions;
 *     - grants;
 *     - revocations;
 *     - policy declarations;
 *     - obligations;
 *     - inheritance relationships;
 *     - authorization expressions;
 *     - declarations.
 *
 * Repetition is represented using ANTLR repetition operators.
 *
 * Any practical limits are implementation/resource-policy concerns and must
 * never become arbitrary source-language limits.
 *
 * ============================================================================
 * SECURITY PRINCIPLES
 * ============================================================================
 *
 * 1. Authentication != authorization.
 * 2. Permission declaration != permission enforcement.
 * 3. Capability != permission.
 * 4. Identity != principal authorization.
 * 5. Requirement != preference.
 * 6. Policy syntax != policy evaluation.
 * 7. Resource name != physical resource.
 * 8. Action name != implementation.
 * 9. Security intent != cryptographic implementation.
 * 10. Source syntax != runtime state.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     - semantic predicates;
 *     - actions;
 *     - random values;
 *     - runtime queries;
 *     - environment-dependent branches;
 *     - hardware discovery;
 *     - policy evaluation.
 *
 * Identical source/token streams therefore have identical parse structures.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser preserves:
 *
 *     - source order;
 *     - identifiers;
 *     - qualified names;
 *     - expression structure;
 *     - declaration structure;
 *     - rule structure;
 *     - source spans;
 *     - explicit modifiers;
 *     - explicit grant/deny/revoke decisions.
 *
 * Semantic analysis may subsequently construct:
 *
 *     Permission
 *     PermissionReference
 *     AuthorizationRule
 *     AuthorizationSubject
 *     AuthorizationAction
 *     AuthorizationResource
 *     AuthorizationCondition
 *     Grant
 *     Revocation
 *     Obligation
 *     PermissionRequirement
 *
 * The parser itself MUST NOT construct those semantic objects.
 *
 * ============================================================================
 * SHARED-RULE CONTRACT
 * ============================================================================
 *
 * This grammar expects the language's shared grammar layer to own:
 *
 *     identifier
 *     qualifiedName
 *     attributes
 *     visibility
 *     genericParameters
 *     expression
 *     argumentList
 *     typeExpression
 *
 * They must not be duplicated here.
 *
 * If the repository uses a different canonical rule spelling, the integration
 * adapter must map these names once at the grammar composition boundary rather
 * than creating another security-specific identifier/type system.
 *
 * ============================================================================
 */

parser grammar Permissions;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. PERMISSION DECLARATIONS
 * ============================================================================
 *
 * Examples:
 *
 *     permission read_data;
 *
 *     permission execute_quantum {
 *         requires capability.quantum_execution;
 *     }
 *
 *     permission project::read;
 *
 * Permission names are open-world.
 */

permissionDeclaration
    : attributes?
      visibility?
      PERMISSION
      qualifiedName
      genericParameters?
      permissionInheritanceClause?
      permissionBody?
      SEMI?
    ;


permissionInheritanceClause
    : EXTENDS
      permissionReferenceList
    ;


permissionReferenceList
    : permissionReference
      (COMMA permissionReference)*
    ;


permissionBody
    : LBRACE
      permissionMember*
      RBRACE
    ;


permissionMember
    : permissionRequirement
    | permissionMetadata
    | permissionObligation
    ;


permissionMetadata
    : identifier
      ASSIGN
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 2. PERMISSION REFERENCES
 * ============================================================================
 *
 * Permission references remain symbolic.
 *
 * The grammar does not determine whether the referenced permission exists.
 * Name resolution does that later.
 */

permissionReference
    : qualifiedName
    ;


permissionReferenceExpression
    : permissionReference
    | permissionReferenceCall
    ;


permissionReferenceCall
    : qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 3. PERMISSION REQUIREMENTS
 * ============================================================================
 *
 * A requirement declares that a permission/security property is needed.
 *
 * This does not grant the permission.
 */

permissionRequirement
    : REQUIRES
      permissionRequirementExpression
      SEMI
    ;


permissionRequirementExpression
    : permissionReferenceExpression
    | capabilityReferenceExpression
    | permissionConditionExpression
    | expression
    ;


capabilityReferenceExpression
    : CAPABILITY
      qualifiedName
    ;


permissionConditionExpression
    : LPAREN
      permissionRequirementExpression
      RPAREN
    ;


/*
 * ============================================================================
 * 4. AUTHORIZATION POLICY
 * ============================================================================
 *
 * A policy groups authorization rules.
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
    | grantDeclaration
    | revokeDeclaration
    | permissionRequirement
    | permissionObligation
    | policyMetadata
    ;


policyMetadata
    : identifier
      ASSIGN
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 5. AUTHORIZATION RULES
 * ============================================================================
 *
 * Canonical high-level form:
 *
 *     allow subject to action on resource;
 *
 *     deny subject to action on resource when condition;
 *
 *     grant subject permission;
 *
 *     revoke subject permission;
 *
 * Conditions are source expressions. They are not evaluated here.
 */

authorizationRule
    : authorizationDecision
      authorizationSubjectClause?
      authorizationActionClause?
      authorizationResourceClause?
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


authorizationSubjectClause
    : SUBJECT
      authorizationSubject
    ;


authorizationActionClause
    : ACTION
      authorizationAction
    ;


authorizationResourceClause
    : RESOURCE
      authorizationResource
    ;


authorizationConditionClause
    : WHEN
      expression
    ;


authorizationObligationClause
    : OBLIGE
      authorizationObligation
    ;


/*
 * ============================================================================
 * 6. SUBJECTS
 * ============================================================================
 *
 * Subjects are symbolic references or expressions.
 *
 * The grammar does not define a finite identity model.
 */

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
 * 7. ACTIONS
 * ============================================================================
 *
 * Actions are open-world names.
 *
 * They may represent:
 *
 *     classical computation
 *     quantum execution
 *     measurement
 *     data access
 *     network operations
 *     hardware configuration
 *     compilation
 *     deployment
 *     future operations
 *
 * No finite action vocabulary is imposed.
 */

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
 * 8. RESOURCES
 * ============================================================================
 *
 * Resources are abstract semantic resources.
 *
 * A resource name must not imply a physical device.
 *
 * Examples:
 *
 *     resource data::customer_records
 *     resource quantum::state
 *     resource hardware::accelerator
 *     resource network::endpoint
 *
 * The actual resource is resolved downstream.
 */

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
 * 9. GRANTS
 * ============================================================================
 *
 * A grant expresses an authorization relationship.
 *
 * It is declarative source syntax only.
 */

grantDeclaration
    : GRANT
      grantSubjectClause?
      grantPermissionClause
      grantResourceClause?
      grantConditionClause?
      grantObligationClause*
      SEMI
    ;


grantSubjectClause
    : TO
      authorizationSubject
    ;


grantPermissionClause
    : PERMISSION
      permissionReferenceExpression
    ;


grantResourceClause
    : ON
      authorizationResource
    ;


grantConditionClause
    : WHEN
      expression
    ;


grantObligationClause
    : OBLIGE
      authorizationObligation
    ;


/*
 * ============================================================================
 * 10. REVOCATIONS
 * ============================================================================
 *
 * Revocation syntax describes an intent to revoke an authorization.
 *
 * Runtime enforcement is outside this grammar.
 */

revokeDeclaration
    : REVOKE
      revokeSubjectClause?
      revokePermissionClause
      revokeResourceClause?
      revokeConditionClause?
      SEMI
    ;


revokeSubjectClause
    : FROM
      authorizationSubject
    ;


revokePermissionClause
    : PERMISSION
      permissionReferenceExpression
    ;


revokeResourceClause
    : ON
      authorizationResource
    ;


revokeConditionClause
    : WHEN
      expression
    ;


/*
 * ============================================================================
 * 11. OBLIGATIONS
 * ============================================================================
 *
 * Obligations describe additional security behavior associated with an
 * authorization decision.
 *
 * Examples:
 *
 *     oblige audit;
 *     oblige log(event);
 *     oblige security::verify(...);
 *
 * The grammar does not execute the obligation.
 */

permissionObligation
    : OBLIGE
      authorizationObligation
      SEMI
    ;


authorizationObligation
    : qualifiedName
    | qualifiedName
      LPAREN
      argumentList?
      RPAREN
    | expression
    ;


/*
 * ============================================================================
 * 12. SOURCE-LEVEL PERMISSION BINDINGS
 * ============================================================================
 *
 * A binding associates a permission with a source declaration.
 *
 * This remains metadata/intent rather than enforcement.
 */

permissionBinding
    : REQUIRES
      PERMISSION
      permissionReferenceExpression
      SEMI
    ;


permissionGrantBinding
    : GRANT
      PERMISSION
      permissionReferenceExpression
      SEMI
    ;


permissionDenyBinding
    : DENY
      PERMISSION
      permissionReferenceExpression
      SEMI
    ;


/*
 * ============================================================================
 * 13. PERMISSION COMPOSITION
 * ============================================================================
 *
 * Permissions may be composed without imposing a fixed hierarchy.
 */

permissionComposition
    : permissionCompositionOperator
      LBRACE
      permissionReferenceList?
      RBRACE
    ;


permissionCompositionOperator
    : ALL
    | ANY
    | UNION
    | INTERSECT
    | EXCEPT
    ;


/*
 * ============================================================================
 * 14. CONDITIONAL PERMISSIONS
 * ============================================================================
 *
 * Conditions remain ordinary Zamani expressions.
 *
 * The security grammar must never create a second expression language.
 */

conditionalPermission
    : permissionReferenceExpression
      WHEN
      expression
    ;


/*
 * ============================================================================
 * 15. DELEGATION
 * ============================================================================
 *
 * Delegation describes source-level authorization intent.
 *
 * It does not establish trust or identity.
 */

permissionDelegation
    : DELEGATE
      authorizationSubject
      TO
      authorizationSubject
      FOR
      permissionReferenceExpression
      delegationCondition?
      SEMI
    ;


delegationCondition
    : WHEN
      expression
    ;


/*
 * ============================================================================
 * 16. TEMPORAL / CONTEXTUAL AUTHORIZATION
 * ============================================================================
 *
 * The grammar deliberately accepts expressions instead of defining fixed
 * notions such as "business hours", "location", "device", or "network".
 *
 * This keeps the language open-ended.
 */

contextualAuthorization
    : authorizationRule
    ;


authorizationContext
    : CONTEXT
      expression
    ;


/*
 * ============================================================================
 * 17. RESOURCE-SCOPED PERMISSIONS
 * ============================================================================
 *
 * Scope is semantic, not physical.
 */

permissionScope
    : SCOPE
      authorizationResource
    ;


scopedPermissionDeclaration
    : PERMISSION
      qualifiedName
      permissionScope
      permissionBody?
      SEMI?
    ;


/*
 * ============================================================================
 * 18. PRINCIPAL-SCOPED PERMISSIONS
 * ============================================================================
 */

principalPermissionBinding
    : authorizationSubject
      REQUIRES
      PERMISSION
      permissionReferenceExpression
      SEMI
    ;


/*
 * ============================================================================
 * 19. DEFAULT DECISIONS
 * ============================================================================
 *
 * A default decision is policy metadata.
 *
 * The actual precedence/combination semantics are defined by the semantic
 * policy engine, not the parser.
 */

defaultAuthorizationDecision
    : DEFAULT
      authorizationDecision
      SEMI
    ;


/*
 * ============================================================================
 * 20. POLICY PRIORITY / ORDER
 * ============================================================================
 *
 * Priority is represented as an expression rather than a bounded integer
 * domain. This avoids imposing arbitrary implementation limits.
 */

policyPriority
    : PRIORITY
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 21. PERMISSION METADATA
 * ============================================================================
 *
 * Metadata remains open-world.
 */

permissionAnnotation
    : AT
      qualifiedName
      (LPAREN argumentList? RPAREN)?
    ;


/*
 * ============================================================================
 * 22. DECLARATION GROUP
 * ============================================================================
 *
 * This rule provides one integration point for the parent security grammar.
 */

permissionsDeclaration
    : permissionDeclaration
    | authorizationPolicyDeclaration
    | permissionBinding
    | permissionGrantBinding
    | permissionDenyBinding
    | permissionDelegation
    | scopedPermissionDeclaration
    | principalPermissionBinding
    | defaultAuthorizationDecision
    | policyPriority
    | contextualAuthorization
    ;


/*
 * ============================================================================
 * 23. SECURITY-SENSITIVE ANNOTATION TARGET
 * ============================================================================
 *
 * This rule intentionally accepts a generic declaration reference rather
 * than reproducing declaration syntax owned elsewhere.
 */

securedReference
    : AT
      qualifiedName
      (LPAREN argumentList? RPAREN)?
    ;


/*
 * ============================================================================
 * 24. SECURITY REQUIREMENT EXPRESSION
 * ============================================================================
 *
 * Generic composition allows future security models without changing this
 * grammar merely because a new security concept is introduced.
 */

securityPermissionExpression
    : permissionReferenceExpression
    | permissionComposition
    | conditionalPermission
    | authorizationContext
    | expression
    ;


/*
 * ============================================================================
 * 25. AUTHORIZATION EXPRESSION
 * ============================================================================
 *
 * This is deliberately symbolic.
 */

authorizationExpression
    : authorizationSubject
      authorizationActionClause?
      authorizationResourceClause?
      authorizationConditionClause?
    ;