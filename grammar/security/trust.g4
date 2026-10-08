/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/security/trust.g4
 *
 * Grammar:
 *     Trust
 *
 * Status:
 *     CANONICAL SECURITY TRUST GRAMMAR
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the single parser-level owner of Zamani trust syntax.
 *
 * It defines portable, open-world source syntax for:
 *
 *     - trust relationships;
 *     - trust requirements;
 *     - trust preferences;
 *     - trust assertions;
 *     - trust references;
 *     - trust conditions;
 *     - trust scopes;
 *     - trust evidence;
 *     - trust properties;
 *     - trust status metadata;
 *     - trust composition;
 *     - trust extensions.
 *
 * Trust syntax expresses security intent.
 *
 * It does NOT establish that something is actually trusted.
 *
 * Authentication, attestation, certificate validation, cryptographic
 * verification, policy evaluation, authorization, capability resolution,
 * trust-store lookup, runtime enforcement, and hardware security evaluation
 * belong to downstream semantic/security/runtime infrastructure.
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * ANTLR4 parser grammar.
 *
 * Rust integration:
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
 *     - no filesystem access;
 *     - no network access;
 *     - no environment inspection;
 *     - no credential access;
 *     - no cryptographic execution;
 *     - no hardware discovery;
 *     - no runtime execution;
 *     - no unsafe implementation requirement.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         ZAMANI SOURCE
 *                              |
 *                              v
 *                       ZamaniLexer
 *                              |
 *                              v
 *                       ZamaniParser
 *                              |
 *                              v
 *                          Security
 *                              |
 *                              v
 *                            Trust
 *                              |
 *                              v
 *                       Frontend AST
 *                              |
 *               +--------------+---------------+
 *               |              |               |
 *               v              v               v
 *          name resolution  security       policy analysis
 *                           analysis
 *               |              |               |
 *               +--------------+---------------+
 *                              |
 *                              v
 *                     semantic trust model
 *                              |
 *               +--------------+---------------+
 *               |              |               |
 *               v              v               v
 *          classical       quantum::ir    HDL/hardware
 *                              |
 *                              v
 *                   optimization / lowering
 *                              |
 *                     routing / scheduling
 *                              |
 *                       resilience / QEC
 *                              |
 *                             ZQN
 *                              |
 *                             HAL
 *                              |
 *                           runtime
 *
 * Trust therefore participates in the semantic/security pipeline without
 * becoming an execution IR or target-selection mechanism.
 *
 * ============================================================================
 * SINGLE AUTHORITY
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     trust relationship syntax
 *     trust requirement syntax
 *     trust preference syntax
 *     trust assertion syntax
 *     trust reference syntax
 *     trust condition syntax
 *     trust scope syntax
 *     trust evidence syntax
 *     trust property syntax
 *     trust status syntax
 *     trust composition syntax
 *     trust extension syntax
 *
 * THIS FILE DOES NOT OWN:
 *
 *     identity declaration
 *     principal declaration
 *     authentication
 *     authorization
 *     permissions
 *     capability declarations
 *     generic capabilities
 *     credentials
 *     cryptographic algorithms
 *     keys
 *     certificates
 *     certificate validation
 *     signature verification
 *     privacy implementation
 *     policy evaluation
 *     trust evaluation
 *     trust-store implementation
 *     attestation implementation
 *     secure boot
 *     TPM/HSM/TEE implementation
 *     hardware discovery
 *     resource allocation
 *     target selection
 *     quantum operations
 *     quantum state
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *     optimization
 *     backend selection
 *     runtime enforcement
 *
 * ============================================================================
 * REPOSITORY INTEGRATION
 * ============================================================================
 *
 * Security composition root:
 *
 *     grammar/security/security.g4
 *
 * That file owns security-domain aggregation.
 *
 * It consumes this grammar's public rules:
 *
 *     trustDeclaration
 *
 * It MUST NOT reproduce trust syntax.
 *
 * Specialized security owners remain separate:
 *
 *     grammar/security/identifiers.g4
 *         identity / principal syntax
 *
 *     grammar/security/permissions.g4
 *         permission / authorization syntax
 *
 *     grammar/security/capabilities.g4
 *         security-authority capability syntax
 *
 *     grammar/security/cryptography.g4
 *         cryptographic intent
 *
 *     grammar/security/privacy.g4
 *         privacy intent
 *
 *     grammar/security/provenance.g4
 *         security interpretation of provenance
 *
 *     grammar/security/security-constraints.g4
 *         security-specific constraints
 *
 * Generic language facilities remain owned by:
 *
 *     grammar/core/
 *     grammar/types/
 *     grammar/expressions/
 *
 * ============================================================================
 * LEXER INTEGRATION
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This grammar consumes:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * No trust-specific lexer grammar is introduced here.
 *
 * No new lexical token is required by this file beyond vocabulary already
 * established by the repository's canonical lexer.
 *
 * The trust grammar intentionally relies on existing language-level concepts:
 *
 *     TRUST
 *     FROM
 *     REQUIRES
 *     PREFER
 *     ASSERT
 *     FOR
 *     WITH
 *     IN
 *     WHEN
 *     AS
 *     EXTENDS
 *     USE
 *     THIN_ARROW
 *     ASSIGN
 *
 * This keeps the lexer open-world.
 *
 * ============================================================================
 * OPEN-WORLD TRUST MODEL
 * ============================================================================
 *
 * Trust entities are represented through ordinary Zamani names and
 * expressions.
 *
 * The grammar MUST NOT enumerate:
 *
 *     trust providers
 *     identity providers
 *     certificate authorities
 *     trust anchors
 *     vendors
 *     cloud providers
 *     machines
 *     CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     enclaves
 *     algorithms
 *     cryptographic schemes
 *     principals
 *     services
 *     future trust mechanisms
 *
 * Examples of valid open-world references:
 *
 *     identity::alice
 *     principal::service
 *     authority::organization
 *     security::attestation
 *     hardware::trusted_execution
 *     quantum::service
 *     future::security::mechanism
 *
 * The semantic layer determines what those names mean.
 *
 * Adding a new trust provider, security mechanism, hardware technology,
 * authority model, or cryptographic mechanism therefore does not require
 * modifying this grammar.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Trust syntax describes portable security intent.
 *
 * It MUST NOT permanently select:
 *
 *     a CPU;
 *     a GPU;
 *     an FPGA;
 *     an ASIC;
 *     a QPU;
 *     a node;
 *     a cluster;
 *     a cloud provider;
 *     a network;
 *     a physical security device.
 *
 * Example:
 *
 *     trust execution
 *         from security::authority
 *         -> compute::service;
 *
 * means:
 *
 *     "express a trust relationship between these semantic entities."
 *
 * It does not mean:
 *
 *     "run on a particular physical machine."
 *
 * Physical realization remains downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar imposes no artificial source-language capacity limits.
 *
 * There is no:
 *
 *     MAX_TRUST_RELATIONSHIPS
 *     MAX_TRUST_ANCHORS
 *     MAX_TRUST_PROVIDERS
 *     MAX_PRINCIPALS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_QPUS
 *     MAX_EVIDENCE
 *     MAX_PROPERTIES
 *     MAX_POLICIES
 *     MAX_SECURITY_DOMAINS
 *
 * or equivalent language-level ceiling.
 *
 * ANTLR repetition constructs are used where collections are required.
 *
 * Practical limitations imposed by:
 *
 *     memory
 *     compilation resources
 *     parser configuration
 *     operating-system resources
 *     runtime resources
 *     target resources
 *
 * are implementation/resource concerns and MUST NOT become language
 * semantics.
 *
 * ============================================================================
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * Parsing a trust declaration MUST NOT grant trust.
 *
 * A successfully parsed declaration means only:
 *
 *     the source conforms to the trust grammar.
 *
 * Semantic/security analysis subsequently determines:
 *
 *     - whether references resolve;
 *     - whether identities are meaningful;
 *     - whether authorities are acceptable;
 *     - whether evidence is valid;
 *     - whether conditions are satisfiable;
 *     - whether policy permits the relationship;
 *     - whether capabilities are available;
 *     - whether cryptographic requirements can be satisfied;
 *     - whether provenance requirements are satisfied;
 *     - whether target execution can preserve mandatory security properties.
 *
 * Runtime infrastructure may then perform:
 *
 *     authentication
 *     attestation
 *     certificate validation
 *     signature verification
 *     policy evaluation
 *     capability validation
 *     trust-anchor validation
 *     secure-environment validation
 *
 * None of these operations are performed by this grammar.
 *
 * ============================================================================
 * SECRET-MATERIAL BOUNDARY
 * ============================================================================
 *
 * Trust syntax MUST NOT provide syntax for embedding:
 *
 *     passwords
 *     private keys
 *     secret keys
 *     bearer tokens
 *     API secrets
 *     session secrets
 *     recovery secrets
 *     raw credentials
 *
 * Trust may refer to a symbolic credential/evidence object:
 *
 *     security::credential::application
 *
 * but the actual secret material belongs to secure credential/key-management
 * infrastructure.
 *
 * ============================================================================
 * IDENTITY INTEGRATION
 * ============================================================================
 *
 * Identity and principal syntax remains owned by:
 *
 *     grammar/security/identifiers.g4
 *
 * Trust only references those entities.
 *
 * Example:
 *
 *     trust application
 *         from identity::alice
 *         -> service::compute;
 *
 * The parser does not authenticate identity::alice.
 *
 * ============================================================================
 * AUTHORIZATION INTEGRATION
 * ============================================================================
 *
 * Authorization and permissions remain owned by:
 *
 *     grammar/security/permissions.g4
 *
 * Trust can be consumed by authorization analysis.
 *
 * Trust does NOT define:
 *
 *     allow
 *     deny
 *     grant
 *     revoke
 *     permission
 *
 * This prevents trust and authorization from becoming duplicate semantic
 * systems.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Security authority capabilities remain owned by:
 *
 *     grammar/security/capabilities.g4
 *
 * Generic computational capabilities remain owned by the generic capability
 * subsystem.
 *
 * Trust may reference either through open-world names/expressions.
 *
 * Examples:
 *
 *     security::trusted_execution
 *     security::attestation
 *     quantum::measurement
 *     hardware::isolated_memory
 *
 * Capability satisfaction is downstream.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Trust is policy-consumable metadata.
 *
 * Policy syntax remains owned by the policy subsystem rather than being
 * duplicated here.
 *
 * A trust condition may therefore reference a policy:
 *
 *     when security::production_policy;
 *
 * A policy may in turn consume trust requirements.
 *
 * The parser only preserves the relationship.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Trust syntax itself does not create runtime effects.
 *
 * Security semantics may associate trust requirements with existing effects,
 * such as:
 *
 *     network
 *     native
 *     foreign
 *     distributed
 *     mutation
 *     measurement
 *     quantum
 *
 * Effect ownership remains in grammar/effects/.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Trust may coexist with resource and capability requirements.
 *
 * Example semantic intent:
 *
 *     trust execution
 *         from security::authority
 *         -> quantum::service
 *         {
 *             requires security::attestation;
 *             requires quantum::measurement;
 *         }
 *
 * Resource feasibility is not evaluated by this grammar.
 *
 * Resource analysis occurs downstream.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Security provenance is owned by:
 *
 *     grammar/security/provenance.g4
 *
 * General data lineage remains owned by:
 *
 *     grammar/data/provenance.g4
 *
 * Trust may reference evidence and provenance-related semantic objects without
 * recreating either provenance grammar.
 *
 * The semantic layer may associate a trust decision with:
 *
 *     source
 *     evidence
 *     derivation
 *     verification
 *     attestation
 *     decision
 *     policy
 *     version
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Trust may protect:
 *
 *     quantum computation
 *     quantum-classical execution
 *     quantum services
 *     quantum data
 *     quantum control
 *     quantum hardware
 *
 * This grammar MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     gate sets
 *     pulse data
 *     calibration
 *     coupling maps
 *     physical topology
 *     QEC codes
 *     backend-specific instructions
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * Trust information may accompany quantum semantic operations and their
 * security requirements, but this grammar creates no second quantum IR.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Trust may reference abstract hardware security properties:
 *
 *     hardware::attestation
 *     hardware::trusted_execution
 *     hardware::isolated_memory
 *
 * It MUST NOT select:
 *
 *     CPU 0
 *     GPU 0
 *     FPGA 0
 *     QPU 0
 *     node 0
 *
 * or any equivalent physical instance as a universal language construct.
 *
 * Hardware realization belongs downstream.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Trust may span:
 *
 *     processes
 *     services
 *     actors
 *     nodes
 *     clusters
 *     regions
 *     distributed systems
 *
 * No finite participant count is encoded.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve enough structure for the domain-neutral AST to
 * represent at least:
 *
 *     TrustRelationship
 *     TrustRequirement
 *     TrustPreference
 *     TrustAssertion
 *     TrustReference
 *     TrustCondition
 *     TrustScope
 *     TrustEvidence
 *     TrustProperty
 *     TrustStatus
 *     TrustComposition
 *     TrustExtension
 *
 * Recommended source-preserving fields include:
 *
 *     source span
 *     declaration name
 *     source reference
 *     target reference
 *     requirement expression
 *     preference expression
 *     assertion target
 *     condition expression
 *     scope expression
 *     evidence expressions
 *     property name/value
 *     status expression
 *     extension reference
 *     composition reference
 *     attributes
 *     visibility
 *
 * The grammar creates no AST directly.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     name resolution
 *     identity resolution
 *     principal resolution
 *     authority resolution
 *     trust relationship validation
 *     trust requirement validation
 *     preference interpretation
 *     assertion validation
 *     evidence validation
 *     condition validation
 *     scope validation
 *     property validation
 *     status interpretation
 *     extension validation
 *     composition validation
 *     policy interaction
 *     capability checking
 *     effect interaction
 *     provenance validation
 *     resource interaction
 *     cryptographic requirement checking
 *     privacy interaction
 *     target feasibility
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Trust is NOT an independent execution IR.
 *
 * Validated trust information may become:
 *
 *     security metadata
 *     semantic constraints
 *     capability requirements
 *     policy metadata
 *     provenance metadata
 *     deployment requirements
 *     verification metadata
 *
 * It may accompany:
 *
 *     classical IR
 *     quantum::ir
 *     HDL/hardware semantic representations
 *     distributed representations
 *     deployment representations
 *
 * No trust-specific execution IR is introduced here.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Given the same:
 *
 *     token stream
 *     grammar version
 *     parser configuration
 *
 * parsing is deterministic.
 *
 * This grammar contains no:
 *
 *     randomness
 *     time-dependent behavior
 *     filesystem access
 *     network access
 *     environment inspection
 *     hardware inspection
 *     runtime calls
 *     external trust-store lookup
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * The parser must preserve source locations for:
 *
 *     TRUST
 *     declaration name
 *     FROM
 *     source reference
 *     THIN_ARROW
 *     target reference
 *     REQUIRES
 *     PREFER
 *     ASSERT
 *     FOR
 *     WHEN
 *     IN
 *     WITH
 *     AS
 *     EXTENDS
 *     USE
 *     property names
 *     expressions
 *     evidence references
 *
 * Diagnostic classification and semantic error codes belong downstream.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Trust syntax must remain compatible with the repository's versioning and
 * compatibility architecture.
 *
 * This grammar must not introduce duplicate lexical aliases for historical
 * spellings.
 *
 * Historical migration belongs under:
 *
 *     grammar/compatibility/
 *
 * The semantic compatibility layer may distinguish:
 *
 *     stable
 *     experimental
 *     deprecated
 *     historical
 *
 * without changing the trust grammar's open-world model.
 *
 * ============================================================================
 * PUBLIC RULE CONTRACT
 * ============================================================================
 *
 * Public reusable rules:
 *
 *     trustFile
 *     trustDeclaration
 *     trustRelationshipDeclaration
 *     trustRequirementDeclaration
 *     trustPreferenceDeclaration
 *     trustAssertionDeclaration
 *     trustReference
 *     trustReferenceList
 *     trustConditionReference
 *     trustRequirementReference
 *
 * Supporting rules are reusable where consumed by Security or security
 * subgrammars.
 *
 * ============================================================================
 */

