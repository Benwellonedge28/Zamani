/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/security/capabilities.g4
 *
 * Grammar:
 *     SecurityCapabilities
 *
 * STATUS
 * ------
 * CANONICAL SECURITY-AUTHORITY CAPABILITY GRAMMAR
 *
 * PURPOSE
 * -------
 * This grammar defines source-level SECURITY AUTHORITY CAPABILITIES.
 *
 * It is deliberately distinct from:
 *
 *     grammar/core/capabilities.g4
 *
 * which owns the language-wide computational capability model.
 *
 * A computational capability answers:
 *
 *     "What can an execution environment provide?"
 *
 * A security capability answers:
 *
 *     "What authority may be represented, delegated, attenuated,
 *      or associated with a security principal?"
 *
 * This grammar describes security authority as SOURCE-LEVEL INTENT only.
 *
 * It does not authenticate, authorize, issue credentials, discover hardware,
 * evaluate policy, execute cryptography, or perform runtime enforcement.
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * Rust:
 *     1.97 / 1.97.1
 *
 * Edition:
 *     Rust 2021
 *
 * Safety:
 *     The grammar contains no embedded Rust actions.
 *
 * The Zamani implementation MUST remain safe Rust.
 *
 * No unsafe code is required by this grammar.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     grammar/antlr/ZamaniLexer.g4
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     SecurityCapabilities
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     security semantic analysis
 *          |
 *          +--> identity resolution
 *          +--> permission resolution
 *          +--> capability resolution
 *          +--> trust analysis
 *          +--> policy analysis
 *          +--> resource analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +--> classical representation
 *          +--> quantum::ir
 *          +--> HDL / hardware representation
 *          +--> distributed representation
 *          +--> other domain representations
 *          |
 *          v
 *     optimization / lowering
 *          |
 *          v
 *     runtime / deployment / enforcement
 *
 * This grammar MUST NOT reverse this dependency direction.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - security capability declarations;
 *     - security capability references;
 *     - security capability inheritance;
 *     - permission associations;
 *     - principal associations;
 *     - action associations;
 *     - resource associations;
 *     - scope declarations;
 *     - issuer declarations;
 *     - security requirements;
 *     - conditions;
 *     - delegation intent;
 *     - attenuation intent;
 *     - open-ended security capability metadata;
 *     - source-level security capability expressions.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - generic computational capabilities;
 *     - generic resource declarations;
 *     - identity declarations;
 *     - authentication;
 *     - credential storage;
 *     - credential verification;
 *     - permission declarations;
 *     - authorization policy declarations;
 *     - policy evaluation;
 *     - trust evaluation;
 *     - cryptographic implementation;
 *     - key management;
 *     - secret storage;
 *     - hardware discovery;
 *     - target selection;
 *     - resource allocation;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - quantum::ir;
 *     - runtime enforcement.
 *
 * ============================================================================
 * CRITICAL CAPABILITY SEPARATION
 * ============================================================================
 *
 * There are three distinct concepts:
 *
 * 1. COMPUTATIONAL CAPABILITY
 *
 *      grammar/core/capabilities.g4
 *
 *      Example:
 *
 *          quantum::measurement
 *          accelerator::tensor
 *
 * 2. EFFECT CAPABILITY REQUIREMENT
 *
 *      grammar/effects/capabilities.g4
 *
 *      Example:
 *
 *          requires {
 *              quantum::measurement
 *          }
 *
 * 3. SECURITY AUTHORITY CAPABILITY
 *
 *      THIS FILE
 *
 *      Example:
 *
 *          capability security::quantum_execution {
 *              permission quantum::execute;
 *              principal service::quantum_executor;
 *              action quantum::execute;
 *              resource quantum::program;
 *          }
 *
 * These concepts MUST NOT be represented by one overloaded AST/semantic
 * concept merely because they all use the word "capability".
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Security capabilities are symbolic and target-independent.
 *
 * This grammar MUST NOT encode:
 *
 *     MAX_CAPABILITIES
 *     MAX_SECURITY_CAPABILITIES
 *     MAX_PRINCIPALS
 *     MAX_PERMISSIONS
 *     MAX_RESOURCES
 *     MAX_ACTIONS
 *     MAX_DELEGATIONS
 *     MAX_ATOMIZATIONS
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * It MUST NOT encode:
 *
 *     physical device IDs
 *     physical qubit IDs
 *     CPU IDs
 *     GPU IDs
 *     FPGA IDs
 *     QPU IDs
 *     fixed providers
 *     fixed backends
 *     fixed topology
 *     fixed node counts
 *     fixed resource capacities
 *
 * Repetition uses `*` and `+`.
 *
 * Therefore the language has no artificial security-capability ceiling.
 *
 * Practical limits imposed by:
 *
 *     memory
 *     compiler resources
 *     parser configuration
 *     runtime resources
 *     target resources
 *
 * remain implementation/resource concerns and MUST NOT become language
 * semantics.
 *
 * ============================================================================
 * OPEN-WORLD SECURITY
 * ============================================================================
 *
 * Security capability identities are open-ended.
 *
 * Valid examples include:
 *
 *     security::data::read
 *     security::quantum::execute
 *     security::hardware::configure
 *     security::network::admin
 *     organization::research::execution
 *     future::security::new_authority
 *
 * This grammar MUST NOT enumerate:
 *
 *     read
 *     write
 *     admin
 *     execute
 *     quantum
 *     GPU
 *     CPU
 *     FPGA
 *     QPU
 *
 * as a closed universe of security capabilities.
 *
 * Those are names, not grammar-level semantic enums.
 *
 * ============================================================================
 * SECRET-MATERIAL BOUNDARY
 * ============================================================================
 *
 * This grammar MUST NOT provide syntax for embedding:
 *
 *     passwords
 *     private keys
 *     secret keys
 *     bearer tokens
 *     API keys
 *     session tokens
 *     recovery secrets
 *     raw credentials
 *
 * A security capability may reference a symbolic security object, but the
 * referenced secret material belongs to the credential/key-management
 * subsystem.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Security authority may protect quantum computation.
 *
 * Example:
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
 *     QEC code
 *     topology
 *     calibration
 *     noise model
 *     routing
 *     scheduling
 *
 * Quantum semantics remain downstream.
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Security capabilities may protect abstract hardware intent:
 *
 *     hardware::configure
 *     hardware::reconfigure
 *     accelerator::execute
 *
 * They MUST NOT select physical hardware.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no actions;
 *     no semantic predicates;
 *     no I/O;
 *     no filesystem access;
 *     no network access;
 *     no randomness;
 *     no hardware discovery;
 *     no runtime calls;
 *     no policy evaluation.
 *
 * Parsing therefore depends only on:
 *
 *     source token stream
 *     grammar version
 *     parser configuration
 *
 * ============================================================================
 */

