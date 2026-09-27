/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/security/security.g4
 *
 * Grammar identity:
 *     Security
 *
 * Status:
 *     CANONICAL SECURITY COMPOSITION GRAMMAR
 *
 * Purpose:
 *     Single parser-level composition boundary for Zamani security syntax.
 *
 * This file is intentionally a COMPOSITION ROOT.
 *
 * It MUST NOT duplicate syntax owned by:
 *
 *     grammar/security/identifiers.g4
 *     grammar/security/capabilities.g4
 *     grammar/security/permissions.g4
 *     grammar/security/cryptography.g4
 *     grammar/security/privacy.g4
 *     grammar/security/trust.g4
 *     grammar/security/security-constraints.g4
 *
 * Generic language facilities remain owned by their canonical shared grammars:
 *
 *     grammar/core/*
 *     grammar/types/*
 *     grammar/expressions/*
 *     grammar/effects/*
 *     grammar/resources/*
 *
 * ============================================================================
 * RUST / SAFETY BASELINE
 * ============================================================================
 *
 * Consumer implementation:
 *
 *     Rust 2021
 *     Rust 1.97 / Rust 1.97.1
 *
 * This grammar contains:
 *
 *     - no embedded Rust actions;
 *     - no semantic predicates;
 *     - no unsafe code;
 *     - no filesystem access;
 *     - no network access;
 *     - no runtime execution;
 *     - no hardware discovery;
 *     - no credential discovery;
 *     - no secret access;
 *     - no policy evaluation;
 *     - no cryptographic execution.
 *
 * The generated Rust frontend MUST remain safe Rust.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                    Zamani source
 *                         |
 *                         v
 *                    ZamaniLexer
 *                         |
 *                         v
 *                 ZamaniParser.g4
 *                         |
 *                         v
 *                       Security
 *                         |
 *          +--------------+--------------+
 *          |              |              |
 *          v              v              v
 *       Identity      Capability      Permission
 *          |              |              |
 *          +--------------+--------------+
 *                         |
 *               +---------+---------+
 *               |         |         |
 *               v         v         v
 *          Cryptography Privacy   Trust
 *               |         |         |
 *               +---------+---------+
 *                         |
 *                         v
 *                    Constraints
 *                         |
 *                         v
 *                  frontend AST
 *                         |
 *                         v
 *                 semantic analysis
 *                         |
 *          +--------------+---------------+
 *          |              |               |
 *          v              v               v
 *     classical IR    quantum::ir    HDL/hardware IR
 *          |              |               |
 *          +--------------+---------------+
 *                         |
 *                         v
 *              optimization / lowering
 *                         |
 *               routing / scheduling
 *                         |
 *                  resilience / QEC
 *                         |
 *                         ZQN
 *                         |
 *                         HAL
 *                         |
 *                         v
 *                   target runtime
 *
 * Security syntax is therefore a SOURCE-LEVEL contract.
 *
 * ============================================================================
 * SINGLE OWNERSHIP RULE
 * ============================================================================
 *
 * This file owns ONLY:
 *
 *     - security composition;
 *     - security dispatch;
 *     - security-domain aggregation;
 *     - standalone security-file entry;
 *     - security construct aggregation.
 *
 * This file does NOT own:
 *
 *     - identities;
 *     - principals;
 *     - credentials;
 *     - authentication implementation;
 *     - capabilities;
 *     - permissions;
 *     - authorization policies;
 *     - cryptographic algorithms;
 *     - keys;
 *     - certificates;
 *     - privacy semantics;
 *     - trust semantics;
 *     - security constraints;
 *     - generic requirements;
 *     - generic capabilities;
 *     - generic constraints;
 *     - generic effects;
 *     - types;
 *     - expressions;
 *     - resource allocation;
 *     - hardware discovery;
 *     - quantum IR;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - backend selection;
 *     - runtime enforcement.
 *
 * ============================================================================
 * SPECIALIZED OWNERS
 * ============================================================================
 *
 * Identity / principal syntax:
 *
 *     grammar/security/identifiers.g4
 *
 * Security capability syntax:
 *
 *     grammar/security/capabilities.g4
 *
 * Permission / authorization syntax:
 *
 *     grammar/security/permissions.g4
 *
 * Cryptographic syntax:
 *
 *     grammar/security/cryptography.g4
 *
 * Privacy syntax:
 *
 *     grammar/security/privacy.g4
 *
 * Trust syntax:
 *
 *     grammar/security/trust.g4
 *
 * Security-specific constraints:
 *
 *     grammar/security/security-constraints.g4
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
 * Generic effects:
 *
 *     grammar/effects/effects.g4
 *
 * ============================================================================
 * CRITICAL RULE
 * ============================================================================
 *
 * DO NOT duplicate a rule from an imported security grammar.
 *
 * For example, this grammar MUST NOT define:
 *
 *     authorizationDecision
 *     permissionDeclaration
 *     securityCapabilityDeclaration
 *     cryptographicDeclaration
 *     privacyDeclaration
 *     trustRelationshipDeclaration
 *     securityConstraintDeclaration
 *
 * Those rules already have authoritative owners.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Security syntax expresses PORTABLE SECURITY INTENT.
 *
 * It does not select the machine on which the program will execute.
 *
 * Security syntax therefore MUST NOT impose language-level limits on:
 *
 *     principals
 *     identities
 *     capabilities
 *     permissions
 *     policies
 *     trust relationships
 *     devices
 *     nodes
 *     CPUs
 *     GPUs
 *     FPGAs
 *     QPUs
 *     qubits
 *     memory
 *     storage
 *     security objects
 *
 * There are no grammar-level constants such as:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_DEVICES
 *     MAX_KEYS
 *     MAX_PRINCIPALS
 *     MAX_PERMISSIONS
 *     MAX_POLICIES
 *
 * A program may express an actual resource requirement, for example:
 *
 *     requires security::trusted_execution;
 *
 * or:
 *
 *     requires security::isolated_memory;
 *
 * but satisfiability is determined downstream.
 *
 * ============================================================================
 * OPEN-WORLD SECURITY
 * ============================================================================
 *
 * Security names remain open-world.
 *
 * The grammar MUST NOT require a closed enumeration of:
 *
 *     algorithms
 *     providers
 *     vendors
 *     identity systems
 *     trust anchors
 *     enclaves
 *     hardware roots
 *     cryptographic mechanisms
 *     future security mechanisms
 *
 * Names such as:
 *
 *     security::trusted_execution
 *     quantum::secure_execution
 *     hardware::root_of_trust
 *     future::security::mechanism
 *
 * are semantic references.
 *
 * Their meaning is resolved downstream.
 *
 * ============================================================================
 * SECRET-MATERIAL BOUNDARY
 * ============================================================================
 *
 * This composition grammar MUST NOT introduce syntax for embedding:
 *
 *     passwords
 *     private keys
 *     secret keys
 *     bearer tokens
 *     API secrets
 *     recovery secrets
 *     session secrets
 *
 * Symbolic references to security objects may be legal where their owning
 * grammar permits them.
 *
 * Actual secret material belongs to secure key-management/runtime systems.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Security may apply to quantum computation.
 *
 * This grammar MUST NOT define:
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
 * Security metadata may accompany quantum semantic representations.
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * This grammar never creates or modifies quantum IR.
 *
 * ============================================================================
 * HARDWARE BOUNDARY
 * ============================================================================
 *
 * Security may apply to:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     embedded system
 *     distributed system
 *     future computational substrate
 *
 * but this grammar never selects a concrete target.
 *
 * Target realization belongs downstream to:
 *
 *     capability analysis
 *     resource analysis
 *     compilation
 *     deployment
 *     runtime
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only upon:
 *
 *     - source tokens;
 *     - grammar version;
 *     - composed parser grammar;
 *     - explicitly selected language/dialect configuration.
 *
 * Parsing MUST NOT depend upon:
 *
 *     wall-clock time;
 *     randomness;
 *     filesystem state;
 *     network state;
 *     environment variables;
 *     hardware availability;
 *     runtime state;
 *     credentials;
 *     policy state.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser preserves source structure.
 *
 * The frontend AST/semantic layer is responsible for constructing:
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
 * Security composition MUST NOT introduce a second semantic security model.
 *
 * Source spans and source ordering MUST remain available to downstream
 * diagnostics and provenance.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - whether security names resolve;
 *     - whether identity references are valid;
 *     - whether capabilities exist;
 *     - whether requirements are satisfiable;
 *     - whether permissions are legal;
 *     - whether policies conflict;
 *     - whether trust relationships are valid;
 *     - whether cryptographic requirements are implementable;
 *     - whether privacy requirements can be satisfied;
 *     - whether target capabilities satisfy mandatory security intent;
 *     - whether security requirements survive lowering.
 *
 * None of those decisions belong to this grammar.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Security semantics may be attached to:
 *
 *     classical IR
 *     quantum::ir
 *     HDL/hardware IR
 *     distributed representations
 *     deployment metadata
 *
 * Lowering MUST preserve mandatory security requirements.
 *
 * Optimization MUST NOT remove a security requirement merely because it is
 * inconvenient for a particular target.
 *
 * If a mandatory security requirement cannot be satisfied, semantic/target
 * analysis must report an explicit diagnostic.
 *
 * The parser itself does not decide satisfiability.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime systems may perform:
 *
 *     authentication
 *     authorization
 *     trust verification
 *     credential resolution
 *     capability verification
 *     policy evaluation
 *     secure-environment verification
 *     auditing
 *
 * None of these operations occur in this grammar.
 *
 * ============================================================================
 * RESILIENCE CONTRACT
 * ============================================================================
 *
 * Security metadata may accompany resilience transformations.
 *
 * Recovery, retry, rerouting, rescheduling, checkpoint restoration,
 * recompilation, backend changes, and failover are downstream concerns.
 *
 * A resilience transformation that violates a mandatory security guarantee
 * must be rejected by downstream analysis.
 *
 * This grammar does not implement recovery.
 *
 * ============================================================================
 * IMPORTS
 * ============================================================================
 *
 * Core:
 *
 *     shared names, attributes, visibility, source-level facilities.
 *
 * Types:
 *
 *     canonical type syntax.
 *
 * Expressions:
 *
 *     canonical expression syntax.
 *
 * Effects:
 *
 *     canonical effect syntax.
 *
 * Security-specific grammars:
 *
 *     identity
 *     capabilities
 *     permissions
 *     cryptography
 *     privacy
 *     trust
 *     constraints
 *
 * Generic requirements/capabilities/constraints remain independently owned.
 *
 * ============================================================================
 */

