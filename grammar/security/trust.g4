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
 *     Canonical parser-level trust grammar.
 *
 * Purpose:
 *     Define portable, open-world source syntax for trust relationships,
 *     trust requirements, trust preferences, trust assertions, trust scope,
 *     trust conditions, trust evidence references, trust composition,
 *     trust properties, trust extension, and trust status metadata.
 *
 * Language baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     - This file contains no Rust code.
 *     - This file contains no embedded actions.
 *     - This file contains no semantic predicates.
 *     - This file performs no I/O.
 *     - This file performs no network access.
 *     - This file performs no credential access.
 *     - This file performs no cryptographic operations.
 *     - This file performs no trust evaluation.
 *     - This file performs no hardware discovery.
 *     - No unsafe Rust is required or permitted by the implementation
 *       contract.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         ZAMANI SOURCE
 *                              |
 *                              v
 *                        ZamaniLexer
 *                              |
 *                              v
 *                        ZamaniParser
 *                              |
 *                    Security composition
 *                              |
 *                              v
 *                       Trust grammar
 *                              |
 *                              v
 *                       Frontend AST
 *                              |
 *              +---------------+----------------+
 *              |               |                |
 *              v               v                v
 *        name resolution   security analysis   policy analysis
 *                              |
 *                              v
 *                       semantic trust model
 *                              |
 *              +---------------+----------------+
 *              |               |                |
 *              v               v                v
 *          classical       quantum::ir       HDL/hardware
 *              |               |                |
 *              +---------------+----------------+
 *                              |
 *                              v
 *                  compilation / lowering /
 *                  routing / scheduling /
 *                  resilience / deployment
 *                              |
 *                              v
 *                           runtime
 *
 * This file exists ABOVE semantic analysis and BELOW the canonical lexer.
 *
 * It describes source syntax only.
 *
 * ============================================================================
 * SINGLE AUTHORITY
 * ============================================================================
 *
 * Trust syntax is owned exclusively by this file.
 *
 * The security composition root:
 *
 *     grammar/security/security.g4
 *
 * imports this grammar and consumes its public rules.
 *
 * Other security grammars MAY reference trust through:
 *
 *     trustConditionReference
 *     trustRequirementReference
 *     trustReference
 *
 * but MUST NOT reproduce trust declaration syntax.
 *
 * ============================================================================
 * NON-OWNERSHIP
 * ============================================================================
 *
 * This grammar DOES NOT own:
 *
 *     identity declaration
 *     principal declaration
 *     authentication
 *     authorization
 *     permissions
 *     capability declaration
 *     cryptographic algorithms
 *     keys
 *     certificates
 *     credential storage
 *     certificate validation
 *     signature verification
 *     privacy enforcement
 *     policy evaluation
 *     trust-store implementation
 *     hardware trust anchors
 *     TPM/HSM/enclave implementation
 *     network transport
 *     network discovery
 *     hardware discovery
 *     target selection
 *     resource allocation
 *     quantum semantics
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *     optimization
 *     backend selection
 *     runtime execution
 *
 * ============================================================================
 * EXISTING REPOSITORY INTEGRATION
 * ============================================================================
 *
 * Canonical parser composition:
 *
 *     grammar/antlr/ZamaniParser.g4
 *             |
 *             +--> Security
 *                      |
 *                      +--> Trust
 *
 * Security-wide composition:
 *
 *     grammar/security/security.g4
 *
 * Existing specialized security owners:
 *
 *     grammar/security/identifiers.g4
 *         identity / principal syntax
 *
 *     grammar/security/permissions.g4
 *         authorization / permission syntax
 *
 *     grammar/security/capabilities.g4
 *         security capability syntax
 *
 *     grammar/security/cryptography.g4
 *         cryptographic intent
 *
 *     grammar/security/privacy.g4
 *         privacy syntax
 *
 * Trust MUST NOT redefine any of these domains.
 *
 * ============================================================================
 * LEXER INTEGRATION
 * ============================================================================
 *
 * The canonical production lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Parser grammars therefore consume:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * Do NOT change this file to:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * ZamaniTokens is the lexical composition vocabulary.
 *
 * ZamaniLexer is the production lexer consumed by parser grammars.
 *
 * ============================================================================
 * EXISTING TOKEN INTEGRATION
 * ============================================================================
 *
 * This grammar intentionally uses only vocabulary already present in the
 * repository's canonical lexical architecture.
 *
 * Existing keywords/operators used here include:
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
 *
 * No new trust-specific lexer token is required for:
 *
 *     to
 *     evidence
 *     valid
 *     evaluate
 *     using
 *     requirement
 *     preference
 *     assertion
 *
 * Those concepts are represented structurally using existing vocabulary and
 * generic names/expressions.
 *
 * This is intentional.
 *
 * ============================================================================
 * OPEN-WORLD TRUST MODEL
 * ============================================================================
 *
 * Trust is intentionally open-world.
 *
 * This grammar MUST NOT enumerate:
 *
 *     trust providers
 *     identity providers
 *     certificate authorities
 *     trust anchors
 *     algorithms
 *     cryptographic schemes
 *     vendors
 *     cloud providers
 *     machines
 *     devices
 *     enclaves
 *     principals
 *     services
 *     QPUs
 *     GPUs
 *     CPUs
 *     FPGAs
 *     future trust mechanisms
 *
 * All such entities are represented through:
 *
 *     qualifiedName
 *     expression
 *
 * and interpreted downstream.
 *
 * Therefore a new trust mechanism does not require a grammar modification.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Trust syntax describes PORTABLE TRUST INTENT.
 *
 * It MUST NOT encode today's physical deployment as permanent program
 * semantics.
 *
 * For example:
 *
 *     trust execution
 *         from security::authority
 *         -> compute::service;
 *
 * does NOT mean:
 *
 *     use machine X
 *     use CPU Y
 *     use GPU Z
 *     use QPU N
 *     use FPGA M
 *     use node K
 *     use enclave E
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar deliberately contains no finite language-level limits on:
 *
 *     trust declarations
 *     trust relationships
 *     trust conditions
 *     trust evidence
 *     trust properties
 *     trust references
 *     trust scopes
 *     trust requirements
 *     trust preferences
 *     trust metadata
 *     trust composition
 *     trust extensions
 *     declaration nesting
 *     qualified-name depth
 *     expression size
 *
 * There is NO:
 *
 *     MAX_TRUST_RELATIONSHIPS
 *     MAX_TRUST_ANCHORS
 *     MAX_TRUST_PROVIDERS
 *     MAX_PRINCIPALS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_QUANTUM_DEVICES
 *     MAX_EVIDENCE
 *
 * or equivalent language-level ceiling.
 *
 * Practical implementation limits remain compiler/resource-policy concerns.
 *
 * ============================================================================
 * SECURITY PRINCIPLE
 * ============================================================================
 *
 * Successfully parsing a trust declaration MUST NOT grant trust.
 *
 * Parsing establishes only that the source conforms to trust syntax.
 *
 * Semantic analysis must subsequently determine:
 *
 *     whether references resolve;
 *     whether identities exist;
 *     whether authorities are valid;
 *     whether evidence is acceptable;
 *     whether conditions are satisfiable;
 *     whether policy permits the relationship;
 *     whether capabilities exist;
 *     whether cryptographic requirements can be met;
 *     whether resource requirements can be met;
 *     whether the target can preserve mandatory security properties.
 *
 * Runtime infrastructure may subsequently perform:
 *
 *     authentication;
 *     attestation;
 *     certificate validation;
 *     signature verification;
 *     policy evaluation;
 *     capability validation;
 *     trust-anchor validation;
 *     secure-environment validation.
 *
 * None of those operations belong here.
 *
 * ============================================================================
 * SECRET MATERIAL
 * ============================================================================
 *
 * Trust syntax MUST NOT create a primitive for embedding:
 *
 *     passwords
 *     private keys
 *     secret keys
 *     bearer tokens
 *     API secrets
 *     session secrets
 *     authentication secrets
 *     recovery secrets
 *     raw credential material
 *
 * A trust declaration may reference an abstract credential/evidence object:
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
 * Identity declarations are owned by:
 *
 *     grammar/security/identifiers.g4
 *
 * Trust may refer to:
 *
 *     identity::alice
 *     principal::service
 *     authority::organization
 *
 * without redefining identity syntax.
 *
 * Example:
 *
 *     trust application
 *         from identity::alice
 *         -> service::compute;
 *
 * The parser does not establish that the identity is authentic.
 *
 * ============================================================================
 * AUTHORIZATION INTEGRATION
 * ============================================================================
 *
 * Authorization and permissions are owned by:
 *
 *     grammar/security/permissions.g4
 *
 * Trust may be consumed by authorization analysis.
 *
 * Trust does NOT itself define:
 *
 *     allow
 *     deny
 *     grant
 *     revoke
 *     permission
 *
 * A permission system may reference trust through:
 *
 *     trustRequirementReference
 *
 * without importing a second trust language.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Capability declarations remain owned by the existing capability grammar.
 *
 * Trust may reference capabilities:
 *
 *     security::trusted_execution
 *     quantum::secure_execution
 *     hardware::attestation
 *
 * The grammar does not determine whether the capability exists.
 *
 * ============================================================================
 * CRYPTOGRAPHY INTEGRATION
 * ============================================================================
 *
 * Cryptographic intent remains owned by:
 *
 *     grammar/security/cryptography.g4
 *
 * Trust can reference cryptographic evidence or properties symbolically.
 *
 * Example:
 *
 *     with security::attestation;
 *
 * or:
 *
 *     with cryptography::verified_measurement;
 *
 * No algorithm list is embedded here.
 *
 * ============================================================================
 * PRIVACY INTEGRATION
 * ============================================================================
 *
 * Privacy syntax remains owned by:
 *
 *     grammar/security/privacy.g4
 *
 * Trust may coexist with privacy requirements without defining privacy
 * semantics.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Trust syntax is target-independent and may protect:
 *
 *     quantum computation
 *     quantum-classical execution
 *     quantum services
 *     quantum data
 *     quantum control
 *     quantum hardware
 *
 * Examples:
 *
 *     trust quantum_execution
 *         from security::authority
 *         -> quantum::service
 *         with quantum::trusted_execution;
 *
 *     trust quantum_measurement
 *         from quantum::service
 *         -> classical::controller
 *         when security::measurement_policy;
 *
 * This grammar MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     topology
 *     coupling map
 *     calibration
 *     pulse data
 *     QEC code
 *     physical backend
 *
 * Quantum semantics remain downstream:
 *
 *     source
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic quantum model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       +--> optimization
 *       +--> QEC
 *       +--> ZQN
 *       +--> routing
 *       +--> scheduling
 *       +--> HAL
 *
 * No second quantum IR is introduced.
 *
 * ============================================================================
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * Trust may refer to abstract hardware security capabilities:
 *
 *     hardware::trusted_execution
 *     hardware::attestation
 *     hardware::isolated_memory
 *
 * It MUST NOT select:
 *
 *     CPU 0
 *     GPU 0
 *     QPU 0
 *     FPGA 0
 *     node 0
 *
 * or encode any universal hardware capacity.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Trust can span:
 *
 *     services
 *     processes
 *     nodes
 *     clusters
 *     regions
 *     distributed systems
 *
 * No finite number of participants is encoded in the grammar.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser should preserve the syntactic structure necessary for the
 * frontend AST to represent:
 *
 *     trust relationship
 *     trust requirement
 *     trust preference
 *     trust assertion
 *     source reference
 *     target reference
 *     condition
 *     scope
 *     evidence
 *     property
 *     extension
 *     status
 *     composition
 *     metadata
 *
 * Recommended semantic concepts are:
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
 *     TrustExtension
 *     TrustStatus
 *
 * These are semantic/AST concepts, not parser actions.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     name resolution
 *     identity resolution
 *     principal resolution
 *     authority resolution
 *     trust relationship validation
 *     requirement validation
 *     preference interpretation
 *     evidence validation
 *     condition validation
 *     policy interaction
 *     capability checking
 *     cryptographic requirement checking
 *     privacy interaction
 *     resource/capability analysis
 *     target feasibility
 *
 * The parser MUST NOT perform these operations.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Trust is NOT an independent universal execution IR.
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
 *     HDL/hardware representations
 *     distributed representations
 *     deployment metadata
 *
 * The grammar does not create any of these representations.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Given an identical token stream and grammar version, parsing is deterministic.
 *
 * This grammar contains:
 *
 *     no actions
 *     no predicates
 *     no randomness
 *     no runtime calls
 *     no external state
 *     no filesystem access
 *     no network access
 *     no hardware discovery
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * The grammar must allow malformed trust syntax to remain distinguishable
 * from valid trust syntax.
 *
 * The frontend must preserve source spans for:
 *
 *     trust keyword
 *     optional declaration name
 *     source reference
 *     target reference
 *     requirement expression
 *     preference expression
 *     assertion target
 *     condition
 *     evidence
 *     properties
 *     metadata
 *
 * Diagnostic classification belongs downstream.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust implementation.
 *
 * The generated Zamani parser/frontend integration MUST:
 *
 *     - compile with Rust 1.97;
 *     - compile with Rust 1.97.1;
 *     - use Rust 2021;
 *     - contain no unsafe Rust;
 *     - preserve source spans;
 *     - remain deterministic;
 *     - avoid machine-specific assumptions.
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 *
 * The following rules are intended for Security and other parser consumers:
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
 * Supporting rules are intentionally reusable where useful.
 *
 * ============================================================================
 */

