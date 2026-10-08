/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/security/provenance.g4
 *
 * GRAMMAR
 * -------
 * SecurityProvenance
 *
 * STATUS
 * ------
 * PRODUCTION SECURITY-DOMAIN LEAF GRAMMAR
 *
 * PURPOSE
 * -------
 * Defines source-level SECURITY CONSTRAINTS, REQUIREMENTS, CLAIMS,
 * EVIDENCE REFERENCES, ATTESTATION REFERENCES, VERIFICATION INTENT,
 * AUDIT INTENT, DISCLOSURE/RETENTION SECURITY INTENT, and security-specific
 * provenance metadata.
 *
 * GENERAL PROVENANCE AUTHORITY
 * ----------------------------
 * General provenance and lineage remain owned by:
 *
 *     grammar/data/provenance.g4
 *     grammar/expressions/provenance.g4
 *     grammar/compile/provenance.g4
 *
 * This file MUST NOT become a second universal provenance grammar.
 *
 * SECURITY PROVENANCE ROLE
 * ------------------------
 * This file expresses how provenance participates in security semantics.
 *
 * It does NOT:
 *
 *     - authenticate;
 *     - authorize;
 *     - evaluate trust;
 *     - verify evidence;
 *     - calculate hashes;
 *     - verify signatures;
 *     - access credentials;
 *     - access secrets;
 *     - discover hardware;
 *     - discover resources;
 *     - select targets;
 *     - execute audit operations;
 *     - persist provenance;
 *     - implement a provenance database;
 *     - implement a ledger;
 *     - implement cryptography;
 *     - implement attestation;
 *     - implement secure boot;
 *     - implement TPM/HSM/TEE functionality;
 *     - implement privacy enforcement;
 *     - implement authorization;
 *     - implement policy evaluation;
 *     - define a competing IR;
 *     - define quantum operations;
 *     - define QEC;
 *     - define ZQN;
 *     - route;
 *     - schedule;
 *     - optimize;
 *     - perform runtime enforcement.
 *
 * ============================================================================
 * ARCHITECTURAL AUTHORITY
 * ============================================================================
 *
 * Human normative authority:
 *
 *     grammar/specification/
 *
 * Machine contracts:
 *
 *     grammar/spec/
 *
 * Security composition:
 *
 *     grammar/security/security.g4
 *
 * General provenance:
 *
 *     grammar/data/provenance.g4
 *
 * Expression provenance:
 *
 *     grammar/expressions/provenance.g4
 *
 * Compilation provenance:
 *
 *     grammar/compile/provenance.g4
 *
 * Trust:
 *
 *     grammar/security/trust.g4
 *
 * Authorization:
 *
 *     grammar/security/authorization.g4
 *
 * Capabilities:
 *
 *     grammar/security/capabilities.g4
 *
 * Generic requirements/capabilities/constraints:
 *
 *     grammar/core/
 *     grammar/resources/
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
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
 * OWNERSHIP CONTRACT
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     securityProvenanceFile
 *     securityProvenanceDeclaration
 *     securityProvenanceBody
 *     securityProvenanceMember
 *     securityProvenanceRequirement
 *     securityProvenanceConstraint
 *     securityProvenancePreference
 *     securityProvenanceHint
 *     securityProvenanceEvidence
 *     securityProvenanceAttestation
 *     securityProvenanceVerification
 *     securityProvenanceAudit
 *     securityProvenanceClaim
 *     securityProvenanceProperty
 *     securityProvenanceReference
 *
 * THIS FILE DOES NOT OWN:
 *
 *     general provenance lineage;
 *     provenance expressions;
 *     data schemas;
 *     source maps;
 *     source spans;
 *     reproducibility;
 *     deterministic compilation;
 *     identity declarations;
 *     principals;
 *     permissions;
 *     authorization;
 *     capabilities;
 *     trust relationships;
 *     cryptographic algorithms;
 *     key management;
 *     privacy policy;
 *     generic contracts;
 *     generic policies;
 *     resource allocation;
 *     effects;
 *     quantum operations;
 *     HDL semantics;
 *     backend realization.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * General lineage:
 *
 *     data/provenance.g4
 *
 * Expression provenance:
 *
 *     expressions/provenance.g4
 *
 * Compilation provenance:
 *
 *     compile/provenance.g4
 *
 * Security interpretation:
 *
 *     security/provenance.g4
 *
 * No file may redefine another owner's provenance syntax merely for
 * convenience.
 *
 * ============================================================================
 * POCO-REAF / OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * This grammar imposes no finite universal capacity.
 *
 * It MUST NOT define or depend on:
 *
 *     MAX_PROVENANCE_RECORDS
 *     MAX_CLAIMS
 *     MAX_EVIDENCE
 *     MAX_ATTESTATIONS
 *     MAX_VERIFICATIONS
 *     MAX_AUDIT_EVENTS
 *     MAX_ENTITIES
 *     MAX_ACTIVITIES
 *     MAX_RELATIONSHIPS
 *     MAX_IDENTITIES
 *     MAX_KEYS
 *     MAX_CERTIFICATES
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_QUBITS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_CPUS
 *     MAX_STORAGE
 *
 * Repetition is represented by grammar repetition.
 *
 * Actual resource limits belong to:
 *
 *     resource analysis
 *     capability negotiation
 *     execution planning
 *     deployment
 *     runtime
 *
 * A target may reject an unsatisfiable security requirement without requiring
 * a change to the source program.
 *
 * ============================================================================
 * OPEN-WORLD SECURITY MODEL
 * ============================================================================
 *
 * Security provenance mechanisms remain open-world.
 *
 * The grammar MUST NOT enumerate:
 *
 *     hash algorithms;
 *     signature algorithms;
 *     attestation technologies;
 *     certificate systems;
 *     identity providers;
 *     trust anchors;
 *     security providers;
 *     hardware roots of trust;
 *     ledger technologies;
 *     future verification mechanisms.
 *
 * Symbolic qualified names and canonical expressions represent such concepts.
 *
 * ============================================================================
 * SECRET-MATERIAL BOUNDARY
 * ============================================================================
 *
 * This grammar contains no syntax for:
 *
 *     passwords;
 *     private keys;
 *     secret keys;
 *     bearer tokens;
 *     API secrets;
 *     authentication secrets;
 *     recovery secrets.
 *
 * A provenance property may reference a symbolic object, but the referenced
 * object is resolved by security infrastructure.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * The grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no Rust;
 *     - no filesystem access;
 *     - no network access;
 *     - no environment inspection;
 *     - no hardware discovery;
 *     - no cryptographic execution;
 *     - no randomness;
 *     - no policy evaluation.
 *
 * Parsing depends only upon:
 *
 *     token stream
 *     composed grammar
 *     grammar version
 *     selected dialect configuration
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST MUST preserve, where applicable:
 *
 *     source span
 *     declaration name
 *     target expression
 *     member kind
 *     member expression
 *     referenced provenance object
 *     requirement/constraint/preference/hint distinction
 *     source ordering
 *     attributes
 *     annotations
 *
 * Recommended semantic categories:
 *
 *     SecurityProvenanceDeclaration
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
 * These are semantic categories, not additional parser grammars.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - whether referenced provenance subjects resolve;
 *     - whether referenced security objects resolve;
 *     - whether evidence is applicable;
 *     - whether a claim is merely asserted or externally supported;
 *     - whether evidence satisfies a requirement;
 *     - whether verification is required;
 *     - whether an attestation is applicable;
 *     - whether authorization permits provenance access;
 *     - whether disclosure restrictions conflict with another policy;
 *     - whether retention requirements are satisfiable;
 *     - whether trust requirements can be satisfied;
 *     - whether capabilities exist;
 *     - whether mandatory requirements survive lowering;
 *     - whether target realization can satisfy security intent.
 *
 * None of these decisions are made by this grammar.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Expressions appearing in this grammar use canonical Zamani expression
 * semantics.
 *
 * This file MUST NOT define a private security expression language.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing produces no runtime effect.
 *
 * Semantic consumers MAY associate security provenance with effects such as:
 *
 *     audit;
 *     disclosure;
 *     verification;
 *     authentication;
 *     authorization;
 *     network;
 *     storage;
 *     foreign;
 *     native;
 *
 * Effect ownership remains in grammar/effects/.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Security provenance may reference required capabilities symbolically.
 *
 * Example semantic intent:
 *
 *     requires capability("provenance.integrity");
 *
 * Capability discovery remains outside this grammar.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Security provenance may participate in resource requirements through the
 * canonical requirement/capability system.
 *
 * This grammar does not define resource quantities or target capacities.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Security provenance requirements MAY participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * semantics owned by the validation subsystem.
 *
 * This file MUST NOT redefine those universal contract constructs.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Security provenance can be constrained by:
 *
 *     authorization;
 *     privacy;
 *     disclosure;
 *     retention;
 *     trust;
 *     sandbox;
 *     security policy.
 *
 * Policy ownership remains outside this file.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * This grammar is a SECURITY CONSUMER of the universal provenance model.
 *
 * It does not create a competing provenance ontology.
 *
 * Security semantics attach to provenance entities, activities, relationships,
 * evidence, decisions, and verification state produced elsewhere.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO IR.
 *
 * Security provenance is normalized into the common semantic model and may
 * accompany:
 *
 *     Classical IR
 *     quantum::ir
 *     HDL/hardware representations
 *     distributed representations
 *     deployment metadata
 *
 * No SecurityProvenanceIR is permitted.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Security provenance may describe security requirements surrounding:
 *
 *     quantum results;
 *     measurement evidence;
 *     execution evidence;
 *     QEC metadata;
 *     routing metadata;
 *     scheduling metadata;
 *     calibration-derived information;
 *     quantum compilation artifacts.
 *
 * It MUST NOT define:
 *
 *     qubits;
 *     gates;
 *     quantum states;
 *     QEC;
 *     routing;
 *     scheduling;
 *     ZQN.
 *
 * The canonical quantum boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Security provenance can accompany HDL and hardware artifacts without
 * defining signals, timing, synthesis, placement, devices, or physical
 * resources.
 *
 * ============================================================================
 * BACKEND BOUNDARY
 * ============================================================================
 *
 * Backends consume semantic security-provenance metadata after:
 *
 *     AST
 *     semantic analysis
 *     type/effect/capability/resource validation
 *     policy analysis
 *     IR lowering
 *
 * Backend realization may attach implementation evidence.
 *
 * The source grammar remains target-independent.
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
 * PUBLIC STANDALONE ENTRY POINT
 * ============================================================================
 */