parser grammar SecurityCapabilities;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * Core supplies:
 *
 *     identifier
 *     qualifiedName
 *     attributes
 *     visibility
 *     genericParameters
 *     expression
 *     typeExpression
 *     capabilityReference
 *
 * Types supplies canonical type syntax.
 *
 * Expressions supplies canonical expression syntax.
 *
 * IMPORTANT:
 *
 * This grammar deliberately does NOT import Permissions or Security directly.
 *
 * That avoids a circular/duplicate ownership relationship:
 *
 *     Security
 *        -> SecurityCapabilities
 *        -> Permissions
 *
 * SecurityCapabilities remains an independent security-authority grammar.
 */
import Core, Types, Expressions;


/* ============================================================================
 * 1. STANDALONE TEST ENTRY
 * ============================================================================
 *
 * This rule is for grammar tests/tooling.
 *
 * The production security composition root MUST consume:
 *
 *     securityCapabilityDeclaration
 *
 * rather than this EOF-bearing rule.
 */
securityCapabilitiesFile
    : securityCapabilityDeclaration+
      EOF
    ;


/* ============================================================================
 * 2. CANONICAL SECURITY CAPABILITY DECLARATION
 * ============================================================================
 *
 * Examples:
 *
 *     capability security::quantum_execution;
 *
 *     capability security::quantum_execution {
 *         permission quantum::execute;
 *     }
 *
 *     capability security::quantum_execution {
 *         permission quantum::execute;
 *         principal service::quantum_executor;
 *         action quantum::execute;
 *         resource quantum::program;
 *         scope security::research;
 *         issuer organization::security_authority;
 *     }
 *
 * No finite number of members is imposed.
 */
securityCapabilityDeclaration
    : attributes*
      visibility?
      CAPABILITY
      securityCapabilityName
      genericParameters?
      securityCapabilityInheritanceClause?
      securityCapabilityBody?
      SEMI?
    ;


/* ============================================================================
 * 3. SECURITY CAPABILITY NAME
 * ============================================================================
 *
 * Security capability identity is deliberately separate from the generic
 * computational capability reference.
 *
 * A security capability is still represented by a normal Zamani qualified
 * name.
 */
securityCapabilityName
    : qualifiedName
    ;


/* ============================================================================
 * 4. SECURITY CAPABILITY REFERENCE
 * ============================================================================
 *
 * A reference does not declare or instantiate a capability.
 */
