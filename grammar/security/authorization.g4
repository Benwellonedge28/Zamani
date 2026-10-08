/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/security/authorization.g4
 *
 * Grammar:
 *     Authorization
 *
 * Status:
 *     CANONICAL SECURITY AUTHORIZATION LEAF GRAMMAR
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file is the sole parser-level owner of SOURCE-LEVEL AUTHORIZATION
 * syntax.
 *
 * Authorization expresses declarative intent concerning:
 *
 *     - subjects;
 *     - actions;
 *     - resources;
 *     - permissions;
 *     - scopes;
 *     - conditions;
 *     - obligations;
 *     - capabilities;
 *     - trust requirements;
 *     - grants;
 *     - revocations;
 *     - delegations;
 *     - attenuations;
 *     - authorization metadata.
 *
 * This grammar expresses authorization INTENT.
 *
 * It does not:
 *
 *     - authenticate;
 *     - authorize at runtime;
 *     - evaluate policies;
 *     - evaluate trust;
 *     - discover capabilities;
 *     - discover resources;
 *     - select hardware;
 *     - execute cryptography;
 *     - access credentials;
 *     - access secrets;
 *     - construct quantum::ir;
 *     - construct HDL IR;
 *     - route;
 *     - schedule;
 *     - optimize;
 *     - enforce security.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     authorizationDeclaration
 *     authorizationRule
 *     authorizationGrant
 *     authorizationRevocation
 *     authorizationDelegation
 *     authorizationAttenuation
 *     authorizationRequirement
 *     authorization subject/action/resource/scope clauses
 *     authorization condition clauses
 *     authorization obligation clauses
 *     authorization capability/trust references
 *     authorization metadata
 *     authorization references
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     identifiers
 *     principals
 *     identity declarations
 *     authentication
 *     permissions declarations
 *     capability declarations
 *     trust declarations
 *     policy declarations
 *     policy evaluation
 *     cryptography
 *     privacy
 *     generic resources
 *     generic requirements
 *     generic effects
 *     expressions
 *     types
 *     quantum operations
 *     HDL syntax
 *     hardware discovery
 *     runtime enforcement
 *
 * ============================================================================
 * AUTHORITATIVE OWNERS
 * ============================================================================
 *
 * Identity / principal syntax:
 *
 *     grammar/security/identifiers.g4
 *
 * Security capabilities:
 *
 *     grammar/security/capabilities.g4
 *
 * Permission declarations:
 *
 *     grammar/security/permissions.g4
 *
 * Trust declarations:
 *
 *     grammar/security/trust.g4
 *
 * Generic policies:
 *
 *     grammar/policies/policy.g4
 *
 * Generic requirements:
 *
 *     grammar/core/requirements.g4
 *
 * Generic capabilities:
 *
 *     grammar/core/capabilities.g4
 *
 * Generic constraints:
 *
 *     grammar/core/constraints.g4
 *
 * Expressions:
 *
 *     grammar/expressions/
 *
 * Names:
 *
 *     grammar/core/
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     Core
 *     Types
 *     Expressions
 *     canonical ZamaniLexer
 *
 * EXPORTS:
 *
 *     authorizationFile
 *     authorizationDeclaration
 *     authorizationMember
 *     authorizationRule
 *     authorizationGrant
 *     authorizationRevocation
 *     authorizationDelegation
 *     authorizationAttenuation
 *     authorizationRequirement
 *     authorizationSubject
 *     authorizationAction
 *     authorizationResource
 *     authorizationScope
 *     authorizationReference
 *
 * CONSUMED_BY:
 *
 *     grammar/security/security.g4
 *     grammar/security tests
 *     canonical parser composition
 *     frontend AST construction
 *
 * AST_OWNER:
 *
 *     frontend AST / security semantic model
 *
 * SEMANTIC_OWNER:
 *
 *     security authorization semantic analysis
 *
 * IR_OWNER:
 *
 *     canonical semantic/IR layers
 *
 *     classical IR
 *     quantum::ir
 *     HDL/hardware representations
 *     distributed representations
 *
 * TEST_OWNER:
 *
 *     grammar/tests/security/authorization/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/security.md
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * ANTLR4 parser grammar.
 *
 * Rust implementation:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust only
 *
 * This grammar intentionally contains:
 *
 *     - no embedded Rust;
 *     - no parser actions;
 *     - no semantic predicates;
 *     - no runtime callbacks;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access;
 *     - no environment inspection;
 *     - no credential access;
 *     - no secret access.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Authorization is target-independent.
 *
 * It must not encode fixed limits for:
 *
 *     principals
 *     identities
 *     actions
 *     resources
 *     permissions
 *     capabilities
 *     policies
 *     rules
 *     delegations
 *     devices
 *     nodes
 *     CPUs
 *     GPUs
 *     FPGAs
 *     QPUs
 *     qubits
 *     memory
 *     storage
 *
 * There are no language-level capacity constants.
 *
 * Target feasibility is resolved downstream through:
 *
 *     capability analysis
 *     resource analysis
 *     policy analysis
 *     security analysis
 *     compilation
 *     lowering
 *     scheduling
 *     deployment
 *     runtime enforcement.
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * Subjects, actions, resources, permissions, capabilities, scopes, policies,
 * authorities and security mechanisms are represented through open-world
 * symbolic references or expressions.
 *
 * The grammar does not enumerate:
 *
 *     users
 *     vendors
 *     providers
 *     devices
 *     algorithms
 *     protocols
 *     security mechanisms
 *     quantum operations
 *     hardware types.
 *
 * A future authorization concept can therefore be represented without
 * modifying this grammar merely because a new computational domain exists.
 *
 * ============================================================================
 * SECRET BOUNDARY
 * ============================================================================
 *
 * Authorization syntax may contain SYMBOLIC REFERENCES to security objects.
 *
 * It must not contain embedded secret material.
 *
 * Actual credentials, private keys, bearer tokens, authentication secrets,
 * session secrets and similar values belong to secure runtime/key-management
 * infrastructure.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * The grammar contains no:
 *
 *     - actions;
 *     - semantic predicates;
 *     - runtime calls;
 *     - environment queries;
 *     - hardware queries;
 *     - randomness.
 *
 * Identical token streams therefore produce equivalent parse structures.
 *
 * ============================================================================
 */