parser grammar Trust;

options {
    tokenVocab = ZamaniLexer;
}

import Core, Types, Expressions;


/* ============================================================================
 * 1. STANDALONE TRUST ENTRY
 * ============================================================================
 *
 * This rule is useful for grammar tests and isolated tooling.
 *
 * The Security composition root does NOT call this rule as its complete
 * program entry point; it consumes trustDeclaration directly.
 * ========================================================================== */

trustFile
    : trustDeclaration* EOF
    ;


/* ============================================================================
 * 2. UNIVERSAL TRUST DECLARATION
 * ========================================================================== */

trustDeclaration
    : trustRelationshipDeclaration
    | trustRequirementDeclaration
    | trustPreferenceDeclaration
    | trustAssertionDeclaration
    ;


/* ============================================================================
 * 3. TRUST RELATIONSHIP
 * ============================================================================
 *
 * Canonical portable form:
 *
 *     trust execution
 *         from security::authority
 *         -> compute::service;
 *
 * The arrow is the existing canonical THIN_ARROW operator.
 *
 * A named relationship is optional because anonymous trust relationships are
 * useful for local/source-level requirements.
 *
 * ========================================================================== */

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
 * 4. TRUST RELATIONSHIP BODY
 * ========================================================================== */

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
    | trustPropertyClause
    | trustExtensionClause
    | trustStatusClause
    | trustCompositionClause
    ;