securityProvenanceFile
    : securityProvenanceDeclaration* EOF
    ;


/*
 * ============================================================================
 * TOP-LEVEL DECLARATION
 * ============================================================================
 *
 * Canonical lexical token:
 *
 *     PROVENANCE
 *
 * The declaration is deliberately explicit.
 *
 * Unlike the previous design, a bare identifier is not accepted as the
 * declaration introducer.
 *
 * A declaration MUST contain a body.
 *
 * This prevents semantically empty provenance declarations.
 * ============================================================================
 */

securityProvenanceDeclaration
    : attributeList?
      visibility?
      PROVENANCE
      securityProvenanceTarget?
      securityProvenanceBody
      SEMI?
    ;


/*
 * ============================================================================
 * TARGET
 * ============================================================================
 *
 * `for <expression>` identifies the semantic object whose security provenance
 * is being constrained or described.
 *
 * The expression may resolve to:
 *
 *     a value;
 *     artifact;
 *     computation;
 *     declaration;
 *     module;
 *     execution;
 *     result;
 *     data object;
 *     quantum result;
 *     hardware artifact;
 *     distributed result;
 *     future domain object.
 *
 * The expression itself remains owned by Expressions.
 * ============================================================================
 */

securityProvenanceTarget
    : FOR expression
    ;


