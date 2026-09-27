/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/security/provenance.g4
 *
 * Status:
 *     PRODUCTION SECURITY-DOMAIN LEAF GRAMMAR
 *
 * Grammar:
 *     SecurityProvenance
 *
 * Purpose:
 *     Define SOURCE-LEVEL SECURITY PROVENANCE syntax.
 *
 * IMPORTANT:
 *
 *     grammar/data/provenance.g4
 *
 * remains the canonical owner of GENERAL DATA PROVENANCE / LINEAGE.
 *
 * This file does NOT duplicate general provenance declarations.
 *
 * This file owns the SECURITY interpretation of provenance:
 *
 *     - provenance evidence requirements;
 *     - provenance integrity requirements;
 *     - provenance authenticity requirements;
 *     - provenance attestation requirements;
 *     - provenance verification requirements;
 *     - provenance audit requirements;
 *     - provenance disclosure restrictions;
 *     - provenance retention/security requirements;
 *     - provenance trust requirements;
 *     - provenance security classifications;
 *     - provenance security properties;
 *     - provenance claims;
 *     - provenance evidence references;
 *     - provenance verification policies;
 *     - provenance security metadata;
 *
 * It expresses SECURITY INTENT only.
 *
 * It does NOT:
 *
 *     - authenticate;
 *     - authorize;
 *     - verify signatures;
 *     - calculate hashes;
 *     - access credentials;
 *     - access secret material;
 *     - contact an identity provider;
 *     - contact a trust service;
 *     - inspect hardware;
 *     - discover devices;
 *     - select a backend;
 *     - execute audit operations;
 *     - store audit records;
 *     - implement a provenance database;
 *     - implement a ledger;
 *     - implement blockchain;
 *     - implement cryptography;
 *     - implement attestation;
 *     - implement secure boot;
 *     - implement TPM/HSM/TEE functionality;
 *     - define quantum::ir;
 *     - define QEC;
 *     - define ZQN;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform optimization;
 *     - perform resilience;
 *     - perform runtime enforcement.
 *
 * ============================================================================
 * ARCHITECTURAL AUTHORITY
 * ============================================================================
 *
 * General provenance:
 *
 *     grammar/data/provenance.g4
 *
 * Security composition:
 *
 *     grammar/security/security.g4
 *
 * Security semantics:
 *
 *     grammar/spec/security.md
 *
 * Generic names / expressions:
 *
 *     Core
 *     Types
 *     Expressions
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical language composition:
 *
 *     grammar/Zamani.g4
 *
 * Domain-neutral AST:
 *
 *     src/frontend/ast/
 *
 * Canonical quantum semantic boundary:
 *
 *     quantum::ir
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     security provenance requirements
 *     security provenance constraints
 *     security provenance preferences
 *     security provenance hints
 *     security provenance evidence references
 *     security provenance claims
 *     security provenance attestations
 *     security provenance verification intent
 *     security provenance audit intent
 *     security provenance integrity intent
 *     security provenance authenticity intent
 *     security provenance trust intent
 *     security provenance disclosure intent
 *     security provenance retention intent
 *     security provenance classification
 *     security provenance properties
 *
 * THIS FILE DOES NOT OWN:
 *
 *     general data lineage
 *     data derivation
 *     data transformation
 *     schemas
 *     persistence
 *     serialization
 *     identity definitions
 *     authorization
 *     cryptographic algorithms
 *     key management
 *     privacy implementation
 *     trust implementation
 *     audit storage
 *     runtime verification
 *     hardware realization
 *     quantum operations
 *     quantum IR
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * There must be exactly one owner for each concept.
 *
 * General lineage:
 *
 *     data/provenance.g4
 *
 * Security provenance:
 *
 *     security/provenance.g4
 *
 * Identity:
 *
 *     security/identity.g4
 *     security/identifiers.g4
 *
 * Authorization:
 *
 *     security/authorization.g4
 *     security/permissions.g4
 *
 * Cryptographic intent:
 *
 *     security/cryptography.g4
 *
 * Trust:
 *
 *     security/trust.g4
 *
 * Privacy:
 *
 *     security/privacy.g4
 *
 * Generic security constraints:
 *
 *     security/security-constraints.g4
 *
 * Security provenance MUST NOT recreate rules owned by those grammars.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Provenance security must remain portable.
 *
 * No source-level security provenance construct may require:
 *
 *     a specific CPU;
 *     a specific GPU;
 *     a specific FPGA;
 *     a specific ASIC;
 *     a specific QPU;
 *     a specific node;
 *     a specific device;
 *     a specific memory capacity;
 *     a specific storage capacity;
 *     a specific network;
 *     a specific provider;
 *     a specific operating system;
 *     a specific physical address;
 *     a fixed topology.
 *
 * The grammar MUST NOT define:
 *
 *     MAX_IDENTITIES
 *     MAX_PROVENANCE_RECORDS
 *     MAX_AUDIT_EVENTS
 *     MAX_CLAIMS
 *     MAX_EVIDENCE
 *     MAX_TRUST_ANCHORS
 *     MAX_SECURITY_DOMAINS
 *     MAX_KEYS
 *     MAX_CERTIFICATES
 *
 * or equivalent language-level limits.
 *
 * Repetition and recursive structures are intentionally unbounded by grammar.
 *
 * Practical resource limits remain implementation/resource-policy concerns.
 *
 * ============================================================================
 * SECURITY PRINCIPLE
 * ============================================================================
 *
 * Provenance is evidence about origin, transformation, execution, or
 * verification history.
 *
 * Provenance is NOT automatically trustworthy.
 *
 * Therefore the grammar distinguishes:
 *
 *     provenance
 *     provenance claim
 *     provenance evidence
 *     provenance attestation
 *     provenance verification requirement
 *
 * A claim is not proof merely because it appears in source code.
 *
 * The semantic/runtime security layers determine whether evidence is:
 *
 *     verified;
 *     trusted;
 *     authentic;
 *     complete;
 *     current;
 *     applicable;
 *     sufficient.
 *
 * ============================================================================
 * OPEN-WORLD MODEL
 * ============================================================================
 *
 * Security properties and evidence kinds remain open-world.
 *
 * Do not enumerate today's complete universe of:
 *
 *     hash algorithms;
 *     signature algorithms;
 *     attestation mechanisms;
 *     certificate systems;
 *     ledgers;
 *     identity providers;
 *     audit systems;
 *     cloud providers;
 *     hardware security technologies.
 *
 * Qualified names and expressions represent future mechanisms.
 *
 * Examples:
 *
 *     security::provenance
 *     security::provenance::integrity
 *     security::provenance::authenticity
 *     security::provenance::attestation
 *     security::provenance::verification
 *     future::security::provenance
 *     vendor::security::evidence
 *
 * The parser does not decide what those names mean.
 *
 * ============================================================================
 * SECRET-MATERIAL BOUNDARY
 * ============================================================================
 *
 * Source-level provenance MUST NOT contain secret material.
 *
 * This grammar therefore has no rules for:
 *
 *     passwords;
 *     private keys;
 *     secret keys;
 *     bearer tokens;
 *     API secrets;
 *     credentials;
 *     authentication secrets;
 *     recovery secrets.
 *
 * A provenance declaration may reference a symbolic security object:
 *
 *     credential::runtime_identity
 *     key::provenance_signing_key
 *     trust::execution_authority
 *
 * but the referenced secret material is resolved outside the grammar.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no Rust;
 *     - no filesystem operations;
 *     - no network operations;
 *     - no environment inspection;
 *     - no hardware discovery;
 *     - no cryptographic execution;
 *     - no randomness;
 *     - no policy evaluation.
 *
 * Parsing therefore depends only upon:
 *
 *     source token stream
 *     grammar version
 *     explicitly selected dialect configuration
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve:
 *
 *     - source span;
 *     - declaration kind;
 *     - qualified names;
 *     - expressions;
 *     - evidence references;
 *     - claim structure;
 *     - requirement structure;
 *     - constraint structure;
 *     - preference structure;
 *     - hint structure;
 *     - source ordering where semantically relevant.
 *
 * Suggested domain-neutral semantic categories:
 *
 *     SecurityProvenanceRequirement
 *     SecurityProvenanceConstraint
 *     SecurityProvenancePreference
 *     SecurityProvenanceHint
 *     SecurityProvenanceClaim
 *     SecurityProvenanceEvidence
 *     SecurityProvenanceAttestation
 *     SecurityProvenanceVerification
 *     SecurityProvenanceAudit
 *     SecurityProvenanceProperty
 *
 * These are semantic contracts, not parser implementation requirements.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - whether a provenance subject resolves;
 *     - whether an evidence reference is valid;
 *     - whether a claim is applicable;
 *     - whether an attestation is relevant;
 *     - whether evidence satisfies a requirement;
 *     - whether an integrity requirement is satisfiable;
 *     - whether authenticity is required;
 *     - whether trust requirements are satisfiable;
 *     - whether privacy restrictions conflict with provenance disclosure;
 *     - whether retention requirements are compatible;
 *     - whether provenance requirements survive lowering;
 *     - whether the target environment can satisfy mandatory requirements.
 *
 * The parser does not make those decisions.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO security-provenance IR.
 *
 * Security provenance lowers through the canonical semantic model.
 *
 * General data provenance remains associated with:
 *
 *     data provenance semantics
 *
 * Security provenance metadata may accompany:
 *
 *     classical semantic/IR representations;
 *     quantum::ir;
 *     HDL/hardware semantic representations;
 *     distributed representations;
 *     deployment metadata.
 *
 * The security provenance grammar MUST NOT create:
 *
 *     SecurityProvenanceIR
 *     QuantumSecurityIR
 *     ProvenanceIR
 *
 * as competing canonical IRs.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Security provenance may describe quantum execution evidence.
 *
 * Examples of semantic properties include:
 *
 *     security::provenance
 *     quantum::measurement_provenance
 *     quantum::execution_attestation
 *     quantum::result_integrity
 *
 * However this grammar MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     topology
 *     calibration
 *     QEC codes
 *     ZQN faults
 *     routing
 *     scheduling
 *
 * The canonical quantum boundary remains:
 *
 *     quantum::ir
 *
 * Security provenance is metadata/constraint information accompanying that
 * representation.
 *
 * ============================================================================
 * CLASSICAL / HDL / HYBRID / DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * The same security provenance model applies to:
 *
 *     classical;
 *     quantum;
 *     hybrid;
 *     HDL;
 *     hardware;
 *     accelerator;
 *     AI;
 *     distributed;
 *     networking;
 *     embedded;
 *     future computational domains.
 *
 * The grammar does not need separate provenance syntax for each domain.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * Mandatory requirement:
 *
 *     requires security::provenance::integrity;
 *
 * Constraint:
 *
 *     constraint security::provenance::disclosure == restricted;
 *
 * Preference:
 *
 *     prefer security::provenance::attestation;
 *
 * Hint:
 *
 *     hint security::provenance::retain_lineage;
 *
 * These categories MUST remain semantically distinct.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar SecurityProvenance;