securityCapabilityReference
    : securityCapabilityName
    ;


/* ============================================================================
 * 5. INHERITANCE
 * ============================================================================
 *
 * Source-level composition only.
 *
 * Semantic analysis MUST determine whether inheritance is legal.
 */
securityCapabilityInheritanceClause
    : EXTENDS
      securityCapabilityReferenceList
    ;


securityCapabilityReferenceList
    : securityCapabilityReference
      (
          COMMA
          securityCapabilityReference
      )*
    ;


/* ============================================================================
 * 6. CAPABILITY BODY
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
    | securityCapabilityRequirement
    | securityCapabilityCondition
    | securityCapabilityDelegation
    | securityCapabilityAttenuation
    | securityCapabilityProperty
    ;


/* ============================================================================
 * 7. PERMISSIONS
 * ============================================================================
 *
 * Permission declarations themselves remain owned by:
 *
 *     grammar/security/permissions.g4
 *
 * This grammar only associates an already named permission with a security
 * capability.
 *
 * We intentionally use a local rule name instead of importing Permissions,
 * preventing grammar coupling and duplicate ownership.
 */
securityCapabilityPermission
    : PERMISSION
      securityPermissionReferenceList
      SEMI
    ;


securityPermissionReferenceList
    : securityPermissionReference
      (
          COMMA
          securityPermissionReference
      )*
    ;


securityPermissionReference
    : qualifiedName
    ;


/* ============================================================================
 * 8. PRINCIPALS
 * ============================================================================
 *
 * A principal is a symbolic security subject.
 *
 * Authentication and identity verification remain downstream.
 */
securityCapabilityPrincipal
    : PRINCIPAL
      securityPrincipalReferenceList
      SEMI
    ;


securityPrincipalReferenceList
    : securityPrincipalReference
      (
          COMMA
          securityPrincipalReference
      )*
    ;


securityPrincipalReference
    : qualifiedName
    | expression
    ;


/* ============================================================================
 * 9. ACTIONS
 * ============================================================================
 *
 * Action names remain open-world.
 */
securityCapabilityAction
    : ACTION
      securityActionReferenceList
      SEMI
    ;


securityActionReferenceList
    : securityActionReference
      (
          COMMA
          securityActionReference
      )*
    ;


securityActionReference
    : qualifiedName
    | expression
    ;


/* ============================================================================
 * 10. RESOURCES
 * ============================================================================
 *
 * Resources are semantic resources, never implicit physical resources.
 */
securityCapabilityResource
    : RESOURCE
      securityResourceReferenceList
      SEMI
    ;


securityResourceReferenceList
    : securityResourceReference
      (
          COMMA
          securityResourceReference
      )*
    ;


securityResourceReference
    : qualifiedName
    | expression
    ;


/* ============================================================================
 * 11. SCOPE
 * ============================================================================
 *
 * Scope is a semantic expression.
 *
 * It may describe:
 *
 *     namespace
 *     workload
 *     data domain
 *     execution context
 *     security domain
 *     temporal condition
 *     resource domain
 *
 * without encoding a physical deployment topology.
 */
securityCapabilityScope
    : SCOPE
      expression
      SEMI
    ;


/* ============================================================================
 * 12. ISSUER
 * ============================================================================
 *
 * The issuer is a symbolic authority reference.
 *
 * This does not authenticate the issuer.
 */
securityCapabilityIssuer
    : ISSUER
      qualifiedName
      SEMI
    ;


/* ============================================================================
 * 13. SECURITY REQUIREMENTS
 * ============================================================================
 *
 * A requirement is not:
 *
 *     a grant;
 *     a runtime authorization decision;
 *     a hardware capability;
 *     a credential.
 *
 * It is source-level security intent.
 */
securityCapabilityRequirement
    : REQUIRES
      securityCapabilityRequirementExpression
      SEMI
    ;


securityCapabilityRequirementExpression
    : securityCapabilityRequirementDisjunction
    ;


securityCapabilityRequirementDisjunction
    : securityCapabilityRequirementConjunction
      (
          OR
          securityCapabilityRequirementConjunction
      )*
    ;


securityCapabilityRequirementConjunction
    : securityCapabilityRequirementPrimary
      (
          AND
          securityCapabilityRequirementPrimary
      )*
    ;


securityCapabilityRequirementPrimary
    : securityCapabilityRequirementAtom
    | LPAREN
      securityCapabilityRequirementExpression
      RPAREN
    ;


securityCapabilityRequirementAtom
    : securityCapabilityReference
    | capabilityReference
    | securityPermissionReference
    | expression
    ;


securityPermissionReference
    : qualifiedName
    ;


/* ============================================================================
 * 14. CONDITIONS
 * ============================================================================
 *
 * Conditions are parsed but never evaluated by the parser.
 */
securityCapabilityCondition
    : WHEN
      expression
      SEMI
    ;


/* ============================================================================
 * 15. DELEGATION
 * ============================================================================
 *
 * Delegation describes source-level authority transfer intent.
 *
 * It does not:
 *
 *     authenticate;
 *     issue credentials;
 *     establish trust;
 *     authorize;
 *     contact a security provider.
 */
securityCapabilityDelegation
    : DELEGATE
      securityDelegationSpecification
      SEMI
    ;


securityDelegationSpecification
    : securityDelegationTarget
    | securityDelegationBody
    ;


securityDelegationTarget
    : securityPrincipalReference
    ;


securityDelegationBody
    : LBRACE
      securityDelegationMember*
      RBRACE
    ;


securityDelegationMember
    : securityDelegationTo
    | securityDelegationPermission
    | securityDelegationAction
    | securityDelegationResource
    | securityDelegationCondition
    | securityDelegationConstraint
    | securityDelegationProperty
    ;


securityDelegationTo
    : TO
      securityPrincipalReference
      SEMI
    ;


securityDelegationPermission
    : PERMISSION
      securityPermissionReferenceList
      SEMI
    ;


securityDelegationAction
    : ACTION
      securityActionReferenceList
      SEMI
    ;


securityDelegationResource
    : RESOURCE
      securityResourceReferenceList
      SEMI
    ;


securityDelegationCondition
    : WHEN
      expression
      SEMI
    ;


securityDelegationConstraint
    : CONSTRAINT
      expression
      SEMI
    ;


securityDelegationProperty
    : identifier
      (
          COLON
          typeExpression
      )?
      (
          ASSIGN
          expression
      )?
      SEMI
    ;


/* ============================================================================
 * 16. ATTENUATION
 * ============================================================================
 *
 * Attenuation can only restrict authority.
 *
 * The grammar records the requested attenuation.
 *
 * Semantic analysis MUST verify monotonic non-expansion.
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
    | securityAttenuationConstraint
    | securityAttenuationProperty
    ;


securityAttenuationPermission
    : PERMISSION
      securityPermissionReferenceList
      SEMI
    ;


securityAttenuationAction
    : ACTION
      securityActionReferenceList
      SEMI
    ;


securityAttenuationResource
    : RESOURCE
      securityResourceReferenceList
      SEMI
    ;


securityAttenuationCondition
    : WHEN
      expression
      SEMI
    ;


securityAttenuationConstraint
    : CONSTRAINT
      expression
      SEMI
    ;


securityAttenuationProperty
    : identifier
      (
          COLON
          typeExpression
      )?
      (
          ASSIGN
          expression
      )?
      SEMI
    ;


/* ============================================================================
 * 17. OPEN-WORLD SECURITY METADATA
 * ============================================================================
 *
 * Unknown/future metadata remains syntactically representable.
 *
 * Example:
 *
 *     assurance_level = security::high;
 *
 *     audit_class: SecurityClass = security::restricted;
 *
 * The parser does not determine the semantic meaning of the property.
 */
securityCapabilityProperty
    : identifier
      (
          COLON
          typeExpression
      )?
      (
          ASSIGN
          expression
      )?
      SEMI
    ;


/* ============================================================================
 * 18. SECURITY CAPABILITY LIST
 * ============================================================================
 *
 * No finite maximum.
 */
securityCapabilityReferenceListExpression
    : securityCapabilityReference
      (
          COMMA
          securityCapabilityReference
      )*
      COMMA?
    ;


/* ============================================================================
 * 19. SECURITY CAPABILITY SET
 * ============================================================================
 *
 * This is a source-level set of symbolic authority references.
 *
 * It does not grant authority by itself.
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
 * 20. SECURITY CAPABILITY BOOLEAN EXPRESSION
 * ============================================================================
 *
 * Precedence:
 *
 *     OR
 *       lower
 *
 *     AND
 *       higher
 *
 *     primary/group
 *       highest
 *
 * Example:
 *
 *     A or B and C
 *
 * means:
 *
 *     A or (B and C)
 *
 * The parser does not decide whether an expression is satisfiable.
 */
securityCapabilityExpression
    : securityCapabilityDisjunction
    ;


securityCapabilityDisjunction
    : securityCapabilityConjunction
      (
          OR
          securityCapabilityConjunction
      )*
    ;


securityCapabilityConjunction
    : securityCapabilityPrimary
      (
          AND
          securityCapabilityPrimary
      )*
    ;


securityCapabilityPrimary
    : securityCapabilityReference
    | LPAREN
      securityCapabilityExpression
      RPAREN
    ;


/* ============================================================================
 * 21. CAPABILITY REQUIREMENT COMPOSITION
 * ============================================================================
 *
 * This rule is deliberately different from:
 *
 *     securityCapabilityExpression
 *
 * because requirements may refer to:
 *
 *     security capabilities;
 *     computational capabilities;
 *     permissions;
 *     ordinary security expressions.
 */
securityCapabilityRequirementExpressionRoot
    : securityCapabilityRequirementDisjunction
    ;


/* ============================================================================
 * 22. SECURITY CAPABILITY MEMBER EXPRESSION
 * ============================================================================
 *
 * This is a reusable expression boundary for future security composition
 * grammars.
 */
securityCapabilityMemberExpression
    : securityCapabilityExpression
    | securityCapabilityRequirementExpression
    | expression
    ;


/* ============================================================================
 * 23. STRUCTURAL VALIDATION BOUNDARY
 * ============================================================================
 *
 * The grammar intentionally does NOT determine:
 *
 *     - whether the capability exists;
 *     - whether a principal exists;
 *     - whether a permission exists;
 *     - whether an action exists;
 *     - whether a resource exists;
 *     - whether an issuer is trusted;
 *     - whether delegation is authorized;
 *     - whether attenuation is monotonic;
 *     - whether a condition is satisfiable;
 *     - whether a capability is available;
 *     - whether a target supports a capability;
 *     - whether a policy allows an operation;
 *     - whether credentials are valid.
 *
 * Those are semantic/security/runtime responsibilities.
 */


/* ============================================================================
 * 24. INTEGRATION CONTRACT
 * ============================================================================
 *
 * SECURITY COMPOSITION
 * --------------------
 *
 * grammar/security/security.g4 already owns the security composition root.
 *
 * It imports:
 *
 *     SecurityCapabilities
 *
 * and therefore receives:
 *
 *     securityCapabilityDeclaration
 *
 * SecurityCapabilities MUST NOT import Security.
 *
 *
 * PERMISSION INTEGRATION
 * ----------------------
 *
 * grammar/security/permissions.g4 owns:
 *
 *     permissionDeclaration
 *     authorizationPolicyDeclaration
 *     authorizationRule
 *     grantDeclaration
 *     revokeDeclaration
 *
 * This file references permissions symbolically.
 *
 * It does not redefine permission semantics.
 *
 *
 * IDENTITY INTEGRATION
 * --------------------
 *
 * grammar/security/identity.g4 / identifiers.g4 own identity syntax.
 *
 * This file references principals symbolically.
 *
 * It does not authenticate them.
 *
 *
 * CORE CAPABILITY INTEGRATION
 * ---------------------------
 *
 * grammar/core/capabilities.g4 owns computational capability syntax.
 *
 * This file may reference:
 *
 *     capabilityReference
 *
 * for requirements.
 *
 * A computational capability requirement MUST NOT be interpreted as a
 * security authorization grant.
 *
 *
 * EFFECT INTEGRATION
 * ------------------
 *
 * grammar/effects/capabilities.g4 owns capability requirements attached to
 * effects.
 *
 * It remains independent of this security-authority capability grammar.
 *
 *
 * RESOURCE INTEGRATION
 * --------------------
 *
 * Resources remain abstract.
 *
 * This grammar may reference:
 *
 *     resource names;
 *     resource expressions;
 *     resource requirements.
 *
 * It does not discover or allocate resources.
 *
 *
 * QUANTUM INTEGRATION
 * -------------------
 *
 * A security capability may protect quantum intent.
 *
 * Example:
 *
 *     capability security::quantum_execution {
 *         permission quantum::execute;
 *         action quantum::execute;
 *         resource quantum::program;
 *     }
 *
 * The security grammar does not construct quantum IR.
 *
 * The canonical boundary remains:
 *
 *     quantum::ir
 *
 *
 * HDL / HARDWARE INTEGRATION
 * --------------------------
 *
 * Security authority may be attached to abstract:
 *
 *     hardware::configure
 *     accelerator::execute
 *     hdl::synthesize
 *
 * without selecting physical targets.
 *
 *
 * DISTRIBUTED / NETWORKING INTEGRATION
 * ------------------------------------
 *
 * Principals, resources and actions may refer to distributed/network
 * abstractions through qualified names.
 *
 * No node count, topology or endpoint capacity is encoded here.
 *
 *
 * AST INTEGRATION
 * ---------------
 *
 * This grammar is syntax-only.
 *
 * The frontend AST MUST preserve at least:
 *
 *     declaration identity;
 *     attributes;
 *     visibility;
 *     generic parameters;
 *     inheritance;
 *     ordered members;
 *     permission references;
 *     principal references;
 *     action references;
 *     resource references;
 *     scope expressions;
 *     issuer reference;
 *     requirement expressions;
 *     conditions;
 *     delegation structure;
 *     attenuation structure;
 *     generic properties;
 *     source spans.
 *
 * IMPORTANT:
 *
 * The existing:
 *
 *     src/frontend/ast/node/capabilities/capability.rs
 *
 * represents the language-wide leaf capability identity/version model.
 *
 * It MUST NOT be overloaded to represent this entire security-authority
 * declaration.
 *
 * Security authority declarations require a distinct security semantic/AST
 * representation, while the existing generic Capability node remains the
 * representation for computational capability identities.
 *
 * This avoids contaminating the domain-neutral computational capability node
 * with security-only state.
 *
 *
 * SEMANTIC INTEGRATION
 * --------------------
 *
 * Semantic analysis MUST perform:
 *
 *     name resolution;
 *     duplicate detection;
 *     namespace validation;
 *     permission resolution;
 *     principal resolution;
 *     action/resource resolution;
 *     issuer resolution;
 *     inheritance validation;
 *     delegation authorization analysis;
 *     attenuation monotonicity analysis;
 *     condition typing;
 *     capability compatibility;
 *     policy compatibility;
 *     trust validation;
 *     resource compatibility;
 *     target capability analysis.
 *
 * None of these operations belong in this grammar.
 *
 *
 * IR INTEGRATION
 * --------------
 *
 * Security capability declarations MUST NOT create a second security IR merely
 * because they occur in source.
 *
 * Security semantics become metadata/constraints/authority information in the
 * canonical semantic representation.
 *
 * When the protected operation is quantum, the semantic information may
 * accompany the path to:
 *
 *     quantum::ir
 *
 * without creating a security-specific quantum IR.
 *
 * When the protected operation is HDL/hardware, the security information may
 * accompany the corresponding canonical hardware representation.
 *
 *
 * COMPILER INTEGRATION
 * --------------------
 *
 * The compiler may:
 *
 *     preserve mandatory security requirements;
 *     validate target compatibility;
 *     propagate authority metadata;
 *     reject impossible security requirements;
 *     perform security-aware lowering;
 *     preserve provenance.
 *
 * The compiler MUST NOT treat a security capability declaration as a direct
 * hardware-selection instruction.
 *
 *
 * RUNTIME INTEGRATION
 * -------------------
 *
 * Runtime security systems may evaluate:
 *
 *     credentials;
 *     principals;
 *     trust;
 *     authorization;
 *     capability possession;
 *     policy conditions;
 *     security state.
 *
 * Those evaluations are downstream of parsing.
 *
 *
 * RESILIENCE INTEGRATION
 * ----------------------
 *
 * Security requirements may constrain recovery and fallback.
 *
 * The grammar does not implement:
 *
 *     retry;
 *     recover;
 *     reroute;
 *     reschedule;
 *     recompile;
 *     backend switching;
 *     quarantine.
 *
 * Those remain resilience/runtime responsibilities.
 *
 * ============================================================================
 * LEXER INTEGRATION
 * ============================================================================
 *
 * The production lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * which consumes:
 *
 *     grammar/lexer/tokens.g4
 *
 * SecurityCapabilities MUST therefore use:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * and MUST NOT use:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * ============================================================================
 * REQUIRED LEXER TOKENS
 * ============================================================================
 *
 * The current repository already supplies:
 *
 *     CAPABILITY
 *     ACTION
 *     RESOURCE
 *     REQUIRES
 *     WHEN
 *     CONSTRAINT
 *     EXTENDS
 *     AND
 *     OR
 *     NOT
 *
 * The security grammar additionally requires these reserved words:
 *
 *     PERMISSION
 *     PRINCIPAL
 *     SCOPE
 *     ISSUER
 *     DELEGATE
 *     ATTENUATE
 *
 * They MUST be added to the canonical keyword vocabulary exactly once:
 *
 *     grammar/lexer/keywords.g4
 *
 * Suggested canonical spellings:
 *
 *     PERMISSION : 'permission' ;
 *     PRINCIPAL  : 'principal' ;
 *     SCOPE      : 'scope' ;
 *     ISSUER     : 'issuer' ;
 *     DELEGATE   : 'delegate' ;
 *     ATTENUATE  : 'attenuate' ;
 *
 * No parser grammar should define these tokens locally.
 *
 * ============================================================================
 * REQUIRED SECURITY COMPOSITION
 * ============================================================================
 *
 * `grammar/security/security.g4` should continue to import:
 *
 *     SecurityCapabilities
 *
 * and consume:
 *
 *     securityCapabilityDeclaration
 *
 * It must not duplicate this grammar's declaration/member rules.
 *
 * `grammar/security/permissions.g4` remains the owner of permission
 * declarations and authorization policy syntax.
 *
 * `grammar/security/trust.g4` remains the owner of trust semantics.
 *
 * `grammar/security/identity.g4` remains the identity façade.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing public integration names retained:
 *
 *     SecurityCapabilities
 *     securityCapabilitiesFile
 *     securityCapabilityDeclaration
 *     securityCapabilityReference
 *     securityCapabilityBody
 *     securityCapabilityMember
 *     securityCapabilityPermission
 *     securityCapabilityPrincipal
 *     securityCapabilityAction
 *     securityCapabilityResource
 *     securityCapabilityScope
 *     securityCapabilityIssuer
 *     securityCapabilityRequirement
 *     securityCapabilityCondition
 *     securityCapabilityDelegation
 *     securityCapabilityAttenuation
 *
 * This preserves the repository's existing integration surface while removing
 * the ambiguous/duplicated implementation details.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains NO:
 *
 *     MAX_CAPABILITIES
 *     MAX_SECURITY_CAPABILITIES
 *     MAX_PERMISSIONS
 *     MAX_PRINCIPALS
 *     MAX_ACTIONS
 *     MAX_RESOURCES
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *     MAX_TENSOR_RANK
 *
 * It contains no:
 *
 *     physical device IDs;
 *     physical qubit IDs;
 *     vendor IDs;
 *     backend IDs;
 *     topology;
 *     routing;
 *     scheduling;
 *     calibration;
 *     QEC implementation;
 *     ZQN implementation;
 *     HAL implementation.
 *
 * ============================================================================
 * SECURITY AUDIT
 * ============================================================================
 *
 * This grammar:
 *
 *     does not authenticate;
 *     does not authorize;
 *     does not grant runtime authority;
 *     does not issue credentials;
 *     does not evaluate trust;
 *     does not retrieve secrets;
 *     does not execute cryptography;
 *     does not contact external systems.
 *
 * Parsing a security capability declaration is never proof of authority.
 *
 * ============================================================================
 * SCALABILITY AUDIT
 * ============================================================================
 *
 * The grammar uses unbounded structural repetition:
 *
 *     *
 *     +
 *
 * for:
 *
 *     declarations;
 *     inheritance;
 *     permissions;
 *     principals;
 *     actions;
 *     resources;
 *     members;
 *     requirements;
 *     delegation members;
 *     attenuation members.
 *
 * No language-level capacity ceiling is introduced.
 *
 * "Infinity" therefore means:
 *
 *     arbitrarily large source/semantic structures subject only to actual
 *     implementation and resource availability.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 *     capability security::quantum_execution;
 *
 *     capability security::quantum_execution {
 *         permission quantum::execute;
 *     }
 *
 *     capability security::quantum_execution {
 *         permission quantum::execute;
 *         principal service::quantum_executor;
 *         action quantum::execute;
 *         resource quantum::program;
 *     }
 *
 *     capability security::hardware_configuration {
 *         permission hardware::configure;
 *         action hardware::configure;
 *         resource hardware::resource;
 *     }
 *
 *     capability security::distributed_execution {
 *         permission distributed::execute;
 *         action distributed::execute;
 *         resource distributed::workload;
 *         scope security::production;
 *     }
 *
 *     capability security::delegated_execution {
 *         permission compute::execute;
 *         delegate {
 *             to service::worker;
 *             permission compute::execute;
 *             action compute::execute;
 *             when request::purpose == "research";
 *         };
 *     }
 *
 *     capability security::attenuated_execution {
 *         permission compute::execute;
 *         attenuate {
 *             permission compute::read;
 *             action compute::read;
 *             resource data::research;
 *             when request::purpose == "research";
 *         };
 *     }
 *
 *     capability security::conditional_execution {
 *         permission compute::execute;
 *         when context::environment == "production";
 *         requires trusted::execution;
 *     }
 *
 *     capability security::future_authority {
 *         permission future::security::new_permission;
 *         action future::compute::new_action;
 *         resource future::resource::new_resource;
 *     }
 *
 *     capability security::versioned_authority
 *         extends security::base_authority {
 *         requirement_level = security::high;
 *     }
 *
 *
 * NEGATIVE
 * --------
 *
 *     capability;
 *
 *     capability security::;
 *
 *     capability security::x {
 *         permission;
 *     }
 *
 *     capability security::x {
 *         principal;
 *     }
 *
 *     capability security::x {
 *         action;
 *     }
 *
 *     capability security::x {
 *         resource;
 *     }
 *
 *     capability security::x {
 *         issuer;
 *     }
 *
 *     capability security::x {
 *         delegate {
 *         };
 *     }
 *
 *     capability security::x {
 *         attenuate {
 *         };
 *     }
 *
 *
 * BOUNDARY
 * --------
 *
 * Test:
 *
 *     one member;
 *     many members;
 *     deeply qualified names;
 *     large inheritance sets;
 *     large permission sets;
 *     large principal sets;
 *     large action sets;
 *     large resource sets;
 *     nested expressions;
 *     deeply nested scopes;
 *     many delegation members;
 *     many attenuation members;
 *     unknown/future capability names.
 *
 *
 * CROSS-DOMAIN
 * ------------
 *
 *     security::classical_execution
 *     security::quantum_execution
 *     security::hdl_synthesis
 *     security::hardware_configuration
 *     security::accelerator_execution
 *     security::distributed_execution
 *     security::ai_execution
 *     security::data_access
 *     security::network_execution
 *     future::domain::authority
 *
 *
 * POCO-REAF
 * ---------
 *
 * The same security capability syntax must remain valid whether the protected
 * computation eventually executes on:
 *
 *     embedded hardware;
 *     CPU;
 *     multicore CPU;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     QPU;
 *     simulator;
 *     accelerator;
 *     HPC;
 *     cluster;
 *     distributed infrastructure;
 *     cloud;
 *     future computational substrates.
 *
 * No target-specific security grammar is required.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * The parser should report syntax errors through the normal ANTLR/Rust
 * diagnostic pipeline.
 *
 * This grammar MUST NOT:
 *
 *     panic;
 *     inspect runtime state;
 *     query hardware;
 *     retrieve credentials;
 *     evaluate authorization;
 *     silently repair security syntax.
 *
 * Source spans remain the responsibility of the parser/frontend AST.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] Existing filename is retained.
 * [x] Grammar name is SecurityCapabilities.
 * [x] Canonical lexer is ZamaniLexer.
 * [x] Generic capability ownership remains in core/capabilities.g4.
 * [x] Effect capability ownership remains in effects/capabilities.g4.
 * [x] Permission ownership remains in security/permissions.g4.
 * [x] Identity ownership remains in security identity grammars.
 * [x] No circular import with Security exists.
 * [x] No second security IR is introduced.
 * [x] No second quantum IR is introduced.
 * [x] quantum::ir remains canonical for quantum semantics.
 * [x] Security capability names are open-world.
 * [x] Permissions are open-world.
 * [x] Principals are open-world.
 * [x] Actions are open-world.
 * [x] Resources are open-world.
 * [x] Delegation is declarative.
 * [x] Attenuation is declarative.
 * [x] Conditions are expressions and are not evaluated by the parser.
 * [x] No physical hardware is selected.
 * [x] No fixed resource capacity is encoded.
 * [x] No fixed machine size is encoded.
 * [x] No embedded Rust exists.
 * [x] No semantic predicates exist.
 * [x] No unsafe implementation is required.
 * [x] Rust 1.97 / 1.97.1 compatibility is documented.
 * [x] Deterministic parsing is preserved.
 * [x] Source-level extensibility is preserved.
 * [x] Integration boundaries are explicit.
 *
 * DOWNSTREAM COMPLETION
 * ---------------------
 *
 * Production readiness of the repository additionally requires:
 *
 *     lexer token integration;
 *     parser composition;
 *     frontend AST mapping;
 *     structural validation;
 *     semantic security analysis;
 *     capability/permission resolution;
 *     trust analysis;
 *     diagnostics;
 *     positive tests;
 *     negative tests;
 *     boundary tests;
 *     scalability tests;
 *     compatibility tests.
 *
 * Those downstream components MUST consume this grammar rather than
 * duplicating it.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This grammar answers exactly one question:
 *
 *     "What is the source syntax for security authority capabilities?"
 *
 * It does NOT answer:
 *
 *     "Who is authenticated?"
 *     "Who is authorized?"
 *     "Which credential is valid?"
 *     "Which machine is selected?"
 *     "Which QPU is selected?"
 *     "Which physical resource is selected?"
 *     "How is quantum execution routed?"
 *     "How is HDL synthesized?"
 *     "How is policy enforced?"
 *
 * Those answers belong downstream.
 *
 * ============================================================================
 */