/*
 * ============================================================================
 * BODY
 * ============================================================================
 */

securityProvenanceBody
    : LBRACE securityProvenanceMember+ RBRACE
    ;


/*
 * ============================================================================
 * MEMBER DISPATCH
 * ============================================================================
 *
 * Universal security provenance controls use canonical lexer tokens where
 * those tokens already exist.
 *
 * Open-world security provenance concepts that do not require a universal
 * reserved keyword use identifier assignment:
 *
 *     claim = ...;
 *     attestation = ...;
 *     verification = ...;
 *     disclosure = ...;
 *     retention = ...;
 *
 * This avoids creating a closed keyword universe for future mechanisms.
 * ============================================================================
 */

securityProvenanceMember
    : securityProvenanceRequirement
    | securityProvenanceConstraint
    | securityProvenancePreference
    | securityProvenanceHint
    | securityProvenanceEvidence
    | securityProvenanceAudit
    | securityProvenanceNamedProperty
    ;


/*
 * ============================================================================
 * REQUIREMENT
 * ============================================================================
 *
 * Mandatory semantic condition.
 *
 * Example:
 *
 *     requires security::provenance::integrity;
 * ============================================================================
 */

securityProvenanceRequirement
    : REQUIRES expression SEMI
    ;


/*
 * ============================================================================
 * CONSTRAINT
 * ============================================================================
 *
 * Mandatory restriction on legal realizations.
 * ============================================================================
 */

securityProvenanceConstraint
    : CONSTRAINT expression SEMI
    ;


/*
 * ============================================================================
 * PREFERENCE
 * ============================================================================
 *
 * Non-mandatory realization preference.
 * ============================================================================
 */

securityProvenancePreference
    : PREFER expression SEMI
    ;


/*
 * ============================================================================
 * HINT
 * ============================================================================
 *
 * Non-binding implementation guidance.
 * ============================================================================
 */

securityProvenanceHint
    : HINT expression SEMI
    ;


/*
 * ============================================================================
 * EVIDENCE
 * ============================================================================
 *
 * Evidence is referenced symbolically or through a canonical expression.
 *
 * This does not embed evidence material or perform verification.
 *
 * Example:
 *
 *     evidence external::execution_record;
 *
 * ============================================================================
 */

securityProvenanceEvidence
    : EVIDENCE expression SEMI
    ;


/*
 * ============================================================================
 * AUDIT
 * ============================================================================
 *
 * Audit intent is represented as an expression.
 *
 * It does not create an audit log.
 * ============================================================================
 */

securityProvenanceAudit
    : AUDIT expression SEMI
    ;


/*
 * ============================================================================
 * OPEN-WORLD SECURITY PROPERTY
 * ============================================================================
 *
 * A property is represented as:
 *
 *     <identifier> = <expression>;
 *
 * This provides stable syntax for future security provenance concepts without
 * continuously expanding the universal lexer.
 *
 * Examples:
 *
 *     claim = source::attestation;
 *     attestation = authority::execution;
 *     verification = verifier::policy;
 *     disclosure = security::restricted;
 *     retention = policy::required;
 *     classification = security::sensitive;
 *
 * The semantic layer assigns meaning to the qualified property name.
 *
 * ============================================================================
 */

securityProvenanceNamedProperty
    : securityProvenancePropertyName
      ASSIGN
      expression
      SEMI
    ;


securityProvenancePropertyName
    : qualifiedName
    ;


/*
 * ============================================================================
 * EXPLICIT SECURITY PROVENANCE REFERENCES
 * ============================================================================
 *
 * A reference is symbolic.
 *
 * It does not imply that the object exists.
 *
 * Resolution belongs to semantic analysis.
 * ============================================================================
 */

securityProvenanceReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * SEMANTIC NORMALIZATION
 * ============================================================================
 *
 * The frontend MUST normalize:
 *
 *     requires
 *     constraint
 *     prefer
 *     hint
 *     evidence
 *     audit
 *     named properties
 *
 * into the common provenance/security semantic model.
 *
 * In particular, these semantic distinctions MUST remain intact:
 *
 *     assertion     != evidence
 *     evidence      != verification
 *     verification  != trust
 *     audit intent  != audit execution
 *     requirement   != preference
 *     preference    != hint
 *     policy        != provenance
 *     capability    != authorization
 *     resource      != capability
 *
 * ============================================================================
 * GENERAL PROVENANCE INTEGRATION
 * ============================================================================
 *
 * General provenance relationships such as:
 *
 *     derived_from
 *     generated_by
 *     consumed_by
 *     transformed_by
 *
 * remain owned by the general provenance subsystem.
 *
 * This file may constrain those relationships semantically, but MUST NOT
 * redefine them.
 *
 * ============================================================================
 * DATA INTEGRATION
 * ============================================================================
 *
 * Data provenance:
 *
 *     grammar/data/provenance.g4
 *
 * Security provenance may constrain:
 *
 *     disclosure;
 *     integrity;
 *     authenticity;
 *     retention;
 *     evidence;
 *     verification;
 *     authorization.
 *
 * Data lineage remains data-provenance-owned.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * Expression provenance:
 *
 *     grammar/expressions/provenance.g4
 *
 * owns provenance(...) expression syntax.
 *
 * This grammar MUST NOT define provenance(...).
 *
 * ============================================================================
 * COMPILATION INTEGRATION
 * ============================================================================
 *
 * Compilation provenance:
 *
 *     grammar/compile/provenance.g4
 *
 * owns compiler/build transformation provenance.
 *
 * This file may constrain the security requirements surrounding those
 * artifacts but does not redefine compilation provenance.
 *
 * ============================================================================
 * TRUST INTEGRATION
 * ============================================================================
 *
 * Trust relationships remain owned by:
 *
 *     grammar/security/trust.g4
 *
 * Security provenance may reference trust objects through:
 *
 *     expression
 *     qualifiedName
 *
 * The semantic layer resolves the relationship.
 *
 * This grammar does not evaluate trust.
 *
 * ============================================================================
 * AUTHORIZATION INTEGRATION
 * ============================================================================
 *
 * Authorization remains owned by:
 *
 *     grammar/security/authorization.g4
 *
 * and the permission subsystem.
 *
 * Provenance does not grant permission.
 *
 * Provenance access authorization is resolved downstream.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Capabilities remain owned by:
 *
 *     grammar/security/capabilities.g4
 *     grammar/core/capabilities.g4
 *     grammar/resources/
 *
 * A provenance requirement may reference a capability:
 *
 *     requires capability("provenance.integrity");
 *
 * The grammar does not determine whether the capability is available.
 *
 * ============================================================================
 * PRIVACY INTEGRATION
 * ============================================================================
 *
 * Privacy semantics remain owned by:
 *
 *     grammar/security/privacy.g4
 *
 * Security provenance may reference privacy/disclosure requirements through
 * canonical expressions and properties.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Policies remain owned by the policy subsystem.
 *
 * Security provenance may be governed by policies concerning:
 *
 *     disclosure;
 *     retention;
 *     verification;
 *     evidence;
 *     audit;
 *     authorization;
 *     trust.
 *
 * This file does not define policy bodies.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Security provenance may be associated semantically with effects such as:
 *
 *     audit;
 *     verification;
 *     disclosure;
 *     network;
 *     storage;
 *     native;
 *     foreign.
 *
 * Effect syntax remains owned by grammar/effects/.
 *
 * ============================================================================
 * SANDBOX INTEGRATION
 * ============================================================================
 *
 * A sandbox may constrain:
 *
 *     provenance disclosure;
 *     provenance persistence;
 *     audit access;
 *     external evidence access;
 *     verification services.
 *
 * Sandbox syntax remains owned by:
 *
 *     grammar/security/sandbox.g4
 *
 * ============================================================================
 * RESOURCE / TARGET INTEGRATION
 * ============================================================================
 *
 * Provenance security requirements may fail because the selected realization
 * cannot satisfy them.
 *
 * That is a semantic/resource diagnostic, not a parser error.
 *
 * The grammar therefore remains independent of:
 *
 *     CPU count;
 *     GPU count;
 *     QPU count;
 *     node count;
 *     memory size;
 *     storage size;
 *     network size;
 *     device count.
 *
 * ============================================================================
 * RESILIENCE INTEGRATION
 * ============================================================================
 *
 * Security provenance MUST survive semantic transformations including:
 *
 *     retry;
 *     recovery;
 *     rerouting;
 *     rescheduling;
 *     checkpoint restoration;
 *     recompilation;
 *     backend migration;
 *     quarantine.
 *
 * This grammar does not implement those operations.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Security provenance may accompany:
 *
 *     quantum execution;
 *     measurement;
 *     QEC metadata;
 *     routing;
 *     scheduling;
 *     calibration;
 *     QPU realization.
 *
 * All quantum semantic information remains downstream.
 *
 * Canonical quantum boundary:
 *
 *     quantum::ir
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Security provenance may accompany:
 *
 *     HDL source;
 *     synthesized artifacts;
 *     hardware descriptions;
 *     accelerators;
 *     physical realization metadata.
 *
 * It does not define hardware syntax.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Security provenance must support semantic provenance associated with:
 *
 *     actors;
 *     tasks;
 *     services;
 *     messages;
 *     distributed results;
 *     retries;
 *     partial failures;
 *     recovery.
 *
 * No node or participant count is hard-coded.
 *
 * ============================================================================
 * AI / LEARNING / ADAPTATION INTEGRATION
 * ============================================================================
 *
 * Security provenance may accompany:
 *
 *     model derivation;
 *     training;
 *     inference;
 *     evidence;
 *     adaptation;
 *     decisions;
 *     explanations.
 *
 * It does not define AI semantics.
 *
 * ============================================================================
 * PROVENANCE TRUST INVARIANTS
 * ============================================================================
 *
 * The semantic model MUST preserve:
 *
 *     claim       != evidence
 *     evidence    != verification
 *     verification != trust
 *     observation != prediction
 *     decision    != evidence
 *     simulation  != observation
 *
 * A source-level claim is never proof merely because it was parsed.
 *
 * ============================================================================
 * PARTIAL / DISTRIBUTED PROVENANCE
 * ============================================================================
 *
 * Semantic provenance MUST be able to represent:
 *
 *     partial lineage;
 *     unresolved external references;
 *     distributed lineage;
 *     incremental lineage;
 *     branching lineage;
 *     merged lineage;
 *     superseded evidence;
 *     retracted claims;
 *     conflicting evidence.
 *
 * These states are semantic concerns and MUST NOT be encoded by artificial
 * grammar-level limits.
 *
 * ============================================================================
 * EXTENSIBILITY CONTRACT
 * ============================================================================
 *
 * New security provenance concepts should first use:
 *
 *     qualified names;
 *     expressions;
 *     named properties;
 *     dialect extensions.
 *
 * A new reserved keyword should be introduced only when the construct has
 * demonstrated universal language-level status.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Structural parser diagnostics MUST cover:
 *
 *     missing provenance body;
 *     empty provenance body;
 *     missing requirement expression;
 *     missing constraint expression;
 *     missing preference expression;
 *     missing hint expression;
 *     missing evidence expression;
 *     missing audit expression;
 *     missing property value;
 *     malformed qualified property name.
 *
 * Semantic diagnostics belong downstream and include:
 *
 *     unresolved provenance reference;
 *     unsatisfied security requirement;
 *     unsupported verification requirement;
 *     unauthorized provenance access;
 *     incompatible disclosure policy;
 *     invalid trust reference;
 *     invalid capability reference;
 *     invalid retention requirement;
 *     invalid evidence applicability;
 *     unsupported target realization.
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * At minimum test:
 *
 *     provenance {
 *         requires security::provenance::integrity;
 *     }
 *
 *     provenance for result {
 *         evidence = execution::record;
 *         verification = security::verifier;
 *     }
 *
 *     provenance for quantum_result {
 *         requires capability("provenance.integrity");
 *         requires quantum::measurement_provenance;
 *     }
 *
 *     provenance for artifact {
 *         claim = source::attestation;
 *         attestation = authority::execution;
 *         audit security::audit::required;
 *         disclosure = security::restricted;
 *         retention = policy::required;
 *     }
 *
 *     provenance for distributed_result {
 *         requires security::provenance::authenticity;
 *         prefer security::provenance::attestation;
 *     }
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * MUST reject:
 *
 *     provenance;
 *
 *     provenance {};
 *
 *     provenance {
 *     }
 *
 *     provenance {
 *         requires;
 *     }
 *
 *     provenance {
 *         evidence;
 *     }
 *
 *     provenance {
 *         claim =;
 *     }
 *
 *     provenance {
 *         = expression;
 *     }
 *
 *     provenance {
 *         requires security::provenance::integrity
 *     }
 *
 * where the required semicolon is part of the canonical statement syntax.
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * MUST cover:
 *
 *     - data + security provenance;
 *     - compile + security provenance;
 *     - expression + security provenance;
 *     - classical + security provenance;
 *     - quantum + security provenance;
 *     - HDL + security provenance;
 *     - AI + security provenance;
 *     - distributed + security provenance;
 *     - resource + security provenance;
 *     - capability + security provenance;
 *     - trust + security provenance;
 *     - authorization + security provenance;
 *     - privacy + security provenance;
 *     - sandbox + security provenance;
 *     - policy + security provenance.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests MUST distinguish:
 *
 *     grammar acceptance limits
 *
 * from:
 *
 *     implementation/resource exhaustion.
 *
 * The semantic model must remain valid for arbitrarily large provenance
 * graphs subject only to available implementation resources.
 *
 * Test dimensions include:
 *
 *     declarations;
 *     members;
 *     evidence references;
 *     properties;
 *     provenance depth;
 *     branching;
 *     merging;
 *     distributed participants;
 *     cross-domain artifacts.
 *
 * No universal finite maximum is permitted.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Identical source and identical parser configuration MUST produce equivalent
 * parse structures independent of:
 *
 *     wall-clock time;
 *     hardware;
 *     network;
 *     filesystem;
 *     environment;
 *     runtime security state.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * New security provenance properties should be introduced through open-world
 * qualified names before becoming reserved language keywords.
 *
 * Existing valid provenance syntax MUST retain its meaning across compatible
 * language versions.
 *
 * Deprecated constructs require explicit compatibility metadata and diagnostics
 * in the compatibility subsystem.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * The grammar contains no Rust code.
 *
 * The generated frontend implementation MUST target:
 *
 *     Rust 1.97 or later
 *     Rust 2021 or later
 *
 * and MUST use safe Rust only.
 *
 * The applicable Rust crate SHOULD enforce:
 *
 *     #![forbid(unsafe_code)]
 *
 * No grammar feature requires unsafe Rust.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/
 *     grammar/types/
 *     grammar/expressions/
 *
 * EXPORTS:
 *
 *     securityProvenanceFile
 *     securityProvenanceDeclaration
 *     securityProvenanceBody
 *     securityProvenanceMember
 *     securityProvenanceRequirement
 *     securityProvenanceConstraint
 *     securityProvenancePreference
 *     securityProvenanceHint
 *     securityProvenanceEvidence
 *     securityProvenanceAudit
 *     securityProvenanceNamedProperty
 *     securityProvenanceReference
 *
 * CONSUMED_BY:
 *
 *     grammar/security/security.g4
 *     security semantic analysis
 *     security provenance AST construction
 *     security tooling
 *     conformance tests
 *
 * AST_OWNER:
 *
 *     src/frontend/ast/
 *
 * SEMANTIC_OWNER:
 *
 *     security semantic model
 *
 * IR_OWNER:
 *
 *     existing canonical domain IRs
 *     quantum::ir for quantum semantics
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/security.md
 *     grammar/spec/provenance.md
 *
 * TEST_OWNER:
 *
 *     grammar/tests/security/provenance/
 *
 * RESOURCE_CONTRACT:
 *
 *     No source-level finite resource limits.
 *
 * CAPABILITY_CONTRACT:
 *
 *     Capability resolution is downstream.
 *
 * EFFECT_CONTRACT:
 *
 *     Effect classification is downstream.
 *
 * CONTRACT_CONTRACT:
 *
 *     Universal contract semantics remain validation-owned.
 *
 * POLICY_CONTRACT:
 *
 *     Policy evaluation remains policy/security-owned.
 *
 * PROVENANCE_CONTRACT:
 *
 *     This grammar consumes the universal provenance semantic model.
 *
 * SCALABILITY_CONTRACT:
 *
 *     Open-world; no hard-coded finite provenance capacity.
 *
 * COMPATIBILITY_CONTRACT:
 *
 *     Qualified-name extension before keyword promotion.
 *
 * COMPLETION_CRITERIA:
 *
 *     This file is complete when:
 *
 *     1. It compiles with the canonical Zamani lexer.
 *     2. It imports only canonical shared grammar facilities.
 *     3. It is imported by Security.
 *     4. It owns no general provenance semantics.
 *     5. It contains no duplicate trust/authorization/capability grammar.
 *     6. It contains no secret-material syntax.
 *     7. It contains no cryptographic implementation.
 *     8. It contains no target-specific capacity.
 *     9. It has positive tests.
 *    10. It has negative tests.
 *    11. It has boundary tests.
 *    12. It has scalability tests.
 *    13. It has determinism tests.
 *    14. It has compatibility tests.
 *    15. Its semantic nodes have explicit AST/semantic ownership.
 *    16. Its security requirements survive canonical lowering.
 *    17. Quantum security provenance reaches quantum::ir as metadata rather
 *        than creating another quantum IR.
 *    18. Rust integration remains safe Rust 1.97+.
 *
 * ============================================================================
 */