parser grammar Trust;

options {
    tokenVocab = ZamaniLexer;
}

import Core, Types, Expressions;


/* ============================================================================
 * STANDALONE ENTRY
 * ============================================================================
 *
 * Used for:
 *
 *     isolated grammar tests
 *     tooling
 *     conformance tests
 *     parser fixtures
 *
 * The production security composition root consumes trustDeclaration directly.
 */

trustFile
    : trustDeclaration* EOF
    ;


/* ============================================================================
 * TRUST DECLARATION DISPATCH
 * ============================================================================
 *
 * Exactly one trust construct is selected at the declaration boundary.
 */

trustDeclaration
    : trustRelationshipDeclaration
    | trustRequirementDeclaration
    | trustPreferenceDeclaration
    | trustAssertionDeclaration
    ;


/* ============================================================================
 * TRUST RELATIONSHIP
 * ============================================================================
 *
 * Canonical form:
 *
 *     trust execution
 *         from security::authority
 *         -> compute::service;
 *
 * A declaration name is optional.
 *
 * The source and target are semantic references rather than physical
 * deployment identifiers.
 */

trustRelationshipDeclaration
    : attributes*
      visibility?
      TRUST
      identifier?
      trustSourceClause
      trustTargetClause
      trustRelationshipBody?
      SEMI?
    ;


