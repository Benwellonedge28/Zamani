/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/networking/security.g4
 *
 * Grammar:
 *     NetworkingSecurity
 *
 * Status:
 *     Production networking/security integration grammar
 *
 * Purpose:
 *     Provide the single networking-domain parser boundary for security
 *     intent while preserving grammar/security/security.g4 as the canonical
 *     security-language authority.
 *
 * Rust baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     Safe Rust only
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This grammar integrates the networking domain with Zamani's canonical
 * security subsystem.
 *
 * It does NOT define a second security language.
 *
 * It provides a stable networking-facing parser boundary through which
 * security declarations may be consumed when a networking composition
 * explicitly permits them.
 *
 * The canonical security syntax remains owned by:
 *
 *     grammar/security/security.g4
 *
 * Security implementation remains downstream.
 *
 * ============================================================================
 *
 * ARCHITECTURAL PIPELINE
 * ----------------------
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     Networking
 *          |
 *          v
 *     NetworkingSecurity
 *          |
 *          v
 *     canonical Security grammar
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          +--> names
 *          +--> types
 *          +--> effects
 *          +--> capabilities
 *          +--> resources
 *          +--> contracts
 *          +--> policies
 *          +--> provenance
 *          |
 *          v
 *     security semantic analysis
 *          |
 *          v
 *     networking semantic analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed representation
 *          |
 *          v
 *     optimization / lowering
 *          |
 *          v
 *     routing / placement / scheduling / resilience
 *          |
 *          v
 *     ZQN / HAL / target realization
 *
 * This grammar never constructs an IR.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns ONLY:
 *
 *     - networking/security parser integration;
 *     - networkingSecurityConstruct;
 *     - networkingSecurityDeclaration;
 *     - the stable networking-facing security adapter boundary.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     - identifiers;
 *     - qualified names;
 *     - attributes;
 *     - expressions;
 *     - types;
 *     - effects;
 *     - generic requirements;
 *     - generic capabilities;
 *     - generic resources;
 *     - generic constraints;
 *     - policies;
 *     - identities;
 *     - principals;
 *     - authentication;
 *     - authorization;
 *     - permissions;
 *     - cryptography;
 *     - hashes;
 *     - signatures;
 *     - keys;
 *     - certificates;
 *     - privacy;
 *     - trust;
 *     - security constraints;
 *     - secrets;
 *     - secure computation;
 *     - provenance;
 *     - networking endpoints;
 *     - addresses;
 *     - protocols;
 *     - channels;
 *     - messages;
 *     - requests;
 *     - responses;
 *     - services;
 *     - service discovery;
 *     - sockets;
 *     - streams;
 *     - routing;
 *     - distributed execution;
 *     - actors;
 *     - placement;
 *     - scheduling;
 *     - transport implementation;
 *     - hardware discovery;
 *     - runtime enforcement;
 *     - classical IR;
 *     - quantum::ir;
 *     - ZQN;
 *     - HAL.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * The canonical security grammar is:
 *
 *     grammar/security/security.g4
 *
 * That grammar owns the complete security declaration vocabulary.
 *
 * This file MUST NOT reproduce any rule owned by Security.
 *
 * In particular, this file MUST NOT redefine:
 *
 *     securityDeclaration
 *     securityDomainDeclaration
 *     identityDeclaration
 *     principalDeclaration
 *     securityCapabilityDeclaration
 *     permissionDeclaration
 *     authorizationPolicyDeclaration
 *     cryptographicDeclaration
 *     privacyDeclaration
 *     trustRelationshipDeclaration
 *     securityConstraintDeclaration
 *
 * This adapter references the canonical rule instead.
 *
 * ============================================================================
 * NETWORKING SECURITY BOUNDARY
 * ============================================================================
 *
 * Networking constructs already carry security intent through their normal
 * property, requirement, capability, constraint, preference, contract and
 * policy mechanisms.
 *
 * Examples of semantic intent include:
 *
 *     security::authentication
 *     security::authorization
 *     security::confidentiality
 *     security::integrity
 *     security::trusted_execution
 *
 * These names are semantic references.
 *
 * They are NOT closed enumerations in this grammar.
 *
 * A future security capability must be usable without changing this file
 * merely because the capability is new.
 *
 * ============================================================================
 * NETWORKING / SECURITY RESPONSIBILITY SPLIT
 * ============================================================================
 *
 * Networking owns:
 *
 *     - communication intent;
 *     - services;
 *     - endpoints;
 *     - addresses;
 *     - channels;
 *     - protocols;
 *     - requests;
 *     - responses;
 *     - sockets;
 *     - streams;
 *     - routes;
 *     - discovery;
 *     - networking capabilities.
 *
 * Security owns:
 *
 *     - identity;
 *     - authorization;
 *     - authentication;
 *     - permissions;
 *     - security capabilities;
 *     - cryptographic intent;
 *     - privacy;
 *     - trust;
 *     - security constraints;
 *     - secure computation;
 *     - secret-management references;
 *     - security provenance.
 *
 * Networking/security integration means:
 *
 *     networking intent
 *          +
 *     security intent
 *          |
 *          v
 *     combined semantic validation
 *
 * It does NOT mean that networking becomes the owner of security semantics.
 *
 * ============================================================================
 * DEPENDS_ON
 * ============================================================================
 *
 * Direct grammar dependency:
 *
 *     grammar/security/security.g4
 *
 * Canonical transitive dependencies are therefore inherited from Security.
 *
 * The networking aggregate remains responsible for importing this adapter.
 *
 * ============================================================================
 * EXPORTS
 * ============================================================================
 *
 * Public rules:
 *
 *     networkingSecurityConstruct
 *     networkingSecurityDeclaration
 *
 * These are deliberately small adapter rules.
 *
 * ============================================================================
 * CONSUMED_BY
 * ============================================================================
 *
 * Primary consumer:
 *
 *     grammar/networking/networking.g4
 *
 * Downstream consumers:
 *
 *     networking frontend AST
 *     networking semantic analysis
 *     security semantic analysis
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates no AST.
 *
 * The frontend must preserve the canonical Security parse-tree structure
 * when mapping the construct into the domain-neutral AST.
 *
 * Networking security intent must remain distinguishable from:
 *
 *     - transport;
 *     - endpoint;
 *     - routing;
 *     - service discovery;
 *     - resource allocation;
 *     - runtime enforcement.
 *
 * Source spans and source ordering must be preserved.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Security semantics are evaluated by the canonical security semantic layer.
 *
 * Networking semantics are evaluated by the networking semantic layer.
 *
 * Cross-domain validation determines whether the security requirements of a
 * networking construct are compatible with:
 *
 *     - the requested communication semantics;
 *     - endpoint relationships;
 *     - protocol requirements;
 *     - service requirements;
 *     - channel requirements;
 *     - stream requirements;
 *     - route requirements;
 *     - distributed execution;
 *     - target capabilities;
 *     - resource availability;
 *     - execution policy.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * This file introduces no new type system.
 *
 * Security-related values continue to use the canonical expression/type
 * system owned elsewhere.
 *
 * Type checking occurs after parsing.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing this grammar produces no effects.
 *
 * Security-related runtime operations may semantically require effects such
 * as:
 *
 *     io
 *     network
 *     native
 *     foreign
 *     distributed
 *     randomness
 *
 * Authentication, key operations, secure computation, attestation and
 * credential operations are classified by the canonical effect subsystem.
 *
 * This grammar never assigns effects itself.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Security capabilities remain open-world semantic capabilities.
 *
 * Networking may require capabilities such as:
 *
 *     security::authentication
 *     security::authorization
 *     security::confidentiality
 *     security::integrity
 *     security::trusted_execution
 *
 * but this file MUST NOT enumerate the complete capability universe.
 *
 * Capability satisfaction belongs to semantic/resource analysis.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * This file introduces no physical resource limits.
 *
 * Security requirements may ultimately depend upon arbitrary resources,
 * including but not limited to:
 *
 *     memory;
 *     protected memory;
 *     secure storage;
 *     accelerator resources;
 *     cryptographic hardware;
 *     network capacity;
 *     trusted execution facilities;
 *     distributed resources;
 *     quantum resources.
 *
 * Resource quantities remain symbolic or expression-based.
 *
 * There are no grammar-level limits such as:
 *
 *     MAX_KEYS
 *     MAX_IDENTITIES
 *     MAX_POLICIES
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_CONNECTIONS
 *     MAX_SECURITY_OBJECTS
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Security guarantees may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * through the canonical contract system.
 *
 * This file does not create a second contract language.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Security policies remain owned by the canonical policy/security subsystem.
 *
 * Networking may reference those policies.
 *
 * Policy evaluation occurs downstream.
 *
 * Policy decisions MUST NOT be made by the parser.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Security intent attached to networking constructs must preserve provenance.
 *
 * Downstream provenance may record:
 *
 *     source declaration;
 *     security requirement;
 *     policy reference;
 *     capability requirement;
 *     validation result;
 *     transformation;
 *     compiler decision;
 *     target realization.
 *
 * This grammar does not create provenance records itself.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Networking security may apply to quantum/classical communication and
 * quantum-related services.
 *
 * This grammar does NOT own:
 *
 *     qubits;
 *     quantum operations;
 *     quantum states;
 *     measurement;
 *     quantum topology;
 *     QEC;
 *     calibration;
 *     QPU selection.
 *
 * Those remain owned by the quantum subsystem.
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * Security metadata must survive lowering when semantically mandatory.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Security may constrain HDL and hardware communication.
 *
 * This grammar does not own:
 *
 *     ports;
 *     signals;
 *     timing;
 *     synthesis;
 *     placement;
 *     physical interfaces;
 *     hardware topology.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * BACKEND BOUNDARY
 * ============================================================================
 *
 * This grammar does not select:
 *
 *     CPU;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     accelerator;
 *     QPU;
 *     NIC;
 *     secure enclave;
 *     cryptographic accelerator;
 *     cloud provider;
 *     network provider;
 *     transport implementation.
 *
 * Backend selection is determined from:
 *
 *     semantic requirements
 *     capabilities
 *     resources
 *     policies
 *     constraints
 *     deployment context
 *     target availability.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * This grammar imposes no physical or implementation capacity.
 *
 * Security intent remains valid from:
 *
 *     tiny embedded systems
 *     single-process systems
 *     multicore systems
 *     GPU systems
 *     FPGA systems
 *     ASIC systems
 *     accelerator systems
 *     QPU systems
 *     simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     future computational substrates
 *
 * provided that the selected realization satisfies the semantic requirements.
 *
 * "Infinity" means no artificial language-level ceiling.
 *
 * It does not claim infinite physical resources.
 *
 * ============================================================================
 * OPEN-WORLD EXTENSIBILITY
 * ============================================================================
 *
 * New security mechanisms must be introducible through the canonical
 * security/capability/policy/dialect systems without requiring this networking
 * adapter to be edited.
 *
 * This includes future:
 *
 *     authentication mechanisms;
 *     authorization models;
 *     trust mechanisms;
 *     cryptographic mechanisms;
 *     secure-computation mechanisms;
 *     hardware security facilities;
 *     distributed security mechanisms;
 *     quantum-security mechanisms.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     - source tokens;
 *     - grammar version;
 *     - parser composition;
 *     - explicitly selected language/dialect configuration.
 *
 * It MUST NOT depend on:
 *
 *     - current network state;
 *     - credentials;
 *     - available keys;
 *     - hardware state;
 *     - filesystem state;
 *     - environment variables;
 *     - randomness;
 *     - wall-clock time;
 *     - runtime state.
 *
 * ============================================================================
 * SECURITY / SECRET BOUNDARY
 * ============================================================================
 *
 * This grammar MUST NOT permit or require embedding actual secret material.
 *
 * It must never become a mechanism for placing source-level:
 *
 *     passwords;
 *     private keys;
 *     secret keys;
 *     bearer credentials;
 *     access tokens;
 *     recovery secrets;
 *     session secrets
 *
 * into networking declarations.
 *
 * Symbolic references may be resolved by secure runtime/key-management
 * infrastructure where permitted by the canonical security grammar.
 *
 * ============================================================================
 * RUNTIME BOUNDARY
 * ============================================================================
 *
 * Runtime/security infrastructure may perform:
 *
 *     authentication;
 *     authorization;
 *     trust verification;
 *     credential resolution;
 *     key resolution;
 *     attestation;
 *     secure-channel establishment;
 *     auditing;
 *     monitoring;
 *     policy enforcement;
 *     recovery.
 *
 * None of those operations occur during parsing.
 *
 * ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics are limited to syntactic errors inherited from the
 * canonical Security grammar.
 *
 * Semantic diagnostics are responsible for:
 *
 *     - unresolved security references;
 *     - invalid security policy references;
 *     - unsatisfied security capabilities;
 *     - contradictory security requirements;
 *     - invalid security/networking combinations;
 *     - unsupported target security guarantees;
 *     - policy conflicts;
 *     - forbidden effects;
 *     - unavailable secure realization.
 *
 * The parser MUST NOT inspect runtime state to produce these diagnostics.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file introduces a new networking integration boundary without creating
 * new security semantics.
 *
 * Existing security syntax remains governed by:
 *
 *     grammar/security/security.g4
 *
 * Compatibility aliases belong to the canonical compatibility architecture.
 *
 * This file MUST NOT create duplicate token identities or historical security
 * syntax.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * -------------
 *
 * The integration suite must verify:
 *
 *     - networking security construct parsing;
 *     - canonical security declaration delegation;
 *     - security policy references;
 *     - security capability references;
 *     - security requirements;
 *     - networking + security composition;
 *     - classical networking security;
 *     - distributed networking security;
 *     - quantum/classical networking security;
 *     - HDL/hardware networking security;
 *     - arbitrary qualified security names.
 *
 * NEGATIVE TESTS
 * --------------
 *
 * Must reject malformed constructs through the canonical security parser,
 * including:
 *
 *     - malformed security declarations;
 *     - malformed names;
 *     - malformed attributes;
 *     - malformed security bodies;
 *     - malformed expressions;
 *     - invalid punctuation;
 *     - incomplete security constructs.
 *
 * SECURITY BOUNDARY TESTS
 * -----------------------
 *
 * Verify that this grammar does NOT accept networking implementations such as:
 *
 *     - raw credential material;
 *     - private-key literals;
 *     - physical NIC allocation;
 *     - transport implementation;
 *     - router configuration;
 *     - firewall implementation;
 *     - socket creation;
 *     - key generation execution;
 *     - authentication execution.
 *
 * Those belong downstream or to their canonical owners.
 *
 * SCALABILITY TESTS
 * -----------------
 *
 * Verify arbitrary numbers of:
 *
 *     - security declarations;
 *     - networking constructs;
 *     - policies;
 *     - capabilities;
 *     - references;
 *     - nested security structures;
 *
 * subject only to actual implementation resources.
 *
 * No test may encode a universal maximum.
 *
 * DETERMINISM TESTS
 * ----------------
 *
 * Parsing identical source with identical grammar configuration must produce
 * identical parse structure and source locations.
 *
 * CROSS-DOMAIN TESTS
 * ------------------
 *
 * Verify integration with:
 *
 *     networking
 *     distributed
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     AI/data
 *     interoperability
 *
 * without introducing domain-specific security syntax into this file.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST contain none of:
 *
 *     MAX_KEYS
 *     MAX_IDENTITIES
 *     MAX_PRINCIPALS
 *     MAX_POLICIES
 *     MAX_PERMISSIONS
 *     MAX_CAPABILITIES
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_CONNECTIONS
 *     MAX_NETWORK_SIZE
 *     MAX_SECURITY_OBJECTS
 *     MAX_QUANTUM_SECURITY_OBJECTS
 *
 * It also MUST NOT enumerate a finite universe of:
 *
 *     algorithms;
 *     vendors;
 *     providers;
 *     identity systems;
 *     trust anchors;
 *     secure devices;
 *     future security mechanisms.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Dependency direction:
 *
 *     canonical Security
 *             ^
 *             |
 *     NetworkingSecurity
 *             ^
 *             |
 *        Networking
 *             ^
 *             |
 *       ZamaniParser
 *
 * The dependency direction MUST NOT be reversed.
 *
 * Security MUST NOT import NetworkingSecurity.
 *
 * NetworkingSecurity MUST NOT import ZamaniParser.
 *
 * NetworkingSecurity MUST NOT create a second security AST or IR.
 *
 * ============================================================================
 * REQUIRED NETWORKING AGGREGATE CHANGE
 * ============================================================================
 *
 * `grammar/networking/networking.g4` must import:
 *
 *     NetworkingSecurity
 *
 * and add:
 *
 *     | networkingSecurityConstruct
 *
 * to its `networkingConstruct` dispatch.
 *
 * No other networking leaf needs to import this file merely to express
 * ordinary security requirements through properties. Existing networking
 * grammars already preserve that target-independent security-property model.
 *
 * ============================================================================
 * AST OWNER
 * ============================================================================
 *
 *     Networking/security frontend AST adapter
 *
 * The canonical security semantic nodes remain owned by the security
 * frontend/semantic layer.
 *
 * ============================================================================
 * SEMANTIC OWNER
 * ============================================================================
 *
 *     Canonical security semantic subsystem
 *
 * Cross-domain networking validation remains owned by networking semantic
 * analysis.
 *
 * ============================================================================
 * IR OWNER
 * ============================================================================
 *
 *     No IR owned here.
 *
 * Security metadata lowers through the existing canonical IR architecture.
 *
 * Quantum-related security metadata ultimately crosses the:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * ============================================================================
 * TEST OWNER
 * ============================================================================
 *
 *     grammar/tests/networking/security/
 *
 * Required suites:
 *
 *     parser/
 *     semantic/
 *     boundary/
 *     scalability/
 *     compatibility/
 *     determinism/
 *     cross-domain/
 *
 * ============================================================================
 * SPEC OWNER
 * ============================================================================
 *
 *     grammar/spec/security.md
 *     grammar/spec/networking.md
 *
 * If a dedicated networking-security specification is introduced later:
 *
 *     grammar/spec/networking/security.md
 *
 * it becomes the integration specification, while security.md remains the
 * authority for security semantics.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [x] It contains no duplicated security language.
 *     [x] Canonical Security remains the security authority.
 *     [x] Networking receives one stable security adapter.
 *     [x] No target-specific implementation is encoded.
 *     [x] No physical resource limits exist.
 *     [x] No security algorithm catalogue is hard-coded.
 *     [x] No secret material syntax is introduced.
 *     [x] No parser actions exist.
 *     [x] No semantic predicates exist.
 *     [x] No embedded Rust exists.
 *     [x] No unsafe Rust requirement exists.
 *     [x] Quantum boundaries are preserved.
 *     [x] HDL boundaries are preserved.
 *     [x] Resource/capability/policy ownership is preserved.
 *     [x] Provenance remains downstream and traceable.
 *     [x] Positive tests are defined.
 *     [x] Negative tests are defined.
 *     [x] Boundary tests are defined.
 *     [x] Scalability tests are defined.
 *     [x] Cross-domain tests are defined.
 *     [x] Determinism tests are defined.
 *     [x] Compatibility tests are defined.
 *     [x] Integration with networking.g4 is defined in advance.
 *
 * Repository acceptance additionally requires:
 *
 *     [ ] ANTLR generation succeeds.
 *     [ ] Networking aggregate generation succeeds.
 *     [ ] Canonical Security generation succeeds.
 *     [ ] Parser integration succeeds.
 *     [ ] AST mapping succeeds.
 *     [ ] Semantic integration succeeds.
 *     [ ] Safe Rust 1.97+ compilation succeeds.
 *     [ ] No unsafe implementation is introduced.
 *
 * ============================================================================
 */

parser grammar NetworkingSecurity;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * CANONICAL SECURITY DEPENDENCY
 * ============================================================================
 *
 * Security owns the actual security language.
 *
 * This import is deliberately the only domain dependency required here.
 * ============================================================================
 */

import Security;

/*
 * ============================================================================
 * PUBLIC NETWORKING/SECURITY ENTRY POINT
 * ============================================================================
 *
 * This rule is intentionally an adapter rather than a second security
 * declaration grammar.
 * ============================================================================
 */

networkingSecurityConstruct
    : networkingSecurityDeclaration
    ;

/*
 * ============================================================================
 * SECURITY DECLARATION ADAPTER
 * ============================================================================
 *
 * Delegate directly to the canonical Security grammar.
 *
 * Do not copy alternatives from Security.securityDeclaration here.
 *
 * This guarantees that future security features added to the canonical
 * security subsystem remain authoritative instead of being silently forked
 * inside networking.
 * ============================================================================
 */

networkingSecurityDeclaration
    : securityDeclaration
    ;