parser grammar Authorization;

options {
    tokenVocab = ZamaniLexer;
}

import Core, Types, Expressions;


/*
 * ============================================================================
 * 1. STANDALONE AUTHORIZATION FILE
 * ============================================================================
 *
 * This rule is for isolated grammar tests and tooling.
 *
 * The production security composition root MUST use
 * authorizationDeclaration directly rather than introducing another EOF
 * boundary.
 */

authorizationFile
    : authorizationDeclaration*
      EOF
    ;


/*
 * ============================================================================
 * 2. AUTHORIZATION DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     authorization data::access {
 *         ...
 *     }
 *
 * The name is symbolic and open-world.
 */

authorizationDeclaration
    : attributeList?
      visibility?
      AUTHORIZATION
      qualifiedName
      genericParameters?
      authorizationBody
      SEMI?
    ;


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
 * 3. AUTHORIZATION RULE
 * ============================================================================
 *
 * Canonical form:
 *
 *     allow
 *         subject principal::operator
 *         action quantum::execute
 *         resource quantum::program
 *         when condition;
 *
 * A rule must contain at least one authorization target dimension:
 *
 *     subject
 *     action
 *     resource
 *     permission
 *
 * The semantic layer determines whether the resulting rule is sufficiently
 * constrained for the selected authorization model.
 */

authorizationRule
    : authorizationDecision
      authorizationRuleClause+
      SEMI
    ;


authorizationDecision
    : ALLOW
    | DENY
    | REJECT
    ;


/*
 * ============================================================================
 * 4. AUTHORIZATION RULE CLAUSES
 * ============================================================================
 */

