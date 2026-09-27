/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/security/identity.g4
 *
 * Status:
 *     Production integration façade
 *
 * Purpose:
 *     Provide the canonical singular identity integration boundary without
 *     duplicating or renaming the repository's existing identity grammar.
 *
 * Existing identity implementation:
 *
 *     grammar/security/identifiers.g4
 *
 * Existing ANTLR grammar name:
 *
 *     Identities
 *
 * This file therefore imports and re-exports the existing Identities grammar.
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * The repository historically contains a naming mismatch:
 *
 *     identifiers.g4
 *         |
 *         +--> parser grammar Identities
 *
 * while surrounding security documentation and grammars refer to:
 *
 *     identities.g4
 *
 * This file establishes:
 *
 *     grammar/security/identity.g4
 *         |
 *         +--> imports Identities
 *         |
 *         +--> provides the singular identity integration entry point
 *
 * It intentionally does NOT copy the rules from identifiers.g4.
 *
 * This avoids two competing identity grammars.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * Zamani source
 *      |
 *      v
 * Canonical Zamani lexer
 *      |
 *      v
 * Security composition
 *      |
 *      v
 * grammar/security/identity.g4
 *      |
 *      +--> grammar/security/identifiers.g4
 *      |        |
 *      |        +--> parser grammar Identities
 *      |
 *      v
 * Frontend AST
 *      |
 *      v
 * Name / identity / principal semantic analysis
 *      |
 *      +-------------------+--------------------+
 *      |                   |                    |
 *      v                   v                    v
 * Authorization        Capability            Trust
 * analysis             analysis              analysis
 *      |                   |                    |
 *      +-------------------+--------------------+
 *                          |
 *                          v
 *                    Canonical semantics
 *                          |
 *                          v
 *                    IR / compilation
 *                          |
 *                          v
 *                    runtime / target
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - singular identity grammar integration;
 *     - compatibility naming for the identity grammar;
 *     - the identity parser entry point;
 *     - composition of the existing Identities grammar;
 *     - the boundary between security.g4 and identity syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical identifiers;
 *     - keyword definitions;
 *     - Unicode identifier rules;
 *     - principal implementation;
 *     - authorization;
 *     - permissions;
 *     - capabilities;
 *     - trust evaluation;
 *     - authentication;
 *     - credential storage;
 *     - cryptographic execution;
 *     - certificates;
 *     - private keys;
 *     - passwords;
 *     - bearer tokens;
 *     - device discovery;
 *     - hardware discovery;
 *     - resource allocation;
 *     - network communication;
 *     - quantum operations;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - target selection;
 *     - runtime enforcement.
 *
 * Identity declaration ownership remains in:
 *
 *     grammar/security/identifiers.g4
 *
 * Security-wide composition remains in:
 *
 *     grammar/security/security.g4
 *
 * ============================================================================
 * SINGLE SOURCE OF TRUTH
 * ============================================================================
 *
 * There MUST be exactly one implementation of identity declaration syntax.
 *
 * Therefore this file MUST NOT reproduce:
 *
 *     identityDeclaration
 *     identityBody
 *     identityMember
 *     principalDeclaration
 *     principalBody
 *     principalMember
 *     principalGroupDeclaration
 *     identityBindingDeclaration
 *     authorityReference
 *     identityReference
 *     securitySubjectReference
 *
 * from identifiers.g4.
 *
 * Those rules remain owned by:
 *
 *     parser grammar Identities
 *
 * imported below.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Identity syntax is intentionally open-world and unbounded by language-level
 * machine limits.
 *
 * This grammar introduces NO limits for:
 *
 *     identities
 *     principals
 *     groups
 *     members
 *     authorities
 *     devices
 *     nodes
 *     services
 *     machines
 *     quantum systems
 *
 * It MUST NOT introduce constructs such as:
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
 * A program may contain an ordinary numeric value as program data.
 *
 * That is different from imposing a grammar/compiler capacity limit.
 *
 * ============================================================================
 * OPEN-WORLD IDENTITY MODEL
 * ============================================================================
 *
 * Identity kinds remain semantic names rather than a closed parser
 * enumeration.
 *
 * Examples accepted by the underlying grammar include concepts equivalent to:
 *
 *     identity application::user;
 *     identity infrastructure::device;
 *     identity quantum::operator;
 *     identity service::worker;
 *     identity future::entity;
 *
 * The grammar MUST NOT enumerate:
 *
 *     USER
 *     ADMIN
 *     DEVICE
 *     SERVICE
 *     ROBOT
 *     GPU
 *     QPU
 *     NODE
 *
 * as the complete universe of identity kinds.
 *
 * Semantic analysis determines the meaning of the qualified name.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * Identity syntax MUST remain independent of physical target realization.
 *
 * The source language may identify semantic subjects associated with:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     accelerator
 *     embedded system
 *     distributed node
 *     cloud service
 *     future computational substrate
 *
 * without requiring physical device identifiers.
 *
 * This grammar MUST NOT define a universal physical-device syntax such as:
 *
 *     gpu0
 *     qpu7
 *     cpu3
 *     node1024
 *
 * as a language-level identity model.
 *
 * A physical realization, where necessary, belongs downstream.
 *
 * ============================================================================
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * Parsing an identity does NOT authenticate the subject.
 *
 * This grammar MUST NOT:
 *
 *     - authenticate;
 *     - authorize;
 *     - establish trust;
 *     - validate credentials;
 *     - verify certificates;
 *     - verify signatures;
 *     - issue credentials;
 *     - revoke credentials;
 *     - contact an identity provider;
 *     - contact a directory;
 *     - contact a network;
 *     - query hardware;
 *     - access an operating-system identity database;
 *     - access secure enclave state;
 *     - access TPM state;
 *     - access HSM state.
 *
 * These are downstream semantic/runtime responsibilities.
 *
 * ============================================================================
 * SECRET-MATERIAL BOUNDARY
 * ============================================================================
 *
 * Source-level identity syntax MUST NOT be interpreted as a secure secret
 * storage mechanism.
 *
 * The identity grammar MUST NOT introduce dedicated syntax for embedding:
 *
 *     passwords
 *     private keys
 *     secret keys
 *     bearer tokens
 *     session tokens
 *     API secrets
 *     provider credentials
 *     raw authentication material
 *
 * Identity metadata may refer to secure external material through semantic
 * references, but the parser does not resolve or retrieve that material.
 *
 * Semantic security validation MUST reject constructs that violate the
 * repository's secret-material policy.
 *
 * ============================================================================
 * AUTHENTICATION BOUNDARY
 * ============================================================================
 *
 * Authentication is deliberately separate from identity declaration.
 *
 * Conceptually:
 *
 *     identity
 *         |
 *         v
 *     authentication
 *         |
 *         v
 *     authenticated subject
 *         |
 *         v
 *     authorization
 *
 * This file describes only the first layer.
 *
 * Authentication syntax/semantics, where supported, belong to the appropriate
 * security authentication layer and MUST NOT be duplicated here.
 *
 * ============================================================================
 * AUTHORIZATION BOUNDARY
 * ============================================================================
 *
 * Authorization is owned by:
 *
 *     grammar/security/permissions.g4
 *
 * and the security composition layer.
 *
 * Identity syntax may be referenced by authorization rules.
 *
 * Identity syntax MUST NOT:
 *
 *     - grant permissions;
 *     - deny permissions;
 *     - evaluate policies;
 *     - establish authorization decisions.
 *
 * ============================================================================
 * CAPABILITY BOUNDARY
 * ============================================================================
 *
 * Capabilities belong to the repository's canonical capability model.
 *
 * This façade MUST NOT introduce a second capability architecture.
 *
 * Security capability syntax is owned by the existing security capability
 * grammar and the generic capability model.
 *
 * An identity may be referenced by a capability or capability policy, but
 * this file does not evaluate capability possession.
 *
 * ============================================================================
 * TRUST BOUNDARY
 * ============================================================================
 *
 * Trust relationships remain owned by:
 *
 *     grammar/security/trust.g4
 *
 * An identity may reference:
 *
 *     authority
 *     trust domain
 *     provenance
 *     binding
 *
 * but this file MUST NOT evaluate whether that relationship is trusted.
 *
 * Conceptually:
 *
 *     identity
 *        |
 *        v
 *     trust reference
 *        |
 *        v
 *     trust analysis
 *
 * ============================================================================
 * CRYPTOGRAPHY BOUNDARY
 * ============================================================================
 *
 * Cryptographic intent belongs to:
 *
 *     grammar/security/cryptography.g4
 *
 * This grammar does not enumerate algorithms and does not execute
 * cryptography.
 *
 * It MUST NOT create parser-level dependencies on a finite algorithm list.
 *
 * ============================================================================
 * HARDWARE BOUNDARY
 * ============================================================================
 *
 * Hardware identities are semantic references.
 *
 * This grammar does not:
 *
 *     - discover hardware;
 *     - enumerate devices;
 *     - select a device;
 *     - bind a logical identity to a physical device;
 *     - inspect device state;
 *     - inspect QPU state;
 *     - inspect GPU state;
 *     - inspect FPGA state.
 *
 * Hardware realization belongs to:
 *
 *     grammar/hardware/
 *     grammar/resources/
 *     compiler
 *     HAL
 *     runtime
 *
 * ============================================================================
 * DISTRIBUTED COMPUTING
 * ============================================================================
 *
 * Identity may represent a distributed subject.
 *
 * The grammar imposes no limit on:
 *
 *     nodes
 *     regions
 *     clusters
 *     services
 *     replicas
 *     processes
 *     actors
 *     workers
 *
 * Distribution semantics belong to the distributed/runtime layers.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Identity may semantically identify:
 *
 *     quantum operators
 *     quantum services
 *     quantum workloads
 *     quantum execution authorities
 *     quantum resource subjects
 *
 * Example:
 *
 *     identity quantum::operator;
 *
 * This does NOT introduce:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     topology
 *     calibration
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 * HDL / HARDWARE-SOFTWARE CO-DESIGN
 * ============================================================================
 *
 * An identity may be associated semantically with an HDL or hardware
 * component, but this grammar does not describe the hardware itself.
 *
 * For example, an implementation may later associate an identity with:
 *
 *     hardware::accelerator
 *     hardware::controller
 *     hardware::module
 *
 * without requiring this grammar to know the physical implementation.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The underlying Identities grammar preserves source structure.
 *
 * This façade MUST NOT introduce runtime objects.
 *
 * The expected conceptual mapping is:
 *
 *     identity
 *         |
 *         v
 *     identityDeclaration
 *         |
 *         v
 *     frontend AST identity node
 *         |
 *         v
 *     identity semantic representation
 *
 * The AST MUST preserve, as applicable:
 *
 *     - declaration kind;
 *     - qualified name;
 *     - source spelling;
 *     - attributes;
 *     - visibility;
 *     - identity members;
 *     - authority references;
 *     - namespace references;
 *     - bindings;
 *     - provenance;
 *     - lifecycle metadata;
 *     - source span;
 *     - source ordering.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis, not this grammar, determines:
 *
 *     - identity uniqueness;
 *     - reference resolution;
 *     - namespace validity;
 *     - authority validity;
 *     - binding validity;
 *     - attribute typing;
 *     - lifecycle validity;
 *     - provenance validity;
 *     - policy compatibility;
 *     - trust relationships;
 *     - authentication state;
 *     - authorization state;
 *     - capability relationships.
 *
 * The parser only establishes structural validity.
 *
 * ============================================================================
 * SOURCE SPANS
 * ============================================================================
 *
 * The identity entry point MUST preserve the source context supplied by the
 * ANTLR parser.
 *
 * Downstream AST construction is responsible for retaining source spans.
 *
 * This façade MUST NOT discard:
 *
 *     line
 *     column
 *     token interval
 *     source file identity
 *
 * where those values are available through the canonical frontend pipeline.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no randomness;
 *     - no hardware queries;
 *     - no runtime calls.
 *
 * For a fixed token stream, parsing is deterministic.
 *
 * ============================================================================
 * ERROR HANDLING
 * ============================================================================
 *
 * Syntax errors remain parser errors.
 *
 * Identity semantic errors MUST NOT be hidden by parser recovery.
 *
 * Examples of semantic errors include:
 *
 *     unknown identity reference
 *     duplicate identity declaration
 *     invalid binding
 *     invalid authority reference
 *     invalid identity attribute
 *     forbidden secret material
 *
 * These belong to semantic/security diagnostics, not grammar actions.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file is additive and compatibility-oriented.
 *
 * Existing:
 *
 *     grammar/security/identifiers.g4
 *
 * remains unchanged and remains the underlying identity grammar owner.
 *
 * Existing consumers that import:
 *
 *     Identities
 *
 * continue to have the same underlying identity rules.
 *
 * New composition code may use:
 *
 *     Identity
 *
 * as the singular identity integration grammar.
 *
 * No existing major file is renamed by this file.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * SECURITY COMPOSITION
 *
 * The intended integration is:
 *
 *     grammar/security/security.g4
 *             |
 *             +--> Identity
 *             |
 *             +--> capabilities
 *             +--> permissions
 *             +--> trust
 *             +--> cryptography
 *             +--> constraints
 *             +--> privacy
 *
 * The security composition root should use this façade as the identity
 * boundary rather than independently redefining identity syntax.
 *
 * The existing identifiers.g4 remains the implementation source of the
 * imported rules.
 *
 * ============================================================================
 * ANTLR IMPORT CONTRACT
 * ============================================================================
 *
 * The imported grammar name is:
 *
 *     Identities
 *
 * NOT:
 *
 *     identifiers
 *
 * NOT:
 *
 *     identities
 *
 * ANTLR imports grammar names, not filesystem paths.
 *
 * This distinction corrects the repository's historical filename/grammar-name
 * mismatch without renaming an existing file.
 *
 * ============================================================================
 * RUST INTEGRATION
 * ============================================================================
 *
 * This file contains no Rust code.
 *
 * The generated parser must integrate with the repository's existing Rust
 * frontend using:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Edition 2021
 *
 * Compiler/frontend implementation MUST remain safe Rust.
 *
 * This grammar does not require:
 *
 *     unsafe
 *     unsafe fn
 *     unsafe impl
 *     unsafe trait
 *     unsafe blocks
 *
 * ============================================================================
 * CANONICAL FRONTEND INTEGRATION
 * ============================================================================
 *
 *     Source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     Zamani token stream
 *       |
 *       v
 *     Identity
 *       |
 *       v
 *     Identities
 *       |
 *       v
 *     parser tree
 *       |
 *       v
 *     src/frontend/ast/
 *       |
 *       v
 *     semantic/security analysis
 *
 * This grammar MUST NOT create a second frontend AST.
 *
 * ============================================================================
 * IR INTEGRATION
 * ============================================================================
 *
 * Identity information is semantic metadata.
 *
 * It MUST NOT create:
 *
 *     identity IR
 *     security IR
 *     quantum identity IR
 *
 * merely because an identity appears in source.
 *
 * Identity information is lowered through the repository's canonical semantic
 * and IR architecture.
 *
 * Quantum-related identity information MUST NOT create a second quantum IR.
 *
 * The canonical quantum boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 * POCO-REAF INTEGRATION
 * ============================================================================
 *
 * Identity is portable when expressed as semantic intent.
 *
 * The source program may say, conceptually:
 *
 *     identity workload::operator;
 *
 * without deciding whether realization occurs on:
 *
 *     embedded hardware
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     accelerator
 *     cluster
 *     cloud
 *     future computational substrate
 *
 * Physical realization remains downstream.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar intentionally contains no:
 *
 *     MAX_IDENTITIES
 *     MAX_PRINCIPALS
 *     MAX_GROUPS
 *     MAX_MEMBERS
 *     MAX_AUTHORITIES
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_USERS
 *     MAX_SERVICES
 *
 * It contains no physical resource enumeration.
 *
 * It contains no finite identity-kind enumeration.
 *
 * It contains no fixed cryptographic algorithm enumeration.
 *
 * It contains no fixed hardware enumeration.
 *
 * ============================================================================
 * SECURITY PROPERTY
 * ============================================================================
 *
 * Parsing an identity declaration MUST NOT be interpreted as proof of:
 *
 *     authenticity
 *     authorization
 *     trust
 *     ownership
 *     possession
 *     capability
 *     hardware identity
 *     credential validity
 *
 * Those properties require downstream evidence and analysis.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] It does not duplicate identity syntax.
 *     [x] It does not rename identifiers.g4.
 *     [x] It resolves the singular/plural integration boundary.
 *     [x] It imports the actual Identities grammar.
 *     [x] It exposes a canonical identity entry point.
 *     [x] It introduces no machine-size limits.
 *     [x] It introduces no physical-device limits.
 *     [x] It contains no Rust actions.
 *     [x] It contains no semantic predicates.
 *     [x] It contains no unsafe code.
 *     [x] It remains target independent.
 *     [x] It preserves the canonical AST boundary.
 *     [x] It preserves the canonical semantic pipeline.
 *     [x] It preserves quantum::ir as the quantum IR boundary.
 *     [x] It separates identity from authentication.
 *     [x] It separates identity from authorization.
 *     [x] It separates identity from trust.
 *     [x] It separates identity from cryptography.
 *     [x] It separates identity from hardware realization.
 *     [x] It supports POCO-REAF.
 *
 * Downstream completion still requires:
 *
 *     - security.g4 integration;
 *     - lexer token conformance;
 *     - parser conformance;
 *     - AST mapping;
 *     - semantic identity analysis;
 *     - security diagnostics;
 *     - positive tests;
 *     - negative tests;
 *     - boundary tests;
 *     - scalability tests;
 *     - compatibility tests.
 *
 * Those are downstream contracts and are deliberately not implemented by
 * this façade.
 *
 * ============================================================================
 */

parser grammar Identity;

options {
    tokenVocab = ZamaniLexer;
}

import Identities;


/*
 * ============================================================================
 * CANONICAL SINGULAR IDENTITY ENTRY POINT
 * ============================================================================
 *
 * This is deliberately a delegation rule.
 *
 * The complete identity syntax remains owned by:
 *
 *     grammar/security/identifiers.g4
 *
 * whose parser grammar is:
 *
 *     Identities
 *
 * Therefore there is exactly one implementation of identity syntax.
 */

identity
    : identityDeclaration
    ;


/*
 * ============================================================================
 * EXPLICIT SECURITY IDENTITY ENTRY POINT
 * ============================================================================
 *
 * This name provides an unambiguous integration point for security.g4 and
 * future composition grammars.
 *
 * It intentionally delegates to the existing identity declaration rule.
 */

securityIdentity
    : identityDeclaration
    ;