/* ============================================================================
 * 5. TRUST REQUIREMENT
 * ============================================================================
 *
 * Reuses the existing canonical REQUIRES keyword and the canonical expression
 * grammar.
 *
 * There is deliberately no second Boolean/trust-specific requirement language.
 *
 * Therefore:
 *
 *     trust requires security::trusted_execution;
 *
 *     trust requires capability("security.attestation");
 *
 *     trust requires security::integrity and security::confidentiality;
 *
 * can all be represented by the same semantic expression infrastructure.
 *
 * ========================================================================== */

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
 * 6. TRUST PREFERENCE
 * ============================================================================
 *
 * Preference is intentionally weaker than requirement.
 *
 * Semantic analysis decides how a preference participates in target selection,
 * policy evaluation, or optimization.
 *
 * ========================================================================== */

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
 * 7. TRUST ASSERTION
 * ============================================================================
 *
 * ASSERT is already a canonical Zamani keyword.
 *
 * This syntax records source-level assertion intent.
 *
 * It does NOT verify the assertion.
 *
 * ========================================================================== */

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
    | trustPropertyClause
    | trustStatusClause
    | trustExtensionClause
    ;


/* ============================================================================
 * 8. TRUST REFERENCE
 * ============================================================================
 *
 * Trust references intentionally reuse the canonical qualified-name grammar.
 *
 * Examples:
 *
 *     identity::alice
 *     principal::service
 *     security::authority
 *     quantum::service
 *     hardware::trusted_execution
 *     future::trust::mechanism
 *
 * Their meaning is semantic, not syntactic.
 * ========================================================================== */