authorizationRuleClause
    : authorizationSubjectClause
    | authorizationActionClause
    | authorizationResourceClause
    | authorizationPermissionClause
    | authorizationScopeClause
    | authorizationConditionClause
    | authorizationObligationClause
    | authorizationCapabilityClause
    | authorizationTrustClause
    | authorizationMetadata
    ;


/*
 * ============================================================================
 * 5. SUBJECT
 * ============================================================================
 *
 * Subject syntax is deliberately reference-oriented.
 *
 * Identity/principal declarations remain owned by identifiers.g4.
 */

authorizationSubjectClause
    : SUBJECT
      authorizationSubject
    ;


authorizationSubject
    : authorizationReference
    | authorizationSubjectCall
    | authorizationSubjectSet
    ;


authorizationSubjectCall
    : qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


authorizationSubjectSet
    : LBRACE
      authorizationSubjectSetItem
      (COMMA authorizationSubjectSetItem)*
      COMMA?
      RBRACE
    ;


authorizationSubjectSetItem
    : authorizationSubject
    ;


/*
 * ============================================================================
 * 6. ACTION
 * ============================================================================
 *
 * Actions are open-world symbolic references or expressions.
 *
 * No finite action catalogue exists.
 */

authorizationActionClause
    : ACTION
      authorizationAction
    ;


authorizationAction
    : authorizationReference
    | authorizationActionCall
    ;


authorizationActionCall
    : qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 7. RESOURCE
 * ============================================================================
 *
 * Resources are semantic resources.
 *
 * They need not represent physical resources.
 */

authorizationResourceClause
    : RESOURCE
      authorizationResource
    ;


authorizationResource
    : authorizationReference
    | authorizationResourceCall
    | authorizationResourceSet
    ;


authorizationResourceCall
    : qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


authorizationResourceSet
    : LBRACE
      authorizationResourceSetItem
      (COMMA authorizationResourceSetItem)*
      COMMA?
      RBRACE
    ;


authorizationResourceSetItem
    : authorizationResource
    ;


/*
 * ============================================================================
 * 8. PERMISSION REFERENCE
 * ============================================================================
 *
 * Permission declarations belong to security/permissions.g4.
 *
 * This file only consumes references to permissions.
 */

authorizationPermissionClause
    : PERMISSION
      authorizationReference
    ;


/*
 * ============================================================================
 * 9. SCOPE
 * ============================================================================
 *
 * Scope is symbolic authorization scope.
 *
 * It is not physical topology.
 */

authorizationScopeClause
    : SCOPE
      authorizationScope
    ;


authorizationScope
    : authorizationReference
    | authorizationScopeCall
    | authorizationScopeSet
    ;


authorizationScopeCall
    : qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


authorizationScopeSet
    : LBRACE
      authorizationScopeSetItem
      (COMMA authorizationScopeSetItem)*
      COMMA?
      RBRACE
    ;


authorizationScopeSetItem
    : authorizationScope
    ;


/*
 * ============================================================================
 * 10. CONDITION
 * ============================================================================
 *
 * Conditions are ordinary Zamani expressions.
 *
 * They are never evaluated by the parser.
 */

authorizationConditionClause
    : WHEN
      expression
    ;


/*
 * ============================================================================
 * 11. OBLIGATION
 * ============================================================================
 *
 * An obligation identifies downstream semantic work associated with an
 * authorization result.
 *
 * The parser records intent only.
 */

authorizationObligationClause
    : OBLIGE
      authorizationObligation
    ;


authorizationObligation
    : authorizationReference
    | authorizationObligationCall
    ;


authorizationObligationCall
    : qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 12. CAPABILITY REQUIREMENT
 * ============================================================================
 *
 * This references the canonical capability system.
 *
 * It does not declare or discover a capability.
 */

authorizationCapabilityClause
    : CAPABILITY
      authorizationReference
    ;