options {
    tokenVocab = ZamaniLexer;
}

import
    Core,
    Types,
    Expressions;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This entry point is intended for:
 *
 *     - grammar conformance tests;
 *     - parser unit tests;
 *     - tooling;
 *     - standalone security-provenance validation.
 *
 * The normal program path enters through Security.securityDeclaration.
 * ============================================================================
 */

securityProvenanceFile
    : securityProvenanceDeclaration*
      EOF
    ;


/*
 * ============================================================================
 * TOP-LEVEL SECURITY PROVENANCE DECLARATION
 * ============================================================================
 *
 * The introducer is intentionally represented as a contextual identifier
 * rather than introducing a new lexer keyword.
 *
 * This avoids unnecessary lexical vocabulary expansion.
 *
 * The semantic layer MUST require the introducer spelling:
 *
 *     provenance
 *
 * Case-sensitive.
 *
 * This keeps "provenance" available as an ordinary identifier everywhere else
 * unless and until the language specification deliberately promotes it to a
 * reserved keyword.
 * ============================================================================
 */

securityProvenanceDeclaration
    : securityProvenanceIntroducer
      securityProvenanceTarget?
      securityProvenanceBody?
      SEMI?
    ;


/*
 * ============================================================================
 * CONTEXTUAL INTRODUCER
 * ============================================================================
 *
 * `identifier` is deliberately used here because the current canonical lexer
 * does not define a dedicated PROVENANCE token.
 *
 * Semantic validation MUST reject:
 *
 *     anything other than the exact identifier "provenance"
 *
 * at this position.
 *
 * This is a contextual-keyword strategy and avoids changing the lexer as part
 * of this independent leaf grammar.
 * ============================================================================
 */

