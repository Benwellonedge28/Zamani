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
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This grammar owns the SOURCE-LEVEL syntax for security-authority
 * capabilities.
 *
 * A security-authority capability represents authority that may be:
 *
 *     - named;
 *     - referenced;
 *     - associated with permissions;
 *     - associated with principals;
 *     - constrained to actions;
 *     - constrained to resources;
 *     - scoped;
 *     - associated with an issuer;
 *     - constrained by requirements;
 *     - constrained by trust;
 *     - constrained by policy;
 *     - delegated;
 *     - attenuated;
 *     - conditioned;
 *     - extended with metadata.
 *
 * It is deliberately distinct from:
 *
 *     grammar/core/capabilities.g4
 *
 * Core capabilities describe computational/execution capabilities.
 *
 * This grammar describes SECURITY AUTHORITY.
 *
 *
 * OWNS
 * ----
 *
 *     securityCapabilitiesFile
 *     securityCapabilityDeclaration
 *     securityCapabilityName
 *     securityCapabilityReference
 *     securityCapabilityInheritanceClause
 *     securityCapabilityBody
 *     securityCapabilityMember
 *     securityCapabilityPermission
 *     securityCapabilityPrincipal
 *     securityCapabilityAction
 *     securityCapabilityResource
 *     securityCapabilityScope
 *     securityCapabilityIssuer
 *     securityCapabilityRequirement
 *     securityCapabilityTrust
 *     securityCapabilityPolicy
 *     securityCapabilityCondition
 *     securityCapabilityDelegation
 *     securityCapabilityAttenuation
 *     securityCapabilityProperty
 *
 * DOES NOT OWN
 * -------------
 *
 * This grammar does NOT own:
 *
 *     generic computational capabilities
 *     generic resource declarations
 *     generic requirements
 *     generic constraints
 *     generic policies
 *     identity declarations
 *     authentication
 *     credentials
 *     secrets
 *     cryptographic operations
 *     permission declarations
 *     authorization decisions
 *     trust relationships
 *     trust evaluation
 *     policy evaluation
 *     resource discovery
 *     capability discovery
 *     hardware discovery
 *     target selection
 *     scheduling
 *     routing
 *     optimization
 *     quantum operations
 *     quantum routing
 *     QEC
 *     ZQN
 *     HAL
 *     runtime enforcement
 *
 * Those concepts remain owned by their respective subsystems.
 *
 * ============================================================================
 * ARCHITECTURE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     SecurityCapabilities
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          +--> identity
 *          +--> permissions
 *          +--> authorization
 *          +--> trust
 *          +--> policy
 *          +--> capabilities
 *          +--> resources
 *          +--> effects
 *          +--> provenance
 *          |
 *          v
 *     security semantic model
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed representation
 *          +--> future representations
 *          |
 *          v
 *     optimization / lowering
 *          |
 *          v
 *     routing / scheduling / resilience
 *          |
 *          v
 *     ZQN / HAL / target realization
 *
 * This grammar MUST NOT reverse that dependency direction.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY CONTRACT
 * ============================================================================
 *
 * Security capabilities are OPEN-WORLD.
 *
 * This grammar MUST NOT enumerate:
 *
 *     permissions
 *     principals
 *     actions
 *     resources
 *     issuers
 *     trust providers
 *     policies
 *     security domains
 *     devices
 *     CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     nodes
 *     qubits
 *     memory
 *     threads
 *
 * There are no language-level maxima.
 *
 * In particular, this file contains no:
 *
 *     MAX_CAPABILITIES
 *     MAX_SECURITY_CAPABILITIES
 *     MAX_PRINCIPALS
 *     MAX_PERMISSIONS
 *     MAX_ACTIONS
 *     MAX_RESOURCES
 *     MAX_DELEGATIONS
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *
 * Repetition is represented with normal ANTLR repetition operators.
 *
 * Practical limits remain implementation/resource constraints rather than
 * language semantics.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * This grammar does not select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     cluster
 *     cloud
 *     network
 *     physical device
 *
 * Security capabilities may protect abstract computational capabilities,
 * resources, quantum operations, HDL intent, distributed services, or other
 * semantic objects.
 *
 * Physical realization remains downstream.
 *
 * ============================================================================
 * SECRET-MATERIAL BOUNDARY
 * ============================================================================
 *
 * This grammar MUST NOT embed:
 *
 *     passwords
 *     private keys
 *     secret keys
 *     API keys
 *     bearer tokens
 *     session secrets
 *     recovery secrets
 *     raw credentials
 *
 * References to security objects are symbolic.
 *
 * Secret material belongs to credential/key-management infrastructure.
 *
 * ============================================================================
 * TRUST BOUNDARY
 * ============================================================================
 *
 * Trust syntax is owned by:
 *
 *     grammar/security/trust.g4
 *
 * This grammar may reference a trust declaration symbolically.
 *
 * It does not define trust relationships.
 *
 * ============================================================================
 * POLICY BOUNDARY
 * ============================================================================
 *
 * Policy syntax is owned by the policy subsystem.
 *
 * This grammar may reference a policy symbolically.
 *
 * It does not define policy evaluation or policy bodies.
 *
 * ============================================================================
 * PERMISSION BOUNDARY
 * ============================================================================
 *
 * Permission declarations are owned by:
 *
 *     grammar/security/permissions.g4
 *
 * This grammar only associates permission references with a security
 * capability.
 *
 * ============================================================================
 * IDENTITY BOUNDARY
 * ============================================================================
 *
 * Principals are symbolic references.
 *
 * Identity declaration, authentication, credential verification, and identity
 * lifecycle remain outside this grammar.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * A security capability may protect quantum computation.
 *
 * For example, a semantic capability may eventually refer to:
 *
 *     quantum::execute
 *     quantum::measure
 *     quantum::control
 *
 * This grammar does not define those quantum operations.
 *
 * It does not define:
 *
 *     qubits
 *     physical qubits
 *     gates
 *     circuits
 *     routing
 *     coupling maps
 *     calibration
 *     noise
 *     QEC
 *
 * Quantum semantics remain downstream.
 *
 * The canonical quantum boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Security capabilities may constrain abstract hardware intent.
 *
 * They do not select physical hardware.
 *
 * ============================================================================
 * EFFECT BOUNDARY
 * ============================================================================
 *
 * Security capability use may contribute security-related effects.
 *
 * Effect ownership remains under:
 *
 *     grammar/effects/
 *
 * This grammar does not redefine the universal effect system.
 *
 * ============================================================================
 * PROVENANCE BOUNDARY
 * ============================================================================
 *
 * Security capability declarations and references may generate provenance
 * metadata downstream.
 *
 * Provenance syntax remains owned by the provenance subsystem.
 *
 * This grammar does not implement provenance storage or auditing.
 *
 * ============================================================================
 * DETERMINISM / SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust;
 *     no semantic predicates;
 *     no runtime calls;
 *     no filesystem access;
 *     no network access;
 *     no environment inspection;
 *     no hardware discovery;
 *     no credential access;
 *     no cryptographic execution;
 *     no policy evaluation;
 *     no randomness.
 *
 * Generated Rust integration MUST remain safe Rust.
 *
 * Rust baseline:
 *
 *     Rust 1.97+
 *     Rust 2021
 *
 * The applicable Rust crate SHOULD enforce:
 *
 *     #![forbid(unsafe_code)]
 *
 * ============================================================================
 */