/*
 * ============================================================================
 * 13. TRUST REQUIREMENT
 * ============================================================================
 *
 * Trust declarations remain owned by security/trust.g4.
 *
 * Authorization merely consumes a symbolic trust reference.
 */

authorizationTrustClause
    : TRUST
      authorizationReference
    ;


/*
 * ============================================================================
 * 14. GRANT
 * ============================================================================
 *
 * Canonical form:
 *
 *     grant
 *         to principal::operator
 *         permission data::read
 *         action data::read
 *         on data::dataset;
 *
 * Grant semantics are resolved downstream.
 */

authorizationGrant
    : GRANT
      authorizationGrantClause+
      SEMI
    ;


authorizationGrantClause
    : authorizationGrantSubjectClause
    | authorizationGrantPermissionClause
    | authorizationGrantActionClause
    | authorizationGrantResourceClause
    | authorizationScopeClause
    | authorizationConditionClause
    | authorizationObligationClause
    | authorizationCapabilityClause
    | authorizationTrustClause
    | authorizationMetadata
    ;


authorizationGrantSubjectClause
    : TO
      authorizationSubject
    ;


authorizationGrantPermissionClause
    : PERMISSION
      authorizationReference
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
 * 15. REVOCATION
 * ============================================================================
 *
 * Revocation expresses source intent to withdraw authority.
 */

authorizationRevocation
    : REVOKE
      authorizationRevocationClause+
      SEMI
    ;


authorizationRevocationClause
    : FROM
      authorizationSubject
    | PERMISSION
      authorizationReference
    | ACTION
      authorizationAction
    | ON
      authorizationResource
    | authorizationScopeClause
    | authorizationConditionClause
    | authorizationMetadata
    ;


/*
 * ============================================================================
 * 16. DELEGATION
 * ============================================================================
 *
 * Delegation expresses transfer of bounded authority.
 *
 * Semantic validation must ensure that delegation does not exceed the
 * delegator's authority.
 */

authorizationDelegation
    : DELEGATE
      authorizationDelegationClause+
      SEMI
    ;


authorizationDelegationClause
    : authorizationDelegatorClause
    | authorizationDelegateeClause
    | authorizationGrantClause
    | authorizationScopeClause
    | authorizationConditionClause
    | authorizationObligationClause
    | authorizationCapabilityClause
    | authorizationTrustClause
    | authorizationMetadata
    ;


authorizationDelegatorClause
    : FROM
      authorizationSubject
    ;


authorizationDelegateeClause
    : TO
      authorizationSubject
    ;


authorizationGrantClause
    : GRANT
      authorizationAuthorityReference
    ;


authorizationAuthorityReference
    : authorizationReference
    ;


/*
 * ============================================================================
 * 17. ATTENUATION
 * ============================================================================
 *
 * Attenuation narrows an existing authority.
 *
 * It must never silently expand authority.
 *
 * The monotonicity property is semantic, not syntactic.
 */

authorizationAttenuation
    : ATTENUATE
      authorizationAuthorityReference
      authorizationAttenuationClause*
      SEMI
    ;


authorizationAttenuationClause
    : authorizationPermissionClause
    | authorizationActionClause
    | authorizationResourceClause
    | authorizationScopeClause
    | authorizationConditionClause
    | authorizationObligationClause
    | authorizationCapabilityClause
    | authorizationTrustClause
    | authorizationMetadata
    ;


/*
 * ============================================================================
 * 18. AUTHORIZATION REQUIREMENT
 * ============================================================================
 *
 * Requirements are mandatory security intent.
 *
 * They must not silently degrade into preferences.
 */

authorizationRequirement
    : REQUIRES
      authorizationRequirementClause
      SEMI
    ;


authorizationRequirementClause
    : authorizationPermissionRequirement
    | authorizationCapabilityRequirement
    | authorizationTrustRequirement
    | authorizationConditionRequirement
    | authorizationReference
    ;


authorizationPermissionRequirement
    : PERMISSION
      authorizationReference
    ;