trustReference
    : qualifiedName
    ;


trustReferenceList
    : trustReference
      (COMMA trustReference)*
      COMMA?
    ;


/* ============================================================================
 * 9. TRUST CONDITIONS
 * ============================================================================
 *
 * WHEN is already a canonical Zamani keyword.
 *
 * The expression remains general and open-world.
 * ========================================================================== */

trustConditionClause
    : WHEN
      expression
      SEMI?
    ;


/* ============================================================================
 * 10. TRUST SCOPE
 * ============================================================================
 *
 * IN is an existing language keyword.
 *
 * Scope is an expression rather than a fixed enumeration.
 *
 * Examples:
 *
 *     in execution;
 *     in security::production;
 *     in deployment::region;
 *     in quantum::execution;
 *
 * ========================================================================== */

trustScopeClause
    : IN
      expression
      SEMI?
    ;


/* ============================================================================
 * 11. TRUST EVIDENCE
 * ============================================================================
 *
 * WITH is an existing keyword.
 *
 * Evidence is represented as one or more references/expressions.
 *
 * The grammar does not define:
 *
 *     certificate
 *     attestation
 *     signature
 *     measurement
 *     log
 *     credential
 *
 * as a closed list.
 *
 * ========================================================================== */

trustEvidenceClause
    : WITH
      trustEvidenceList
      SEMI?
    ;


trustEvidenceList
    : trustEvidence
      (COMMA trustEvidence)*
      COMMA?
    ;


trustEvidence
    : trustReference
    | expression
    ;


/* ============================================================================
 * 12. TRUST PROPERTIES
 * ============================================================================
 *
 * Generic properties remain open-world.
 *
 * This allows future trust models without requiring new parser keywords.
 *
 * Examples:
 *
 *     security::integrity = required;
 *     organization::jurisdiction = "example";
 *     quantum::measurement = security::protected;
 *
 * Semantic analysis decides whether a property is valid.
 * ========================================================================== */

trustPropertyClause
    : WITH
      trustPropertyList
    ;


trustPropertyList
    : trustProperty
      (COMMA trustProperty)*
      COMMA?
    ;


trustProperty
    : qualifiedName
      ASSIGN
      expression
    ;


/* ============================================================================
 * 13. TRUST STATUS
 * ============================================================================
 *
 * AS is already a canonical language keyword.
 *
 * Status is deliberately an expression rather than an enumeration.
 *
 * This avoids hard-coding a finite trust lifecycle.
 *
 * ==========================================================================