parser grammar SecurityCapabilities;

options {
    tokenVocab = ZamaniLexer;
}

import Core, Types, Expressions;


/* ============================================================================
 * 1. STANDALONE FILE ENTRY
 * ============================================================================
 *
 * This rule is intended for isolated grammar tests and tooling.
 *
 * The production security composition root consumes:
 *
 *     securityCapabilityDeclaration
 *
 * directly.
 */

securityCapabilitiesFile
    : securityCapabilityDeclaration+
      EOF
    ;


/* ============================================================================
 * 2. SECURITY CAPABILITY DECLARATION
 * ============================================================================
 *
 * Canonical forms:
 *
 *     capability security::quantum_execution;
 *
 *     capability security::quantum_execution {
 *         permission quantum::execute;
 *     }
 *
 *     public capability security::quantum_execution {
 *         permission quantum::execute;
 *         principal service::quantum_executor;
 *         action quantum::execute;
 *         resource quantum::program;
 *     }
 *
 * A declaration without a body is a named capability declaration.
 *
 * Semantic validation determines whether an incomplete declaration is
 * sufficient for the selected language mode.
 */

securityCapabilityDeclaration
    : attributeList?
      visibility?
      CAPABILITY
      securityCapabilityName
      securityCapabilityInheritanceClause?
      securityCapabilityBody?
      SEMICOLON?
    ;


