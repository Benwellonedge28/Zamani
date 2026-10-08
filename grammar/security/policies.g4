/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/security/policies.g4
 *
 * GRAMMAR
 * -------
 * SecurityPolicies
 *
 * STATUS
 * ------
 * CANONICAL SECURITY POLICY ADAPTER / COMPOSITION FACADE
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar provides the SECURITY-SPECIFIC policy composition boundary.
 *
 * It deliberately does NOT define a second policy language.
 *
 * Universal policy syntax is owned by:
 *
 *     grammar/policies/policy.g4
 *
 * Security authorization-policy syntax is owned by:
 *
 *     grammar/security/permissions.g4
 *
 * This file exists only to provide a stable security-facing composition
 * boundary between those policy facilities and the canonical security
 * composition grammar.
 *
 * The architecture is:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     parser
 *          |
 *          +------------------------------+
 *          |                              |
 *          v                              v
 *     universal policy              security policy
 *     grammar                      adapter
 *          |                              |
 *          |                              v
 *          |                     authorization policy
 *          |                              |
 *          +---------------+--------------+
 *                          |
 *                          v
 *                    domain-neutral AST
 *                          |
 *                          v
 *                  structural validation
 *                          |
 *              +-----------+-----------+
 *              |           |           |
 *              v           v           v
 *           policy      security     provenance
 *           analysis    analysis     analysis
 *              |           |           |
 *              +-----------+-----------+
 *                          |
 *                          v
 *                  canonical semantics
 *                          |
 *             +------------+-------------+
 *             |            |             |
 *             v            v             v
 *        classical     quantum::ir    HDL/hardware
 *             |            |             |
 *             +------------+-------------+
 *                          |
 *                    optimization
 *                          |
 *                      lowering
 *                          |
 *                 routing/scheduling
 *                          |
 *                   resilience/QEC
 *                          |
 *                         ZQN
 *                          |
 *                         HAL
 *                          |
 *                    target runtime
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * ANTLR4 parser grammar.
 *
 * Rust consumer:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust only
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no parser actions;
 *     - no runtime execution;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware inspection;
 *     - no resource discovery;
 *     - no credential access;
 *     - no secret access;
 *     - no cryptographic execution;
 *     - no policy evaluation.
 *
 * Generated Rust parser/frontend code MUST remain safe Rust.
 *
 * The Rust crate/workspace containing the generated frontend should enforce:
 *
 *     #![deny(unsafe_code)]
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - security policy composition;
 *     - the security-policy parser facade;
 *     - the security-policy standalone test boundary;
 *     - the security-policy reference boundary;
 *     - security-specific policy aggregation.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - universal policy declarations;
 *     - universal policy rules;
 *     - generic requirements;
 *     - generic constraints;
 *     - generic capabilities;
 *     - expressions;
 *     - identifiers;
 *     - qualified names;
 *     - authorization-rule syntax;
 *     - permissions;
 *     - identities;
 *     - principals;
 *     - trust;
 *     - cryptography;
 *     - privacy;
 *     - policy evaluation;
 *     - authorization evaluation;
 *     - capability resolution;
 *     - resource negotiation;
 *     - hardware discovery;
 *     - runtime enforcement.
 *
 * Universal policy syntax remains owned by:
 *
 *     grammar/policies/policy.g4
 *
 * Security authorization policy syntax remains owned by:
 *
 *     grammar/security/permissions.g4
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There must be exactly one owner for each policy concept.
 *
 * Universal:
 *
 *     policyDeclaration
 *         -> grammar/policies/policy.g4
 *
 * Security authorization:
 *
 *     authorizationPolicyDeclaration
 *         -> grammar/security/permissions.g4
 *
 * Security composition:
 *
 *     securityPolicyDeclaration
 *         -> this file
 *
 * This file MUST NOT redefine:
 *
 *     policyDeclaration
 *     policyBody
 *     policyMember
 *     policyRule
 *     policyRequirement
 *     policyConstraint
 *     policyCapability
 *     policyResource
 *     authorizationPolicyDeclaration
 *     authorizationRule
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/core/core.g4
 *     grammar/expressions/expressions.g4
 *     grammar/policies/policy.g4
 *     grammar/security/permissions.g4
 *
 * LEXICAL DEPENDENCY:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The lexer itself is composed through:
 *
 *     grammar/lexer/lexer.g4
 *     grammar/lexer/tokens.g4
 *
 * EXPORTS:
 *
 *     securityPolicyDeclaration
 *     securityPolicyReference
 *     securityPolicyReferenceList
 *     securityPolicyFile
 *     securityPolicyConstruct
 *
 * CONSUMED_BY:
 *
 *     grammar/security/security.g4
 *     security parser tooling
 *     security conformance tests
 *     security AST construction
 *
 * AST_OWNER:
 *
 *     domain-neutral frontend AST
 *
 * SEMANTIC_OWNER:
 *
 *     security policy semantic analysis
 *     universal policy semantic analysis
 *
 * IR_OWNER:
 *
 *     canonical semantic/IR layers
 *
 *     classical IR
 *     quantum::ir
 *     HDL/hardware representation
 *     distributed representation
 *     deployment representation
 *
 * TEST_OWNER:
 *
 *     grammar/tests/security/policies/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/security.md
 *     grammar/spec/policies.md
 *     grammar/specification/
 *
 * ============================================================================
 * IMPORTS
 * ============================================================================
 *
 * ANTLR imports are grammar-name imports, not filesystem paths.
 *
 * ============================================================================
 */