securityProvenanceIntroducer
    : identifier
    ;


/*
 * ============================================================================
 * TARGET
 * ============================================================================
 *
 * Provenance security may apply to:
 *
 *     a value;
 *     a computation;
 *     an artifact;
 *     a declaration;
 *     a resource;
 *     a module;
 *     a quantum result;
 *     a hardware artifact;
 *     a distributed result;
 *     a future semantic object.
 *
 * The target remains a canonical expression.
 * ============================================================================
 */

securityProvenanceTarget
    : FOR
      expression
    ;


/*
 * ============================================================================
 * BODY
 * ============================================================================
 */

securityProvenanceBody
    : LBRACE
      securityProvenanceMember*
      RBRACE
    ;


/*
 * ============================================================================
 * MEMBERS
 * ============================================================================
 */

securityProvenanceMember
    : securityProvenanceClaim
    | securityProvenanceEvidence
    | securityProvenanceAttestation
    | securityProvenanceVerification
    | securityProvenanceAudit
    | securityProvenanceRequirement
    | securityProvenanceConstraint
    | securityProvenancePreference
    | securityProvenanceHint
    | securityProvenanceProperty
    ;


/*
 * ============================================================================
 * CLAIM
 * ============================================================================
 *
 * A claim is an assertion about provenance.
 *
 * A claim is NOT automatically verified.
 * ============================================================================
 */

securityProvenanceClaim
    : CLAIM
      securityProvenanceClaimBody
    ;


securityProvenanceClaimBody
    : expression
      SEMI
    | LBRACE
      securityProvenanceProperty*
      RBRACE
    ;


/*
 * ============================================================================
 * EVIDENCE
 * ============================================================================
 *
 * Evidence identifies or describes evidence relevant to a provenance claim.
 *
 * The actual evidence remains outside the parser.
 * ============================================================================
 */

securityProvenanceEvidence
    : EVIDENCE
      expression
      SEMI?
    ;


/*
 * ============================================================================
 * ATTESTATION
 * ============================================================================
 *
 * Attestation expresses a requirement or declaration concerning externally
 * supplied provenance evidence.
 *
 * The parser does not verify the attestation.
 * ============================================================================
 */

securityProvenanceAttestation
    : ATTESTATION
      expression
      SEMI?
    ;


/*
 * ============================================================================
 * VERIFICATION
 * ============================================================================
 *
 * Verification expresses desired verification semantics.
 *
 * It does not perform verification.
 * ============================================================================
 */

securityProvenanceVerification
    : VERIFICATION
      securityProvenanceVerificationExpression
      SEMI?
    ;


securityProvenanceVerificationExpression
    : expression
    ;


/*
 * ============================================================================
 * AUDIT
 * ============================================================================
 *
 * Audit intent describes what provenance security information should be
 * auditable.
 *
 * It does not create an audit log.
 * ============================================================================
 */

securityProvenanceAudit
    : AUDIT
      expression
      SEMI?
    ;


/*
 * ============================================================================
 * REQUIREMENTS
 * ============================================================================
 *
 * Requirements are mandatory semantic conditions.
 * ============================================================================
 */

securityProvenanceRequirement
    : REQUIRES
      securityProvenanceExpression
      SEMI?
    ;


/*
 * ============================================================================
 * CONSTRAINTS
 * ============================================================================
 *
 * Constraints restrict valid realizations.
 * ============================================================================
 */

securityProvenanceConstraint
    : CONSTRAINT
      securityProvenanceExpression
      SEMI?
    ;


/*
 * ============================================================================
 * PREFERENCES
 * ============================================================================
 *
 * Preferences are not mandatory.
 * ============================================================================
 */

securityProvenancePreference
    : PREFER
      securityProvenanceExpression
      SEMI?
    ;


/*
 * ============================================================================
 * HINTS
 * ============================================================================
 *
 * Hints guide implementation without establishing correctness requirements.
 * ============================================================================
 */

securityProvenanceHint
    : HINT
      securityProvenanceExpression
      SEMI?
    ;


/*
 * ============================================================================
 * OPEN-WORLD PROPERTIES
 * ============================================================================
 *
 * Property names are canonical qualified names.
 *
 * Examples:
 *
 *     security::provenance::integrity = required;
 *     security::provenance::classification = restricted;
 *     future::provenance::property = value;
 *
 * No finite property registry is embedded in this grammar.
 * ============================================================================
 */

securityProvenanceProperty
    : qualifiedName
      ASSIGN
      securityProvenanceExpression
      SEMI?
    ;


/*
 * ============================================================================
 * EXPRESSIONS
 * ============================================================================
 *
 * Security provenance deliberately reuses the canonical Zamani expression
 * grammar.
 *
 * This prevents creation of a private security expression language.
 * ============================================================================
 */

securityProvenanceExpression
    : expression
    ;


/*
 * ============================================================================
 * NAMED PROVENANCE REFERENCES
 * ============================================================================
 *
 * This rule exists as an explicit integration contract for downstream semantic
 * analysis.
 *
 * It does not imply that the referenced object exists.
 * Name resolution remains downstream.
 * ============================================================================
 */

securityProvenanceReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * REQUIREMENT TARGETS
 * ============================================================================
 *
 * A security provenance requirement can identify a semantic property through
 * an ordinary qualified name or expression.
 *
 * Examples:
 *
 *     requires security::provenance::integrity;
 *     requires security::provenance::authenticity;
 *     requires capability("provenance.verification");
 *     requires security::trusted_execution;
 * ============================================================================
 */


/*
 * ============================================================================
 * SEMANTIC NORMALIZATION CONTRACT
 * ============================================================================
 *
 * The semantic layer should normalize source constructs into categories such
 * as:
 *
 *     CLAIM
 *     EVIDENCE
 *     ATTESTATION
 *     VERIFICATION
 *     AUDIT
 *     REQUIREMENT
 *     CONSTRAINT
 *     PREFERENCE
 *     HINT
 *     PROPERTY
 *
 * These categories are semantic concepts, not parser-level closed enums.
 *
 * ============================================================================
 * DATA-PROVENANCE INTEGRATION
 * ============================================================================
 *
 * General lineage remains owned by:
 *
 *     grammar/data/provenance.g4
 *
 * Security provenance can reference or constrain that lineage.
 *
 * Example conceptual relationship:
 *
 *     data provenance
 *           |
 *           v
 *     security provenance requirement
 *           |
 *           v
 *     semantic analysis
 *
 * Security provenance MUST NOT reimplement:
 *
 *     derived_from
 *     produced_by
 *     consumed_by
 *     transformed_by
 *     general lineage
 *
 * Those already belong to the data provenance grammar.
 *
 * ============================================================================
 * IDENTITY INTEGRATION
 * ============================================================================
 *
 * Identity references may appear as expressions or qualified names.
 *
 * Identity definition remains owned by:
 *
 *     security/identity.g4
 *     security/identifiers.g4
 *
 * This grammar does not authenticate identities.
 *
 * ============================================================================
 * TRUST INTEGRATION
 * ============================================================================
 *
 * Trust references may be expressed through qualified names and expressions.
 *
 * Trust relationships remain owned by:
 *
 *     security/trust.g4
 *
 * This grammar does not evaluate trust.
 *
 * ============================================================================
 * CRYPTOGRAPHIC INTEGRATION
 * ============================================================================
 *
 * Cryptographic properties may be referenced symbolically:
 *
 *     crypto::integrity
 *     crypto::authenticity
 *     crypto::signature
 *     future::crypto::property
 *
 * Cryptographic syntax and algorithm intent remain owned by:
 *
 *     security/cryptography.g4
 *
 * This grammar does not define algorithms.
 *
 * ============================================================================
 * PRIVACY INTEGRATION
 * ============================================================================
 *
 * Provenance can itself be sensitive.
 *
 * Security provenance may therefore express disclosure restrictions:
 *
 *     requires security::provenance::confidential
 *     constraint security::provenance::disclosure == restricted
 *
 * Privacy semantics remain owned by:
 *
 *     security/privacy.g4
 *
 * ============================================================================
 * AUTHORIZATION INTEGRATION
 * ============================================================================
 *
 * Provenance access may be subject to authorization.
 *
 * Authorization remains owned by:
 *
 *     security/authorization.g4
 *     security/permissions.g4
 *
 * This grammar does not grant permissions.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Security provenance requirements may be checked against target capabilities.
 *
 * Example:
 *
 *     requires capability("provenance.integrity");
 *
 * The grammar does not discover whether such capability exists.
 *
 * Resource/capability analysis occurs downstream.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Security provenance can accompany:
 *
 *     quantum state results;
 *     measurement results;
 *     logical execution;
 *     compilation artifacts;
 *     QEC metadata;
 *     ZQN metadata;
 *     calibration-derived metadata;
 *     scheduling metadata;
 *     routed artifacts.
 *
 * However:
 *
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *     calibration
 *
 * remain owners of their respective semantics.
 *
 * Security provenance only expresses the security requirements surrounding
 * those artifacts.
 *
 * ============================================================================
 * RESILIENCE INTEGRATION
 * ============================================================================
 *
 * Security provenance may need to survive:
 *
 *     retry;
 *     recover;
 *     reroute;
 *     reschedule;
 *     rollback;
 *     recompile;
 *     backend migration;
 *     quarantine.
 *
 * This grammar does not perform any of those actions.
 *
 * Mandatory security provenance requirements MUST be preserved by downstream
 * semantic and resilience transformations.
 *
 * ============================================================================
 * COMPILATION INTEGRATION
 * ============================================================================
 *
 * Security provenance metadata may accompany:
 *
 *     source identity;
 *     semantic identity;
 *     IR identity;
 *     compiled artifact identity;
 *     scheduled artifact identity;
 *     routed artifact identity;
 *     result identity.
 *
 * The compiler is responsible for preserving semantic relationships.
 *
 * This grammar does not prescribe the representation of hashes, signatures,
 * fingerprints, or artifact identifiers.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime systems may:
 *
 *     verify evidence;
 *     evaluate trust;
 *     validate attestations;
 *     enforce disclosure restrictions;
 *     authorize provenance access;
 *     record audit events.
 *
 * None of these operations occur during parsing.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The following are intentionally represented through repetition:
 *
 *     securityProvenanceMember*
 *     securityProvenanceProperty*
 *
 * and through canonical expressions/qualified names.
 *
 * There is no finite grammar-level maximum for:
 *
 *     claims;
 *     evidence items;
 *     attestations;
 *     verification requirements;
 *     audit requirements;
 *     properties;
 *     provenance subjects;
 *     security provenance declarations.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     MAX_*
 *     fixed device count;
 *     fixed node count;
 *     fixed identity count;
 *     fixed policy count;
 *     fixed evidence count;
 *     fixed provenance count;
 *     fixed key count;
 *     fixed certificate count;
 *     fixed hardware identifier;
 *     vendor-specific security implementation;
 *     closed cryptographic algorithm enumeration.
 *
 * ============================================================================
 * RUST 1.97 / 1.97.1 CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust implementation code.
 *
 * Generated parser/compiler integration MUST:
 *
 *     - target Rust 1.97 or 1.97.1;
 *     - use Rust 2021;
 *     - compile without unsafe Rust;
 *     - avoid embedded target-language actions;
 *     - preserve deterministic parsing.
 *
 * The Rust implementation should enforce:
 *
 *     #![forbid(unsafe_code)]
 *
 * at the applicable crate boundary.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] SecurityProvenance is imported by the canonical security composition
 *     root.
 *
 * [ ] General data provenance remains owned by data/provenance.g4.
 *
 * [ ] No second general provenance grammar exists.
 *
 * [ ] The contextual `provenance` introducer is semantically validated.
 *
 * [ ] Security provenance claims are represented structurally.
 *
 * [ ] Evidence is represented without embedding evidence material.
 *
 * [ ] Attestation is represented without performing attestation.
 *
 * [ ] Verification is represented without performing verification.
 *
 * [ ] Audit intent is represented without creating an audit log.
 *
 * [ ] Requirements, constraints, preferences, and hints remain distinct.
 *
 * [ ] Open-world qualified names remain supported.
 *
 * [ ] No machine/hardware limits are encoded.
 *
 * [ ] No secret material syntax is introduced.
 *
 * [ ] No cryptographic algorithm enumeration is introduced.
 *
 * [ ] No quantum-specific implementation types are introduced.
 *
 * [ ] No competing security-provenance IR is introduced.
 *
 * [ ] Source spans are preserved by the frontend AST contract.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Security provenance survives canonical semantic lowering.
 *
 * [ ] Mandatory security requirements survive optimization, routing,
 *     scheduling, resilience, and target lowering.
 *
 * [ ] Rust 1.97 / 1.97.1 integration remains safe Rust only.
 *
 * ============================================================================
 */