parser grammar Security;

options {
    tokenVocab = ZamaniLexer;
}

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
    SecurityConstraints
;


/*
 * ============================================================================
 * 1. STANDALONE SECURITY FILE
 * ============================================================================
 *
 * This is the public standalone entry point for:
 *
 *     - grammar validation;
 *     - parser tests;
 *     - security tooling;
 *     - documentation tooling;
 *     - isolated security parsing.
 *
 * No finite declaration count is imposed.
 *
 * ============================================================================
 */

securityFile
    : securityDeclaration*
      EOF
    ;


/*
 * ============================================================================
 * 2. SECURITY DECLARATION DISPATCH
 * ============================================================================
 *
 * This is the ONLY security-wide declaration aggregation point.
 *
 * Every alternative delegates to the grammar that owns that syntax.
 *
 * No specialized security syntax is reproduced here.
 *
 * ============================================================================
 */

securityDeclaration
    : securityDomainDeclaration

    // ------------------------------------------------------------------------
    // Identity / principal
    // ------------------------------------------------------------------------
    | identityDeclaration
    | principalDeclaration
    | principalGroupDeclaration
    | identityBindingDeclaration

    // ------------------------------------------------------------------------
    // Security capabilities
    // ------------------------------------------------------------------------
    | securityCapabilityDeclaration

    // ------------------------------------------------------------------------
    // Authorization / permissions
    // ------------------------------------------------------------------------
    | permissionDeclaration
    | authorizationPolicyDeclaration
    | permissionBinding
    | permissionGrantBinding
    | permissionDenyBinding
    | permissionDelegation
    | scopedPermissionDeclaration
    | principalPermissionBinding

    // ------------------------------------------------------------------------
    // Cryptographic intent
    // ------------------------------------------------------------------------
    | cryptographicDeclaration
    | cryptographicRequirementDeclaration
    | cryptographicConstraintDeclaration
    | cryptographicPreferenceDeclaration
    | cryptographicObjectDeclaration

    // ------------------------------------------------------------------------
    // Privacy
    // ------------------------------------------------------------------------
    | privacyDeclaration
    | privacyPolicyDeclaration
    | privacyRequirementDeclaration
    | privacyConstraintDeclaration
    | privacyPreferenceDeclaration
    | privacyPurposeDeclaration
    | privacyClassificationDeclaration
    | privacyObligationDeclaration
    | privacyActionDeclaration

    // ------------------------------------------------------------------------
    // Trust
    // ------------------------------------------------------------------------
    | trustRelationshipDeclaration
    | trustRequirementDeclaration
    | trustPreferenceDeclaration
    | trustAssertionDeclaration

    // ------------------------------------------------------------------------
    // Security-specific constraints
    // ------------------------------------------------------------------------
    | securityConstraintDeclaration
    ;