parser grammar SecurityPolicies;

options {
    tokenVocab = ZamaniLexer;
}

import
    Core,
    Expressions,
    Policy,
    Permissions
;


/*
 * ============================================================================
 * 1. STANDALONE SECURITY POLICY FILE
 * ============================================================================
 *
 * This is the isolated parser entry point for security-policy conformance
 * tests and security tooling.
 *
 * It does not define the complete Zamani source unit.
 *
 * The canonical source parser remains responsible for complete programs.
 *
 * No fixed number of declarations is imposed.
 */

securityPolicyFile
    : securityPolicyDeclaration*
      EOF
    ;


/*
 * ============================================================================
 * 2. SECURITY POLICY DECLARATION
 * ============================================================================
 *
 * Security policy declarations are authorization policies whose syntax is
 * owned by grammar/security/permissions.g4.
 *
 * This facade deliberately delegates rather than copying the declaration.
 *
 * This prevents the historical problem of having two policy grammars that
 * gradually diverge.
 */

securityPolicyDeclaration
    : authorizationPolicyDeclaration
    ;


/*
 * ============================================================================
 * 3. SECURITY POLICY CONSTRUCT
 * ============================================================================
 *
 * Stable single-construct tooling boundary.
 */

securityPolicyConstruct
    : securityPolicyDeclaration
    ;


/*
 * ============================================================================
 * 4. SECURITY POLICY REFERENCE
 * ============================================================================
 *
 * A policy reference is an open-world qualified name.
 *
 * The reference does not prove that the policy exists.
 *
 * Name resolution belongs downstream.
 *
 * This deliberately does not enumerate:
 *
 *     security providers
 *     authorization systems
 *     policy engines
 *     vendors
 *     operating systems
 *     devices
 *     CPUs
 *     GPUs
 *     FPGAs
 *     QPUs
 *     nodes
 *     clusters
 *     clouds
 *     future policy mechanisms
 */

securityPolicyReference
    : qualifiedName
    ;


securityPolicyReferenceList
    : securityPolicyReference
      (
          COMMA
          securityPolicyReference
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 5. POLICY BINDING
 * ============================================================================
 *
 * A security policy reference can be attached to a security context.
 *
 * The actual relationship is determined by semantic analysis.
 *
 * Examples of semantic uses include:
 *
 *     policy security::execution_policy;
 *
 *     policy security::trusted_execution;
 *
 * The grammar deliberately keeps the target of the binding open-world.
 *
 * No implementation-specific policy engine is required.
 */

securityPolicyBinding
    : POLICY
      securityPolicyReference
      SEMICOLON
    ;


/*
 * ============================================================================
 * 6. POLICY REQUIREMENT REFERENCE
 * ============================================================================
 *
 * This boundary allows security consumers to refer to a policy as part of
 * an existing requirement expression without creating a new requirement
 * language.
 *
 * The canonical requirement semantics remain owned by Core/Requirements.
 */

securityPolicyRequirementReference
    : securityPolicyReference
    ;


/*
 * ============================================================================
 * 7. POLICY PREFERENCE REFERENCE
 * ============================================================================
 *
 * This is a semantic reference boundary only.
 *
 * Preference semantics remain owned by the universal policy/resource system.
 */

securityPolicyPreferenceReference
    : securityPolicyReference
    ;


/*
 * ============================================================================
 * 8. POLICY COMPOSITION REFERENCE
 * ============================================================================
 *
 * Composition is represented as references.
 *
 * Whether composition means:
 *
 *     inheritance
 *     conjunction
 *     layering
 *     delegation
 *     specialization
 *     replacement
 *     attenuation
 *
 * is a semantic concern.
 *
 * This grammar does not select one interpretation.
 */

securityPolicyCompositionReference
    : securityPolicyReference
    ;


securityPolicyCompositionReferenceList
    : securityPolicyCompositionReference
      (
          COMMA
          securityPolicyCompositionReference
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 9. SECURITY POLICY METADATA REFERENCE
 * ============================================================================
 *
 * Security policy metadata is deliberately represented by an ordinary
 * qualified-name reference.
 *
 * Metadata interpretation belongs to semantic analysis.
 *
 * This avoids creating a closed metadata vocabulary.
 */

securityPolicyMetadataReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 10. SECURITY POLICY STATUS REFERENCE
 * ============================================================================
 *
 * Policy status is open-world.
 *
 * The grammar does not enumerate states such as:
 *
 *     active
 *     inactive
 *     pending
 *     revoked
 *     expired
 *
 * Those are semantic values and may evolve independently of syntax.
 */

securityPolicyStatusReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 11. SECURITY POLICY TARGET REFERENCE
 * ============================================================================
 *
 * A target is a semantic expression/reference.
 *
 * It does not mean a physical machine.
 *
 * A target may represent:
 *
 *     a security domain
 *     a workload
 *     a resource class
 *     an operation
 *     a service
 *     a capability
 *     a deployment context
 *     a quantum computation
 *     a hardware abstraction
 *     a distributed execution context
 *     a future semantic object
 *
 * Physical realization is downstream.
 */

securityPolicyTargetReference
    : expression
    ;


/*
 * ============================================================================
 * 12. SECURITY POLICY SET
 * ============================================================================
 *
 * This is an unbounded collection boundary.
 *
 * No fixed policy count is encoded.
 */

securityPolicySet
    : LBRACE
      securityPolicyReferenceList?
      RBRACE
    ;


/*
 * ============================================================================
 * 13. SECURITY POLICY APPLICATION
 * ============================================================================
 *
 * This expresses that a policy applies to an abstract target.
 *
 * It does NOT evaluate or enforce the policy.
 *
 * Example:
 *
 *     policy security::execution
 *         for security::trusted_execution;
 *
 * The surrounding security grammar may choose where this construct is legal.
 */

securityPolicyApplication
    : POLICY
      securityPolicyReference
      FOR
      securityPolicyTargetReference
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. SECURITY POLICY COMPOSITION
 * ============================================================================
 *
 * This is only a source-level composition boundary.
 *
 * The actual policy algebra is owned by semantic analysis.
 */

securityPolicyComposition
    : POLICY
      securityPolicyReference
      EXTENDS
      securityPolicyCompositionReferenceList
      SEMICOLON
    ;


/*
 * ============================================================================
 * 15. SECURITY POLICY REQUIREMENT
 * ============================================================================
 *
 * This is a security-facing reference to a policy requirement.
 *
 * It intentionally does not duplicate the universal requirement grammar.
 */

securityPolicyRequirement
    : REQUIRES
      securityPolicyReference
      SEMICOLON
    ;


/*
 * ============================================================================
 * 16. SECURITY POLICY PREFERENCE
 * ============================================================================
 *
 * A preference remains advisory.
 *
 * It MUST NOT silently become a requirement.
 */

securityPolicyPreference
    : PREFER
      securityPolicyReference
      SEMICOLON
    ;


/*
 * ============================================================================
 * 17. SECURITY POLICY MEMBER
 * ============================================================================
 *
 * This is an adapter-level grouping of policy references and applications.
 *
 * The universal policy body remains owned by Policy.
 *
 * The authorization-policy body remains owned by Permissions.
 */

securityPolicyMember
    : securityPolicyBinding
    | securityPolicyApplication
    | securityPolicyComposition
    | securityPolicyRequirement
    | securityPolicyPreference
    ;


/*
 * ============================================================================
 * 18. SECURITY POLICY CONTEXT
 * ============================================================================
 *
 * A security policy context groups policy references without creating another
 * policy declaration syntax.
 *
 * Example:
 *
 *     security policy_context {
 *         policy security::execution;
 *         requires security::trusted_execution;
 *         prefer security::attestation;
 *     }
 *
 * The keyword CONTEXT is intentionally not required here because the
 * surrounding security grammar owns security-context syntax.
 *
 * This rule is therefore a member-level grouping facility rather than a
 * top-level language construct.
 */

securityPolicyMemberList
    : securityPolicyMember*
    ;


/*
 * ============================================================================
 * 19. SEMANTIC REFERENCE BOUNDARIES
 * ============================================================================
 *
 * These aliases exist so semantic consumers can depend on explicit security
 * policy concepts without duplicating syntax.
 */

securityAuthorizationPolicyReference
    : securityPolicyReference
    ;


securityPolicyEngineReference
    : securityPolicyReference
    ;


securityPolicyDomainReference
    : securityPolicyReference
    ;


/*
 * ============================================================================
 * 20. AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve enough source structure for the domain-neutral AST
 * to distinguish:
 *
 *     SecurityPolicyDeclaration
 *     SecurityPolicyReference
 *     SecurityPolicyBinding
 *     SecurityPolicyApplication
 *     SecurityPolicyComposition
 *     SecurityPolicyRequirement
 *     SecurityPolicyPreference
 *
 * The AST must preserve source spans and source ordering.
 *
 * This grammar creates no AST directly.
 *
 * ============================================================================
 * 21. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     policy name resolution
 *     authorization-policy resolution
 *     policy compatibility
 *     policy composition
 *     policy inheritance
 *     policy conflict analysis
 *     requirement satisfaction
 *     preference handling
 *     capability interaction
 *     resource interaction
 *     effect interaction
 *     trust interaction
 *     provenance
 *     authorization
 *     deployment feasibility
 *
 * Parsing does none of these.
 *
 * ============================================================================
 * 22. TYPE CONTRACT
 * ============================================================================
 *
 * Policy references may occur in contexts whose semantic types are determined
 * by the surrounding construct.
 *
 * This grammar does not introduce a policy-specific type system.
 *
 * ============================================================================
 * 23. EFFECT CONTRACT
 * ============================================================================
 *
 * Merely referring to a security policy has no runtime effect.
 *
 * Policy enforcement, authorization, auditing, authentication, attestation,
 * cryptography, network access, or resource acquisition are semantic/runtime
 * concerns.
 *
 * If a policy governs an operation with effects, those effects remain owned by
 * grammar/effects/.
 *
 * ============================================================================
 * 24. CAPABILITY CONTRACT
 * ============================================================================
 *
 * Policy syntax does not grant capabilities.
 *
 * A policy may constrain or require capabilities through the canonical
 * capability system.
 *
 * Capability syntax remains owned by:
 *
 *     grammar/core/capabilities.g4
 *     grammar/security/capabilities.g4
 *
 * ============================================================================
 * 25. RESOURCE CONTRACT
 * ============================================================================
 *
 * This grammar imposes no resource limits.
 *
 * Policy semantics may reference:
 *
 *     memory
 *     compute
 *     storage
 *     bandwidth
 *     latency
 *     energy
 *     reliability
 *     quantum resources
 *     hardware resources
 *     distributed resources
 *
 * without encoding a universal maximum.
 *
 * Resource feasibility is resolved downstream.
 *
 * ============================================================================
 * 26. CONTRACT CONTRACT
 * ============================================================================
 *
 * Security policies may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * but this file does not duplicate contract syntax.
 *
 * Contract ownership remains in the canonical validation/contract grammar.
 *
 * ============================================================================
 * 27. TRUST CONTRACT
 * ============================================================================
 *
 * Trust syntax remains owned exclusively by:
 *
 *     grammar/security/trust.g4
 *
 * A security policy may refer semantically to trust.
 *
 * This file does not define trust relationships or trust evaluation.
 *
 * ============================================================================
 * 28. PROVENANCE CONTRACT
 * ============================================================================
 *
 * Policy provenance remains a semantic concern.
 *
 * The semantic layer may preserve:
 *
 *     source policy
 *     policy version
 *     derivation
 *     composition
 *     decision
 *     evidence
 *     verification
 *     transformation
 *
 * General provenance remains owned by the canonical provenance subsystem.
 *
 * Security provenance remains owned by:
 *
 *     grammar/security/provenance.g4
 *
 * ============================================================================
 * 29. QUANTUM BOUNDARY
 * ============================================================================
 *
 * Security policies may constrain quantum execution.
 *
 * Examples include semantic requirements concerning:
 *
 *     quantum measurement
 *     trusted execution
 *     attestation
 *     isolation
 *     confidentiality
 *     integrity
 *     approved capabilities
 *
 * This grammar MUST NOT define:
 *
 *     qubit identifiers
 *     gate sets
 *     coupling maps
 *     physical QPU identifiers
 *     calibration
 *     routing
 *     scheduling
 *     QEC
 *     quantum backend instructions
 *
 * Quantum semantic representation remains:
 *
 *     quantum::ir
 *
 * Security metadata accompanies that representation downstream.
 *
 * ============================================================================
 * 30. HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Security policy references may constrain hardware realization without
 * naming a physical instance.
 *
 * Valid semantic examples include:
 *
 *     hardware::trusted_execution
 *     hardware::attestation
 *     hardware::isolated_memory
 *
 * The grammar does not enumerate hardware families.
 *
 * ============================================================================
 * 31. BACKEND BOUNDARY
 * ============================================================================
 *
 * Backend selection is not parser behavior.
 *
 * A security policy may survive into:
 *
 *     deployment metadata
 *     security constraints
 *     capability requirements
 *     execution plans
 *     classical IR metadata
 *     quantum::ir metadata
 *     HDL/hardware metadata
 *
 * Mandatory security requirements must survive lowering.
 *
 * A backend that cannot satisfy a mandatory requirement must be rejected by
 * downstream semantic/target analysis rather than causing the source grammar
 * to change.
 *
 * ============================================================================
 * 32. POCO-REAF / SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes no fixed maximum for:
 *
 *     policies
 *     policy references
 *     rules
 *     subjects
 *     principals
 *     resources
 *     capabilities
 *     devices
 *     nodes
 *     CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     memory
 *     trust relationships
 *     policy domains
 *
 * Collections use ANTLR repetition operators.
 *
 * Qualified names remain open-ended.
 *
 * Hardware and resource feasibility remain downstream.
 *
 * The same source-level policy model may therefore be considered for:
 *
 *     tiny embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     simulators
 *     HPC
 *     clusters
 *     distributed systems
 *     cloud deployments
 *     future computational substrates
 *
 * ============================================================================
 * 33. DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source token stream
 *     grammar version
 *     parser configuration
 *
 * It does not depend on:
 *
 *     target hardware
 *     available memory
 *     available devices
 *     network state
 *     credentials
 *     policy-engine state
 *     runtime state
 *     randomness
 *     wall-clock time
 *
 * ============================================================================
 * 34. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics must identify source spans for:
 *
 *     policy
 *     policy names
 *     qualified references
 *     requires
 *     prefer
 *     for
 *     extends
 *
 * Semantic diagnostics are responsible for:
 *
 *     unknown policy
 *     unavailable capability
 *     unsatisfied requirement
 *     policy conflict
 *     invalid authorization relationship
 *     incompatible composition
 *     unsupported target realization
 *
 * ============================================================================
 * 35. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file introduces no new lexical vocabulary.
 *
 * Compatibility aliases belong to:
 *
 *     grammar/compatibility/
 *
 * Historical policy syntax must be migrated there rather than duplicated here.
 *
 * Stable policy syntax remains owned by:
 *
 *     grammar/policies/policy.g4
 *
 * Stable authorization-policy syntax remains owned by:
 *
 *     grammar/security/permissions.g4
 *
 * ============================================================================
 * 36. SECURITY INTEGRATION CONTRACT
 * ============================================================================
 *
 * grammar/security/security.g4 MUST:
 *
 *     import SecurityPolicies
 *
 * and should aggregate:
 *
 *     securityPolicyDeclaration
 *
 * rather than importing/redeclaring another policy grammar in parallel.
 *
 * The security composition root remains the only security-wide aggregation
 * point.
 *
 * ============================================================================
 * 37. REQUIRED INTEGRATION TESTS
 * ============================================================================
 *
 * POSITIVE:
 *
 *     policy security::execution;
 *
 *     policy security::execution
 *         for security::trusted_execution;
 *
 *     policy security::execution
 *         extends security::base;
 *
 *     requires security::trusted_execution;
 *
 *     prefer security::attestation;
 *
 * OPEN-WORLD:
 *
 *     policy future::security::policy;
 *
 *     policy future::security::policy
 *         for future::execution::domain;
 *
 * QUANTUM:
 *
 *     policy quantum::secure_execution
 *         for quantum::service;
 *
 * HARDWARE:
 *
 *     policy hardware::trusted_execution
 *         for hardware::accelerator;
 *
 * DISTRIBUTED:
 *
 *     policy distributed::secure_execution
 *         for distributed::service;
 *
 * NEGATIVE:
 *
 *     policy;
 *
 *     policy security::execution extends;
 *
 *     requires;
 *
 *     prefer;
 *
 *     policy 123;
 *
 *     policy security::execution for;
 *
 * The semantic layer must separately reject unresolved policy references,
 * invalid authorization relationships, unsatisfied requirements, and
 * incompatible policy composition.
 *
 * ============================================================================
 * 38. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     1. It contains no duplicate universal policy grammar.
 *
 *     2. It contains no duplicate authorization-policy grammar.
 *
 *     3. Universal policy syntax remains owned by Policy.
 *
 *     4. Security authorization policy syntax remains owned by Permissions.
 *
 *     5. Security.g4 can consume securityPolicyDeclaration as its sole
 *        security-policy facade.
 *
 *     6. No undefined lexer token is introduced.
 *
 *     7. No embedded Rust or unsafe implementation is required.
 *
 *     8. No target-specific capacity is encoded.
 *
 *     9. Open-world qualified names remain valid.
 *
 *    10. Standalone security-policy parsing is testable.
 *
 *    11. AST ownership is unambiguous.
 *
 *    12. Semantic ownership is unambiguous.
 *
 *    13. Policy provenance can be preserved.
 *
 *    14. Security policy metadata can accompany classical IR,
 *        quantum::ir, and HDL/hardware representations.
 *
 *    15. Policy requirements cannot silently become preferences.
 *
 *    16. Policy references do not perform policy evaluation.
 *
 *    17. The grammar remains independent of target hardware.
 *
 *    18. The grammar remains scalable subject only to implementation
 *        resources.
 *
 * ============================================================================
 */