trustSourceClause
    : FROM
      trustReference
    ;


trustTargetClause
    : THIN_ARROW
      trustReference
    ;


/* ============================================================================
 * TRUST RELATIONSHIP BODY
 * ============================================================================
 *
 * Body members remain independently owned by the trust grammar.
 *
 * No member performs semantic evaluation.
 */

trustRelationshipBody
    : LBRACE
      trustMember*
      RBRACE
    ;


trustMember
    : trustConditionClause
    | trustScopeClause
    | trustEvidenceClause
    | trustRequirementMember
    | trustPreferenceMember
    | trustAssertionMember
    | trustStatusClause
    | trustCompositionClause
    | trustExtensionClause
    | trustPropertyClause
    ;


/* ============================================================================
 * TRUST REQUIREMENT DECLARATION
 * ============================================================================
 *
 * Examples:
 *
 *     trust requires security::trusted_execution;
 *
 *     trust requires capability("security.attestation");
 *
 *     trust requires security::integrity and security::confidentiality;
 *
 * The expression grammar owns Boolean/operator semantics.
 */

trustRequirementDeclaration
    : attributes*
      visibility?
      TRUST
      REQUIRES
      identifier?
      expression
      SEMI?
    ;


trustRequirementMember
    : REQUIRES
      expression
      SEMI?
    ;


/* ============================================================================
 * TRUST PREFERENCE DECLARATION
 * ============================================================================
 *
 * Preference is weaker than requirement.
 *
 * Semantic analysis decides how it participates in policy, realization,
 * deployment, or optimization.
 */

trustPreferenceDeclaration
    : attributes*
      visibility?
      TRUST
      PREFER
      identifier?
      expression
      SEMI?
    ;


trustPreferenceMember
    : PREFER
      expression
      SEMI?
    ;


/* ============================================================================
 * TRUST ASSERTION DECLARATION
 * ============================================================================
 *
 * Assertion records source-level trust intent.
 *
 * It does not prove the assertion.
 */

trustAssertionDeclaration
    : attributes*
      visibility?
      TRUST
      ASSERT
      identifier?
      trustAssertionTarget
      trustAssertionBody?
      SEMI?
    ;


trustAssertionTarget
    : FOR
      trustReference
    ;


trustAssertionBody
    : LBRACE
      trustAssertionMember*
      RBRACE
    ;


trustAssertionMember
    : trustConditionClause
    | trustScopeClause
    | trustEvidenceClause
    | trustStatusClause
    | trustExtensionClause
    | trustPropertyClause
    ;


/* ============================================================================
 * TRUST REFERENCES
 * ============================================================================
 *
 * Trust references intentionally reuse canonical qualified names.
 *
 * Examples:
 *
 *     identity::alice
 *     principal::service
 *     authority::organization
 *     security::authority
 *     quantum::service
 *     hardware::trusted_execution
 *     future::security::mechanism
 *
 * Semantic analysis resolves their meaning.
 */