/* ============================================================================
 * 3. SECURITY CAPABILITY NAME
 * ============================================================================
 *
 * Security capability names use the canonical qualified-name system.
 *
 * This grammar does not define another identifier language.
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
    : qualifiedName
    ;


/* ============================================================================
 * 5. INHERITANCE
 * ============================================================================
 *
 * Inheritance is declarative composition.
 *
 * Whether two security capabilities may actually be composed is a semantic
 * validation concern.
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
 * 6. SECURITY CAPABILITY BODY
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
    | securityCapabilityTrust
    | securityCapabilityPolicy
    | securityCapabilityCondition
    | securityCapabilityDelegation
    | securityCapabilityAttenuation
    | securityCapabilityProperty
    ;


/* ============================================================================
 * 7. PERMISSION ASSOCIATION
 * ============================================================================
 *
 * Permission declarations remain owned by the permission subsystem.
 *
 * This rule merely associates existing symbolic permissions with the
 * capability.
 */

securityCapabilityPermission
    : PERMISSION
      securityPermissionReferenceList
      SEMICOLON
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
 * 8. PRINCIPAL ASSOCIATION
 * ============================================================================
 *
 * Principals are symbolic references.
 *
 * No identity verification occurs here.
 */

securityCapabilityPrincipal
    : PRINCIPAL
      securityPrincipalReferenceList
      SEMICOLON
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
 * 9. ACTION ASSOCIATION
 * ============================================================================
 *
 * Actions remain open-world symbolic names or expressions.
 */

securityCapabilityAction
    : ACTION
      securityActionReferenceList
      SEMICOLON
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
 * 10. RESOURCE ASSOCIATION
 * ============================================================================
 *
 * Resources are abstract semantic resources.
 *
 * They are not physical devices.
 */

securityCapabilityResource
    : RESOURCE
      securityResourceReferenceList
      SEMICOLON
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
 * Scope is intentionally expressed using the canonical expression grammar.
 *
 * This avoids creating a second scope language.
 */

securityCapabilityScope
    : SCOPE
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 12. ISSUER
 * ============================================================================
 *
 * Issuer is a symbolic reference.
 *
 * Authentication and issuer verification are downstream responsibilities.
 */

securityCapabilityIssuer
    : ISSUER
      qualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 13. SECURITY REQUIREMENT
 * ============================================================================
 *
 * Requirements are source-level constraints.
 *
 * They are not runtime checks performed by the parser.
 *
 * The expression is deliberately delegated to the canonical expression
 * grammar rather than reproducing boolean-expression syntax here.
 */

securityCapabilityRequirement
    : REQUIRES
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 14. TRUST REFERENCE
 * ============================================================================
 *
 * Trust relationships remain owned by:
 *
 *     grammar/security/trust.g4
 *
 * This rule only references a trust object symbolically.
 */

securityCapabilityTrust
    : TRUST
      qualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 15. POLICY REFERENCE
 * ============================================================================
 *
 * Policy declarations and evaluation remain outside this grammar.
 */