/*
 * ============================================================================
 * 3. SECURITY DOMAIN
 * ============================================================================
 *
 * A security domain is a source-level grouping/namespace for security
 * declarations.
 *
 * It is NOT:
 *
 *     - a hardware security domain;
 *     - an enclave;
 *     - a trust boundary implementation;
 *     - a runtime security context.
 *
 * ============================================================================
 */

securityDomainDeclaration
    : attributeList?
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
 * 4. SECURITY CONSTRUCT
 * ============================================================================
 *
 * Stable tooling entry point for one security construct.
 *
 * ============================================================================
 */

securityConstruct
    : securityDeclaration
    ;


/*
 * ============================================================================
 * 5. SECURITY DECLARATION LIST
 * ============================================================================
 *
 * Reusable list for parent grammars that already own the surrounding block.
 *
 * ============================================================================
 */

securityDeclarationList
    : securityDeclaration*
    ;


/*
 * ============================================================================
 * 6. SECURITY DOMAIN MEMBER LIST
 * ============================================================================
 */

securityDomainMemberList
    : securityDomainMember*
    ;


/*
 * ============================================================================
 * 7. COMPOSITION INVARIANTS
 * ============================================================================
 *
 * Invariant 1:
 *
 *     This file is a composition root, not a security implementation.
 *
 * Invariant 2:
 *
 *     Every specialized security construct has exactly one grammar owner.
 *
 * Invariant 3:
 *
 *     Generic requirements remain owned by grammar/core/requirements.g4.
 *
 * Invariant 4:
 *
 *     Generic capabilities remain owned by grammar/core/capabilities.g4.
 *
 * Invariant 5:
 *
 *     Generic constraints remain owned by grammar/core/constraints.g4.
 *
 * Invariant 6:
 *
 *     Trust syntax remains owned exclusively by grammar/security/trust.g4.
 *
 * Invariant 7:
 *
 *     Authorization syntax remains owned exclusively by
 *     grammar/security/permissions.g4.
 *
 * Invariant 8:
 *
 *     Cryptographic syntax remains owned exclusively by
 *     grammar/security/cryptography.g4.
 *
 * Invariant 9:
 *
 *     Identity syntax remains owned exclusively by
 *     grammar/security/identifiers.g4.
 *
 * Invariant 10:
 *
 *     Privacy syntax remains owned exclusively by grammar/security/privacy.g4.
 *
 * Invariant 11:
 *
 *     No security grammar creates a second quantum IR.
 *
 * Invariant 12:
 *
 *     quantum::ir remains the canonical quantum semantic boundary.
 *
 * Invariant 13:
 *
 *     Security syntax contains no target-specific resource limits.
 *
 * Invariant 14:
 *
 *     Parsing performs no runtime security operation.
 *
 * Invariant 15:
 *
 *     No secret material is required by grammar syntax.
 *
 * Invariant 16:
 *
 *     Security metadata survives semantic lowering.
 *
 * Invariant 17:
 *
 *     Mandatory requirements cannot silently become preferences.
 *
 * Invariant 18:
 *
 *     Security remains open-world and extensible.
 *
 * Invariant 19:
 *
 *     The generated Rust implementation requires no unsafe code.
 *
 * Invariant 20:
 *
 *     Zamani remains one language rather than separate security/classical/
 *     quantum/HDL languages.
 *
 * ============================================================================
 * 8. POCO-REAF INVARIANTS
 * ============================================================================
 *
 * The security grammar must remain valid for programs executing on:
 *
 *     - tiny embedded targets;
 *     - single processors;
 *     - multicore processors;
 *     - GPUs;
 *     - FPGAs;
 *     - ASICs;
 *     - accelerators;
 *     - QPUs;
 *     - simulators;
 *     - hybrid systems;
 *     - distributed systems;
 *     - HPC systems;
 *     - cloud systems;
 *     - future computational substrates.
 *
 * No target becomes part of the syntax merely because it exists today.
 *
 * ============================================================================
 * 9. SCALABILITY INVARIANTS
 * ============================================================================
 *
 * Repetition is represented using ANTLR repetition operators.
 *
 * There is no language-level maximum on:
 *
 *     declarations
 *     principals
 *     capabilities
 *     permissions
 *     trust relationships
 *     policy objects
 *     security domains
 *     metadata
 *     security references.
 *
 * Actual parser/compiler/resource limits are implementation concerns and must
 * not become grammar semantics.
 *
 * ============================================================================
 * 10. TOOLING CONTRACT
 * ============================================================================
 *
 * The following rules are stable tooling entry points:
 *
 *     securityFile
 *     securityDeclaration
 *     securityConstruct
 *     securityDomainDeclaration
 *
 * They may be consumed by:
 *
 *     formatters
 *     syntax highlighters
 *     IDEs
 *     source indexers
 *     documentation generators
 *     grammar validators
 *     conformance tests.
 *
 * Tooling MUST NOT interpret parsing as security enforcement.
 *
 * ============================================================================
 * 11. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics belong to the parser/frontend.
 *
 * Semantic diagnostics belong to security analysis.
 *
 * Runtime security failures belong to runtime/deployment systems.
 *
 * The parser MUST NOT:
 *
 *     - expose secret material;
 *     - contact external identity providers;
 *     - query hardware;
 *     - evaluate policies;
 *     - silently discard security syntax.
 *
 * ============================================================================
 * 12. TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     - empty security file;
 *     - one security declaration;
 *     - many security declarations;
 *     - security domain;
 *     - identity declarations;
 *     - capability declarations;
 *     - permission declarations;
 *     - cryptographic declarations;
 *     - privacy declarations;
 *     - trust declarations;
 *     - security constraints.
 *
 * Negative:
 *
 *     - malformed security domain;
 *     - malformed qualified name;
 *     - malformed nested declaration;
 *     - malformed specialized declaration;
 *     - unexpected tokens after a security construct.
 *
 * Boundary:
 *
 *     - empty domain;
 *     - deeply nested security domains;
 *     - long qualified names;
 *     - large declaration sets;
 *     - large metadata sets.
 *
 * Cross-domain:
 *
 *     classical + security
 *     quantum + security
 *     HDL + security
 *     hardware + security
 *     distributed + security
 *     AI + security
 *     networking + security
 *     hybrid + security
 *
 * POCO-REAF:
 *
 *     Identical source semantics must not depend on target size.
 *
 * Determinism:
 *
 *     Identical token streams must produce equivalent parse structures.
 *
 * ============================================================================
 * 13. HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden grammar-level concepts include:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_KEYS
 *     MAX_PRINCIPALS
 *     MAX_PERMISSIONS
 *     MAX_POLICIES
 *     MAX_TRUST_RELATIONSHIPS
 *
 * This file contains none of them as language constraints.
 *
 * Numeric literals appearing in source programs remain program semantics and
 * are not interpreted as universal implementation limits.
 *
 * ============================================================================
 * 14. COMPLETION CRITERIA
 * ============================================================================
 *
 * security/security.g4 is COMPLETE when:
 *
 * [x] It is a composition root.
 * [x] It does not duplicate specialized security syntax.
 * [x] It delegates identity syntax.
 * [x] It delegates capability syntax.
 * [x] It delegates authorization syntax.
 * [x] It delegates cryptographic syntax.
 * [x] It delegates privacy syntax.
 * [x] It delegates trust syntax.
 * [x] It delegates security constraints.
 * [x] It does not create a second generic requirement language.
 * [x] It uses the canonical attribute interface.
 * [x] It imposes no hardware/resource maxima.
 * [x] It remains open-world.
 * [x] It contains no Rust actions.
 * [x] It contains no unsafe implementation requirement.
 * [x] It performs no security evaluation.
 * [x] It creates no quantum IR.
 * [x] It preserves the quantum::ir boundary.
 * [x] It provides a standalone security entry point.
 * [x] It provides a universal security declaration dispatcher.
 * [x] It provides stable tooling entry points.
 *
 * Repository-wide generation is complete when:
 *
 * [ ] the canonical Core composition grammar exists;
 * [ ] all imported parser grammars resolve;
 * [ ] the duplicate effects/security.g4 Security grammar is retired as an
 *     active competing owner;
 * [ ] all specialized security grammars generate together;
 * [ ] ZamaniParser.g4 imports this Security grammar successfully;
 * [ ] the frontend AST accepts every security construct;
 * [ ] semantic security analysis consumes every security construct;
 * [ ] security metadata reaches the canonical semantic/IR boundaries;
 * [ ] quantum security metadata reaches quantum::ir without creating a second
 *     quantum IR;
 * [ ] positive, negative, boundary, scalability, determinism and
 *     cross-domain tests pass.
 *
 * ============================================================================
 */