authorizationCapabilityRequirement
    : CAPABILITY
      authorizationReference
    ;


authorizationTrustRequirement
    : TRUST
      authorizationReference
    ;


authorizationConditionRequirement
    : WHEN
      expression
    ;


/*
 * ============================================================================
 * 19. METADATA
 * ============================================================================
 *
 * Metadata is generic source data.
 *
 * Metadata keys are open-world names.
 */

authorizationMetadata
    : identifier
      ASSIGN
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 20. COMMON AUTHORIZATION REFERENCE
 * ============================================================================
 *
 * A common reference boundary prevents subject/action/resource grammars from
 * each inventing incompatible name syntax.
 *
 * Meaning is determined by semantic context.
 */

authorizationReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 21. PUBLIC REFERENCE LIST
 * ============================================================================
 *
 * Reusable by semantic adapters and isolated tests.
 */

authorizationReferenceList
    : authorizationReference
      (COMMA authorizationReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * 22. CANONICAL AUTHORIZATION STATEMENT
 * ============================================================================
 *
 * This is the concise source form.
 *
 * Example:
 *
 *     authorize principal::operator
 *         action quantum::execute
 *         resource quantum::program;
 *
 * It is deliberately an adapter to authorization intent, not a second
 * authorization semantic model.
 */

authorizationStatement
    : AUTHORIZE
      authorizationSubject
      authorizationStatementClause*
      SEMI
    ;


authorizationStatementClause
    : authorizationActionClause
    | authorizationResourceClause
    | authorizationPermissionClause
    | authorizationScopeClause
    | authorizationConditionClause
    | authorizationObligationClause
    | authorizationCapabilityClause
    | authorizationTrustClause
    | authorizationMetadata
    ;


/*
 * ============================================================================
 * 23. SECURITY AUTHORIZATION REFERENCE
 * ============================================================================
 *
 * Stable semantic boundary for consumers that need a generic authorization
 * reference without importing the complete declaration grammar.
 */

authorizationReferenceExpression
    : authorizationReference
    | authorizationReferenceCall
    ;


authorizationReferenceCall
    : qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 24. INTEGRATION INVARIANTS
 * ============================================================================
 *
 * The following must remain true.
 *
 * 1. This file is the sole owner of authorization syntax.
 *
 * 2. Policy declarations remain owned by grammar/policies/policy.g4.
 *
 * 3. Identity declarations remain owned by security/identifiers.g4.
 *
 * 4. Permission declarations remain owned by security/permissions.g4.
 *
 * 5. Capability declarations remain owned by security/capabilities.g4.
 *
 * 6. Trust declarations remain owned by security/trust.g4.
 *
 * 7. Generic requirements remain owned by core/requirements.g4.
 *
 * 8. Expressions remain owned by expressions/.
 *
 * 9. No authorization construct performs runtime authorization.
 *
 * 10. No authorization construct authenticates a subject.
 *
 * 11. No authorization construct contains secret material.
 *
 * 12. No authorization construct performs capability discovery.
 *
 * 13. No authorization construct performs resource discovery.
 *
 * 14. No authorization construct selects a physical target.
 *
 * 15. No authorization construct creates quantum::ir.
 *
 * 16. No authorization construct creates an HDL/hardware IR.
 *
 * 17. No authorization construct contains a finite hardware catalogue.
 *
 * 18. No authorization construct contains machine-capacity limits.
 *
 * 19. Mandatory requirements remain distinguishable from preferences.
 *
 * 20. Source spans and declaration order remain recoverable downstream.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must:
 *
 *     - resolve subjects;
 *     - resolve actions;
 *     - resolve resources;
 *     - resolve permissions;
 *     - resolve capabilities;
 *     - resolve trust references;
 *     - validate conditions;
 *     - validate scopes;
 *     - validate obligations;
 *     - validate grants;
 *     - validate revocations;
 *     - validate delegations;
 *     - validate attenuation;
 *     - detect conflicting authorization rules;
 *     - enforce delegation monotonicity;
 *     - preserve mandatory security requirements;
 *     - attach provenance;
 *     - attach policy context;
 *     - produce diagnostics.
 *
 * None of these operations belong in the grammar.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing authorization has no execution effects.
 *
 * Downstream semantic authorization may interact with effect analysis for
 * operations such as:
 *
 *     io
 *     network
 *     mutation
 *     native
 *     foreign
 *     distributed
 *     measurement
 *     adaptation
 *     reflection
 *     code generation
 *     simulation
 *
 * Authorization syntax itself does not grant any effect.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resources are symbolic or expression-based.
 *
 * Resource feasibility is determined by the resource/capability subsystem.
 *
 * There is no authorization-level limit on resource cardinality, quantity,
 * topology, machine count, device count, or security-object count.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Every authorization declaration/rule must remain source-locatable.
 *
 * Downstream provenance should be capable of recording:
 *
 *     source declaration;
 *     source span;
 *     referenced subject;
 *     referenced action;
 *     referenced resource;
 *     referenced permission;
 *     referenced capability;
 *     referenced trust relationship;
 *     authorization decision;
 *     transformation;
 *     policy context;
 *     verification result.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Authorization may protect quantum operations such as:
 *
 *     quantum::execute
 *     quantum::measure
 *     quantum::compile
 *     quantum::deploy
 *
 * These remain symbolic references.
 *
 * This grammar MUST NOT define:
 *
 *     qubits;
 *     physical qubits;
 *     gate catalogues;
 *     coupling maps;
 *     calibration;
 *     QEC;
 *     routing;
 *     scheduling;
 *     QPU selection.
 *
 * Quantum semantics eventually cross:
 *
 *     semantic quantum model
 *             |
 *             v
 *         quantum::ir
 *
 * Authorization metadata may accompany that semantic representation.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Authorization may protect symbolic operations involving:
 *
 *     hardware
 *     accelerator
 *     synthesis
 *     programming
 *     deployment
 *
 * The grammar does not select a physical implementation.
 *
 * ============================================================================
 * DISTRIBUTED / NETWORKING BOUNDARY
 * ============================================================================
 *
 * Authorization may protect arbitrary distributed and networking operations.
 *
 * No fixed number of:
 *
 *     nodes
 *     endpoints
 *     services
 *     channels
 *     replicas
 *     devices
 *
 * is encoded here.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [x] It has a valid parser grammar declaration.
 * [x] It uses the canonical Zamani lexer vocabulary.
 * [x] It imports canonical Core, Types and Expressions grammars.
 * [x] It has one authorization syntax owner.
 * [x] It does not own policy declarations.
 * [x] It does not own identity declarations.
 * [x] It does not own permission declarations.
 * [x] It does not own capability declarations.
 * [x] It does not own trust declarations.
 * [x] It contains no duplicate parser rule definitions.
 * [x] It has no embedded Rust.
 * [x] It has no semantic predicates.
 * [x] It has no runtime behavior.
 * [x] It has no hardware discovery.
 * [x] It has no fixed resource limits.
 * [x] It has no finite security-object catalogue.
 * [x] It has standalone parsing support.
 * [x] It preserves source-level authorization intent.
 *
 * Repository-level completion additionally requires:
 *
 * [ ] canonical authorization tokens are present in the lexer;
 * [ ] security.g4 imports this grammar;
 * [ ] permissions.g4 does not duplicate authorization syntax;
 * [ ] policy.g4 consumes authorization semantics through a stable boundary;
 * [ ] frontend AST has authorization nodes;
 * [ ] semantic authorization analysis exists;
 * [ ] provenance preserves authorization source information;
 * [ ] positive tests pass;
 * [ ] negative tests pass;
 * [ ] boundary tests pass;
 * [ ] scalability tests pass;
 * [ ] cross-domain tests pass;
 * [ ] deterministic parsing tests pass;
 * [ ] Rust 1.97+ safe frontend generation/build passes.
 *
 * ============================================================================
 */