securityCapabilityPolicy
    : POLICY
      qualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 16. CONDITIONAL SECURITY CAPABILITY
 * ============================================================================
 *
 * Conditions use canonical expressions.
 *
 * The parser never evaluates them.
 */

securityCapabilityCondition
    : WHEN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 17. DELEGATION
 * ============================================================================
 *
 * Delegation expresses authority-transfer intent.
 *
 * It does NOT:
 *
 *     authenticate either principal;
 *     issue credentials;
 *     establish trust;
 *     evaluate policy;
 *     execute authorization;
 *     contact a security provider.
 *
 * Canonical form:
 *
 *     delegate principal::source
 *         to principal::target
 *         for security::permission;
 *
 * Multiple permissions are supported without an artificial limit.
 */

securityCapabilityDelegation
    : DELEGATE
      securityPrincipalReference
      TO
      securityPrincipalReference
      FOR
      securityPermissionReferenceList
      securityCapabilityConditionClause?
      SEMICOLON
    ;


securityCapabilityConditionClause
    : WHEN
      expression
    ;


/* ============================================================================
 * 18. ATTENUATION
 * ============================================================================
 *
 * Attenuation describes a derived capability with a restricted authority
 * surface.
 *
 * The grammar expresses the intent only.
 *
 * Semantic validation determines whether an attenuation is actually a valid
 * subset of the source capability.
 */

securityCapabilityAttenuation
    : ATTENUATE
      securityCapabilityReference
      WITH
      securityCapabilityAttenuationMember+
      SEMICOLON
    ;


securityCapabilityAttenuationMember
    : securityCapabilityPermission
    | securityCapabilityAction
    | securityCapabilityResource
    | securityCapabilityRequirement
    | securityCapabilityCondition
    | securityCapabilityProperty
    ;


/* ============================================================================
 * 19. EXTENSIBLE PROPERTY
 * ============================================================================
 *
 * Property metadata remains open-world.
 *
 * Example:
 *
 *     property classification = security::restricted;
 *
 * No finite property catalogue is embedded here.
 */

securityCapabilityProperty
    : PROPERTY
      qualifiedName
      (
          ASSIGN
          expression
      )?
      SEMICOLON
    ;


/* ============================================================================
 * 20. EXPLICIT REFERENCE ENTRY POINTS
 * ============================================================================
 *
 * These rules provide stable integration points for downstream grammars.
 *
 * They do not create additional semantics.
 */

securityCapabilityReferenceListExpression
    : securityCapabilityReference
      (
          COMMA
          securityCapabilityReference
      )*
    ;