trustReference
    : qualifiedName
    ;


trustReferenceList
    : trustReference
      (
          COMMA
          trustReference
      )*
      COMMA?
    ;


/*
 * Reusable semantic reference boundaries.
 *
 * These rules deliberately remain syntactically equivalent to trustReference.
 * Their distinct names give downstream security grammars an explicit semantic
 * dependency without creating duplicate syntax.
 */

trustConditionReference
    : trustReference
    ;


trustRequirementReference
    : trustReference
    ;


/* ============================================================================
 * TRUST CONDITIONS
 * ============================================================================
 *
 * WHEN consumes the canonical Zamani expression system.
 *
 * No trust-specific Boolean language is created.
 */

trustConditionClause
    : WHEN
      expression
      SEMI?
    ;


/* ============================================================================
 * TRUST SCOPE
 * ============================================================================
 *
 * Scope remains an expression.
 *
 * It may represent:
 *
 *     a namespace
 *     a workload
 *     an execution context
 *     a security domain
 *     a deployment domain
 *     a temporal condition
 *     a resource domain
 *     a semantic scope
 *
 * The grammar does not enumerate scope kinds.
 */

trustScopeClause
    : IN
      expression
      SEMI?
    ;


/* ============================================================================
 * TRUST EVIDENCE
 * ============================================================================
 *
 * WITH is reserved here for evidence.
 *
 * Evidence is intentionally open-world.
 *
 * Examples:
 *
 *     with security::attestation;
 *     with security::measurement;
 *     with provenance::verified_source;
 *     with expression;
 *
 * The grammar does not enumerate certificate, attestation, measurement,
 * signature, log, credential, or any other finite evidence universe.
 */

trustEvidenceClause
    : WITH
      trustEvidenceList
      SEMI?
    ;


trustEvidenceList
    : trustEvidence
      (
          COMMA
          trustEvidence
      )*
      COMMA?
    ;


trustEvidence
    : trustReference
    | expression
    ;


/* ============================================================================
 * TRUST PROPERTIES
 * ============================================================================
 *
 * Properties use the existing USE keyword as a generic trust-property
 * namespace.
 *
 * Two forms are supported:
 *
 *     use security::integrity = required;
 *
 *     use security::classification;
 *
 * The second form is a symbolic extension/reference.
 *
 * This deliberately avoids overloading WITH, which is reserved for evidence.
 */

trustPropertyClause
    : USE
      trustPropertyList
      SEMI?
    ;


trustPropertyList
    : trustProperty
      (
          COMMA
          trustProperty
      )*
      COMMA?
    ;


trustProperty
    : qualifiedName
      (
          ASSIGN
          expression
      )?
    ;


/* ============================================================================
 * TRUST STATUS
 * ============================================================================
 *
 * AS introduces source-level status metadata.
 *
 * Status remains an expression rather than a closed enumeration.
 *
 * Therefore the language does not need a finite list such as:
 *
 *     trusted
 *     untrusted
 *     verified
 *     revoked
 *     pending
 *     expired
 *
 * New trust lifecycle models remain possible without grammar changes.
 *
 * Examples:
 *
 *     as security::verified;
 *     as security::pending;
 *     as future::trust_state;
 */

trustStatusClause
    : AS
      expression
      SEMI?
    ;


/* ============================================================================
 * TRUST COMPOSITION
 * ============================================================================
 *
 * EXTENDS expresses semantic composition/inheritance.
 *
 * Example:
 *
 *     extends security::base_trust;
 *
 * Multiple references are permitted without imposing a fixed number.
 *
 * Semantic analysis determines:
 *
 *     inheritance validity
 *     conflict resolution
 *     authority compatibility
 *     policy interaction
 *     provenance
 *     attenuation/strengthening semantics
 */

trustCompositionClause
    : EXTENDS
      trustReferenceList
      SEMI?
    ;


/* ============================================================================
 * TRUST EXTENSION
 * ============================================================================
 *
 * USE is also the generic trust extension namespace.
 *
 * The distinction from trustPropertyClause is structural:
 *
 *     use name;
 *
 *     use name = expression;
 *
 * are represented by the property/reference form.
 *
 * This rule provides a stable semantic extension boundary for downstream
 * security tooling without requiring new keywords for every future trust
 * mechanism.
 *
 * A USE entry with an assignment is interpreted as a trust property.
 *
 * A USE entry without an assignment may be interpreted as an extension or
 * symbolic trust feature.
 */

trustExtensionClause
    : USE
      trustExtensionReferenceList
      SEMI?
    ;


trustExtensionReferenceList
    : trustExtensionReference
      (
          COMMA
          trustExtensionReference
      )*
      COMMA?
    ;


trustExtensionReference
    : qualifiedName
    ;


/* ============================================================================
 * TRUST ASSERTION MEMBERS
 * ============================================================================
 *
 * Assertions inside a relationship or assertion body preserve trust claims
 * without performing verification.
 */

trustAssertionMember
    : ASSERT
      trustAssertionMemberTarget
      SEMI?
    ;


trustAssertionMemberTarget
    : trustReference
    ;


/* ============================================================================
 * SOURCE-LEVEL COMPOSITION HELPERS
 * ============================================================================
 *
 * These rules exist to give semantic consumers explicit boundaries while
 * preserving the single trust grammar authority.
 */

trustRelationshipReference
    : trustReference
    ;


trustEvidenceReference
    : trustReference
    ;


trustPropertyReference
    : qualifiedName
    ;


trustStatusReference
    : trustReference
    ;


/* ============================================================================
 * FILE COMPLETION CONTRACT
 * ============================================================================
 *
 * OWNED:
 *
 *     trust syntax only.
 *
 * DEPENDS_ON:
 *
 *     ZamaniLexer
 *     Core
 *     Types
 *     Expressions
 *
 * CONSUMED_BY:
 *
 *     grammar/security/security.g4
 *     security-specific semantic analysis
 *     trust conformance tests
 *
 * AST OWNER:
 *
 *     domain-neutral frontend AST
 *
 * SEMANTIC OWNER:
 *
 *     security/trust semantic analysis
 *
 * IR OWNER:
 *
 *     canonical semantic/IR layers
 *
 *     classical semantic representation
 *     quantum::ir
 *     HDL/hardware representation
 *     distributed representation
 *
 * TEST OWNER:
 *
 *     grammar/tests/security/
 *     grammar/tests/parser/
 *     grammar/tests/semantic/
 *
 * SPEC OWNER:
 *
 *     grammar/spec/security.md
 *     relevant security/trust specification material
 *
 * ============================================================================
 * REQUIRED INTEGRATION TESTS
 * ============================================================================
 *
 * POSITIVE:
 *
 *     trust execution
 *         from security::authority
 *         -> compute::service;
 *
 *     trust requires security::trusted_execution;
 *
 *     trust prefers security::attestation;
 *
 *     trust assert for security::service;
 *
 *     trust execution
 *         from identity::alice
 *         -> service::compute
 *         {
 *             requires security::attestation;
 *             prefer security::isolated_execution;
 *             when security::production_policy;
 *             in deployment::production;
 *             with security::attestation;
 *             as security::verified;
 *             extends security::base_trust;
 *             use security::audit;
 *             use security::classification = security::restricted;
 *         };
 *
 * OPEN-WORLD:
 *
 *     trust execution
 *         from future::authority::alpha
 *         -> future::service::beta
 *         {
 *             with future::evidence::mechanism;
 *             use future::trust::extension;
 *             use future::property = future::value;
 *             as future::trust::state;
 *         };
 *
 * QUANTUM:
 *
 *     trust quantum_execution
 *         from security::authority
 *         -> quantum::service
 *         {
 *             requires quantum::measurement;
 *             with hardware::attestation;
 *         };
 *
 * DISTRIBUTED:
 *
 *     trust distributed_execution
 *         from security::authority
 *         -> distributed::service
 *         {
 *             requires security::distributed_execution;
 *             in distributed::domain;
 *         };
 *
 * HDL/HARDWARE:
 *
 *     trust hardware_execution
 *         from security::authority
 *         -> hardware::accelerator
 *         {
 *             requires hardware::attestation;
 *         };
 *
 * NEGATIVE:
 *
 *     trust;
 *     trust execution from;
 *     trust execution ->;
 *     trust requires;
 *     trust prefer;
 *     trust assert;
 *     trust execution from security::a ->;
 *
 * SECURITY NEGATIVE:
 *
 *     trust execution
 *         from security::authority
 *         -> compute::service
 *         {
 *             with "private-key-material";
 *         };
 *
 * The semantic/security layer must reject secret-material misuse where the
 * relevant policy/specification forbids it. The grammar itself only parses
 * syntax.
 *
 * SCALABILITY:
 *
 *     - arbitrarily many trust declarations subject to implementation
 *       resources;
 *     - arbitrarily many relationship members;
 *     - arbitrarily many evidence references;
 *     - arbitrarily many requirements;
 *     - arbitrarily many preferences;
 *     - arbitrarily many composition references;
 *     - arbitrarily deep qualified names subject only to implementation
 *       resources;
 *     - arbitrarily many semantic trust domains;
 *     - no fixed hardware capacity;
 *     - no fixed participant count;
 *     - no fixed trust-provider count;
 *     - no fixed evidence count.
 *
 * DETERMINISM:
 *
 *     identical token streams + identical grammar/parser configuration
 *     produce identical parsing behavior.
 *
 * SAFETY:
 *
 *     generated Rust integration MUST remain safe Rust.
 *
 *     #![deny(unsafe_code)]
 *
 * should remain enforced by the Rust crate/workspace containing the generated
 * frontend implementation.
 *
 * ============================================================================
 * NON-GOALS
 * ============================================================================
 *
 * This file does NOT:
 *
 *     authenticate;
 *     authorize;
 *     grant trust;
 *     revoke trust;
 *     validate certificates;
 *     verify signatures;
 *     evaluate policies;
 *     inspect hardware;
 *     inspect network state;
 *     discover resources;
 *     choose a target;
 *     allocate resources;
 *     execute code;
 *     perform cryptography;
 *     access secrets;
 *     create a quantum IR;
 *     route quantum operations;
 *     schedule hardware;
 *     perform QEC;
 *     emit ZQN;
 *     select a HAL implementation.
 *
 * ============================================================================
 * FINAL RULE
 * ============================================================================
 *
 * Trust is a DECLARATIVE SECURITY-INTENT LANGUAGE FEATURE.
 *
 * The source program remains target-independent.
 *
 * Trust requirements constrain realization.
 *
 * Trust preferences guide realization.
 *
 * Trust evidence supplies semantic evidence references.
 *
 * Trust conditions constrain applicability.
 *
 * Trust properties carry extensible metadata.
 *
 * Trust composition and extensions provide open-world evolution.
 *
 * Security semantics, policy, provenance, capabilities, resources, effects,
 * classical computation, quantum computation, HDL, distributed execution,
 * compilation, lowering, routing, scheduling, resilience, ZQN, HAL, and
 * runtime enforcement remain separate architectural layers.
 *
 * This separation is what permits the same Zamani source to remain portable
 * from very small systems to arbitrarily larger systems, limited only by the
 * resources and capabilities actually available to the realization.
 *
 * ============================================================================
 */