/* ============================================================================
 * 21. COMPLETION CONTRACT
 * ============================================================================
 *
 * AST CONTRACT
 * ------------
 *
 * A conforming frontend SHOULD map this grammar to domain-neutral nodes
 * equivalent to:
 *
 *     SecurityCapabilityDeclaration
 *     SecurityCapabilityReference
 *     SecurityCapabilityInheritance
 *     SecurityCapabilityPermissionAssociation
 *     SecurityCapabilityPrincipalAssociation
 *     SecurityCapabilityActionAssociation
 *     SecurityCapabilityResourceAssociation
 *     SecurityCapabilityScope
 *     SecurityCapabilityIssuer
 *     SecurityCapabilityRequirement
 *     SecurityCapabilityTrustReference
 *     SecurityCapabilityPolicyReference
 *     SecurityCapabilityCondition
 *     SecurityCapabilityDelegation
 *     SecurityCapabilityAttenuation
 *     SecurityCapabilityProperty
 *
 * The exact Rust type names are owned by the frontend AST implementation.
 *
 *
 * SEMANTIC CONTRACT
 * -----------------
 *
 * Semantic analysis MUST resolve:
 *
 *     capability identity
 *     inheritance
 *     permissions
 *     principals
 *     actions
 *     resources
 *     scope
 *     issuer
 *     requirements
 *     trust references
 *     policy references
 *     conditions
 *     delegation
 *     attenuation
 *     properties
 *
 * It MUST reject semantically invalid combinations after parsing.
 *
 *
 * TYPE CONTRACT
 * -------------
 *
 * This grammar does not introduce a security-specific type system.
 *
 * Expressions are typed by the canonical Zamani type system.
 *
 *
 * EFFECT CONTRACT
 * --------------
 *
 * Capability declarations themselves do not execute effects.
 *
 * Downstream operations using security capabilities may contribute:
 *
 *     security
 *     authorization
 *     identity
 *     audit
 *     network
 *     foreign
 *     native
 *     mutation
 *
 * effects where appropriate.
 *
 *
 * CAPABILITY CONTRACT
 * -------------------
 *
 * Security authority capabilities are distinct from computational
 * capabilities.
 *
 * A semantic implementation MUST NOT collapse them into one overloaded
 * concept merely because both use the word "capability".
 *
 *
 * RESOURCE CONTRACT
 * -----------------
 *
 * Resource references are symbolic.
 *
 * Resource feasibility is determined by the resource/capability negotiation
 * layer.
 *
 * This grammar never discovers or allocates resources.
 *
 *
 * POLICY CONTRACT
 * ---------------
 *
 * Policy references are symbolic.
 *
 * Policy evaluation remains outside this grammar.
 *
 *
 * TRUST CONTRACT
 * --------------
 *
 * Trust references are symbolic.
 *
 * Trust evaluation remains owned by the trust/security semantic layer.
 *
 *
 * PROVENANCE CONTRACT
 * -------------------
 *
 * The frontend SHOULD preserve source spans and declaration identity so that
 * security capability declarations can participate in provenance and audit
 * records downstream.
 *
 *
 * IR CONTRACT
 * -----------
 *
 * This grammar creates no IR.
 *
 * Security capability metadata may become semantic metadata attached to:
 *
 *     classical IR
 *     quantum::ir
 *     HDL/hardware representation
 *     distributed representation
 *     other canonical representations
 *
 * No security-specific replacement for quantum::ir may be introduced.
 *
 *
 * QUANTUM BOUNDARY
 * ----------------
 *
 * Quantum-related security metadata may accompany quantum semantic constructs.
 *
 * The canonical quantum boundary remains:
 *
 *     quantum::ir
 *
 *
 * HDL BOUNDARY
 * ------------
 *
 * Hardware security metadata remains attached to abstract HDL/hardware
 * semantics and is lowered downstream.
 *
 *
 * BACKEND BOUNDARY
 * ----------------
 *
 * Backend selection, device selection, scheduling, routing, credential
 * enforcement, and runtime authorization are downstream responsibilities.
 *
 *
 * DIAGNOSTICS
 * -----------
 *
 * Parser diagnostics MUST cover at least:
 *
 *     missing capability name
 *     malformed qualified name
 *     missing body delimiter
 *     missing permission reference
 *     missing principal reference
 *     missing action reference
 *     missing resource reference
 *     missing scope expression
 *     missing issuer
 *     missing requirement expression
 *     missing trust reference
 *     missing policy reference
 *     malformed delegation
 *     malformed attenuation
 *     malformed property
 *
 * Semantic diagnostics SHOULD cover:
 *
 *     unresolved capability
 *     unresolved permission
 *     unresolved principal
 *     unresolved action
 *     unresolved resource
 *     unresolved issuer
 *     unresolved trust reference
 *     unresolved policy reference
 *     invalid inheritance
 *     invalid delegation
 *     invalid attenuation
 *     conflicting constraints
 *     unauthorized association
 *
 * These semantic conditions MUST NOT be implemented as parser actions.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * -------------
 *
 * capability security::quantum_execution;
 *
 * capability security::quantum_execution {
 *     permission quantum::execute;
 *     principal service::quantum_executor;
 *     action quantum::execute;
 *     resource quantum::program;
 * };
 *
 * capability security::research_execution
 *     extends security::base_execution
 * {
 *     permission research::execute;
 *     principal organization::research;
 *     action compute::execute;
 *     resource compute::workload;
 *     scope security::research;
 *     issuer organization::security_authority;
 *     requires security::trusted_execution;
 *     trust security::research_trust;
 *     policy security::research_policy;
 *     when execution::approved;
 *     property classification = security::restricted;
 * };
 *
 * capability future::security::authority {
 *     permission future::permission::new_authority;
 *     action future::execution::new_action;
 *     resource future::resource::new_resource;
 * };
 *
 *
 * DELEGATION TEST
 * ---------------
 *
 * capability security::delegated_execution {
 *     permission compute::execute;
 *
 *     delegate
 *         principal::controller
 *         to
 *         principal::worker
 *         for
 *         compute::execute;
 * };
 *
 *
 * ATTENUATION TEST
 * ----------------
 *
 * capability security::restricted_execution {
 *     permission compute::execute;
 *
 *     attenuate security::base_execution with
 *         permission compute::execute;
 *         action compute::execute;
 *         resource compute::workload;
 *         requires security::trusted_execution;
 *         when execution::approved;
 *         property classification = security::restricted;
 *     ;
 * };
 *
 *
 * NEGATIVE TESTS
 * --------------
 *
 * The parser MUST reject:
 *
 *     capability;
 *
 *     capability ::security;
 *
 *     capability security::;
 *
 *     capability security::foo {
 *         permission;
 *     };
 *
 *     capability security::foo {
 *         principal;
 *     };
 *
 *     capability security::foo {
 *         action;
 *     };
 *
 *     capability security::foo {
 *         resource;
 *     };
 *
 *     capability security::foo {
 *         requires;
 *     };
 *
 *     capability security::foo {
 *         trust;
 *     };
 *
 *     capability security::foo {
 *         policy;
 *     };
 *
 *     capability security::foo {
 *         delegate;
 *     };
 *
 *
 * BOUNDARY TESTS
 * -------------
 *
 * Test:
 *
 *     deeply qualified names;
 *     large permission sets;
 *     large principal sets;
 *     large action sets;
 *     large resource sets;
 *     large inheritance lists;
 *     many capability declarations;
 *     deeply nested expressions in conditions;
 *     future-domain names;
 *     Unicode identifiers supported by the canonical lexer;
 *     cross-domain capability references;
 *     quantum capability references;
 *     accelerator capability references;
 *     HDL capability references;
 *     distributed capability references;
 *     networking capability references.
 *
 *
 * SCALABILITY TESTS
 * -----------------
 *
 * There MUST be no grammar-level limit on:
 *
 *     number of declarations;
 *     number of permissions;
 *     number of principals;
 *     number of actions;
 *     number of resources;
 *     number of inheritance parents;
 *     number of requirements;
 *     number of properties.
 *
 * Any practical limit comes from available implementation resources.
 *
 *
 * DETERMINISM TESTS
 * -----------------
 *
 * Identical source and identical parser configuration MUST produce identical
 * parse structure.
 *
 * No security capability parse may depend on:
 *
 *     machine;
 *     hardware;
 *     network;
 *     filesystem;
 *     runtime state;
 *     target availability;
 *     resource availability;
 *     randomness;
 *     wall-clock time.
 *
 *
 * COMPATIBILITY
 * -------------
 *
 * Historical spellings MUST be handled by the compatibility subsystem.
 *
 * This grammar MUST NOT create duplicate lexical identities for aliases.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * UPSTREAM
 * --------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/lexer/tokens.g4
 *     grammar/core/
 *     grammar/types/
 *     grammar/expressions/
 *
 *
 * COMPOSITION OWNER
 * -----------------
 *
 *     grammar/security/security.g4
 *
 * The security composition root consumes:
 *
 *     securityCapabilityDeclaration
 *
 *
 * RELATED OWNERS
 * --------------
 *
 *     grammar/core/capabilities.g4
 *         generic computational capabilities
 *
 *     grammar/security/permissions.g4
 *         permission declarations
 *
 *     grammar/security/trust.g4
 *         trust syntax
 *
 *     grammar/security/authorization.g4
 *         authorization syntax
 *
 *     grammar/security/identifiers.g4
 *         identity/principal syntax
 *
 *     grammar/security/provenance.g4
 *         security provenance
 *
 *     grammar/security/sandbox.g4
 *         sandbox boundaries
 *
 *     grammar/policies/
 *         policy syntax
 *
 *     grammar/resources/
 *         resource/capability negotiation
 *
 *     grammar/effects/
 *         effect semantics
 *
 *
 * DOWNSTREAM
 * ----------
 *
 *     domain-neutral AST
 *     structural validation
 *     security semantic analysis
 *     capability resolution
 *     permission resolution
 *     trust analysis
 *     policy analysis
 *     provenance
 *     canonical semantic representation
 *     classical IR
 *     quantum::ir
 *     HDL/hardware semantic representation
 *     target-independent lowering
 *     runtime enforcement
 *
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/lexer/tokens.g4
 *     grammar/core/
 *     grammar/types/
 *     grammar/expressions/
 *
 *
 * EXPORTS
 * -------
 *
 *     securityCapabilitiesFile
 *     securityCapabilityDeclaration
 *     securityCapabilityReference
 *
 *
 * CONSUMED_BY
 * ----------
 *
 *     grammar/security/security.g4
 *     security parser tests
 *     security AST construction
 *
 *
 * AST_OWNER
 * ---------
 *
 *     Rust frontend AST/security semantic model
 *
 *
 * SEMANTIC_OWNER
 * --------------
 *
 *     security capability semantic analysis
 *
 *
 * IR_OWNER
 * --------
 *
 *     canonical semantic/IR layers
 *
 *     quantum security metadata:
 *         quantum::ir
 *
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/security/capabilities/
 *
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/spec/security.md
 *     grammar/specification/security/
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     NO physical device catalogue
 *     NO fixed provider catalogue
 *     NO fixed permission catalogue
 *     NO fixed principal catalogue
 *     NO fixed action catalogue
 *     NO fixed resource catalogue
 *     NO hardware capacities
 *     NO quantum capacities
 *     NO machine capacities
 *     NO MAX_* constants
 *     NO finite security universe
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [ ] ANTLR generation succeeds.
 *
 * [ ] All imported grammars resolve.
 *
 * [ ] Every referenced lexer token exists exactly once in the canonical lexer.
 *
 * [ ] No duplicate parser rule exists elsewhere as the owner of these rules.
 *
 * [ ] Generic capability syntax remains owned by Core.
 *
 * [ ] Permission declaration remains owned by the permission subsystem.
 *
 * [ ] Trust declaration remains owned by Trust.
 *
 * [ ] Policy declaration remains owned by the policy subsystem.
 *
 * [ ] Identity declaration remains owned by the identity subsystem.
 *
 * [ ] Authorization declaration remains owned by Authorization.
 *
 * [ ] No semantic actions exist.
 *
 * [ ] No semantic predicates exist.
 *
 * [ ] No secret material is representable.
 *
 * [ ] No physical target is selected.
 *
 * [ ] No resource is allocated.
 *
 * [ ] No policy is evaluated.
 *
 * [ ] No trust decision is performed.
 *
 * [ ] No authorization decision is performed.
 *
 * [ ] No quantum implementation is introduced.
 *
 * [ ] quantum::ir remains the canonical quantum boundary.
 *
 * [ ] No artificial scalability ceiling exists.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Scalability tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Compatibility tests pass.
 *
 * [ ] Rust 1.97+ frontend integration passes.
 *
 * [ ] Rust integration requires no unsafe code.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */