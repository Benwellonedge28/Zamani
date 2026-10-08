/*
 * ============================================================================
 * ZAMANI UNIVERSAL PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/security/audit.g4
 *
 * GRAMMAR
 * -------
 * SecurityAudit
 *
 * STATUS
 * ------
 * CANONICAL PRODUCTION SECURITY-AUDIT GRAMMAR
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This grammar is the single parser-level owner of SECURITY AUDIT INTENT.
 *
 * It defines source syntax for expressing:
 *
 *     - audit declarations;
 *     - audit requirements;
 *     - audit targets;
 *     - audit traces;
 *     - audit evidence;
 *     - audit verification intent;
 *     - audit decisions;
 *     - audit provenance references;
 *     - audit constraints;
 *     - audit preferences;
 *     - audit hints;
 *     - audit properties;
 *     - audit attachments;
 *     - audit statements.
 *
 * Audit syntax expresses WHAT must be observed, recorded, verified, traced,
 * explained, or associated with provenance.
 *
 * It does NOT implement auditing.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     auditFile
 *     securityAuditDeclaration
 *     auditBody
 *     auditMember
 *     auditRequirement
 *     auditTarget
 *     auditTrace
 *     auditEvidence
 *     auditVerification
 *     auditDecision
 *     auditProvenance
 *     auditConstraint
 *     auditPreference
 *     auditHint
 *     auditProperty
 *     auditAttachment
 *     auditStatement
 *     auditReference
 *     auditReferenceList
 *     auditPropertyName
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     identity declarations
 *     principals
 *     authentication
 *     authorization
 *     permissions
 *     capabilities
 *     policies
 *     trust relationships
 *     general provenance
 *     data provenance
 *     contracts
 *     effects
 *     resources
 *     types
 *     expressions
 *     persistence
 *     serialization
 *     logging implementation
 *     telemetry implementation
 *     cryptography
 *     attestation
 *     hardware discovery
 *     target selection
 *     runtime enforcement
 *     quantum operations
 *     quantum routing
 *     QEC
 *     ZQN
 *     HDL synthesis
 *     backend selection
 *
 * Those responsibilities remain with their canonical owners.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This file is the ONLY grammar owner of:
 *
 *     securityAuditDeclaration
 *     security audit declaration members
 *     auditStatement
 *
 * Other grammars MUST consume these rules rather than reproduce them.
 *
 * In particular:
 *
 *     grammar/security/security.g4
 *     grammar/effects/security.g4
 *     grammar/security/sandbox.g4
 *     grammar/security/provenance.g4
 *     grammar/statements/
 *
 * MUST NOT define competing audit grammar rules.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     ANTLR parser
 *          |
 *          v
 *     SecurityAudit
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *     +----+----+----+----+----+
 *     |    |    |    |    |    |
 *     v    v    v    v    v    v
 *   type effect capability resource policy provenance
 *          |
 *          v
 *     security semantic model
 *          |
 *     +-------------------------+
 *     |            |            |
 *     v            v            v
 * classical    quantum::ir   HDL/hardware
 *     |            |            |
 *     +------------+------------+
 *                  |
 *                  v
 *       optimization / lowering
 *                  |
 *          routing / scheduling
 *                  |
 *       resilience / recovery
 *                  |
 *                 ZQN
 *                  |
 *                 HAL
 *                  |
 *             target runtime
 *
 * Audit syntax therefore remains SOURCE-LEVEL INTENT.
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * ANTLR4 parser grammar.
 *
 * Rust:
 *
 *     Rust 1.97+
 *     Rust 2021+
 *
 * Safety:
 *
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no parser actions;
 *     - no unsafe implementation requirement;
 *     - no filesystem access;
 *     - no network access;
 *     - no environment inspection;
 *     - no credential access;
 *     - no secret access;
 *     - no runtime execution.
 *
 * Generated Rust frontend code MUST remain safe Rust.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * The canonical parser-facing lexical vocabulary is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * backed by:
 *
 *     grammar/lexer/tokens.g4
 *
 * This grammar does NOT define lexical tokens.
 *
 * Existing canonical audit/provenance vocabulary includes:
 *
 *     AUDIT
 *     TRACE
 *     EVIDENCE
 *     EXPLAIN
 *     PROVENANCE
 *     SOURCE
 *     DERIVATION
 *     DECISION
 *     GENERATED
 *     TRANSFORMED
 *     VERIFIED
 *     REQUIRES
 *     CONSTRAINT
 *     PREFER
 *     HINT
 *     WITH
 *     EFFECTS
 *     VERIFY
 *
 * No additional audit-specific token is required by this grammar.
 *
 * This deliberately prevents unnecessary lexical expansion.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * Core:
 *
 *     attributes
 *     visibility
 *     qualifiedName
 *     annotation
 *
 * Types:
 *
 *     canonical type syntax where required by expressions
 *
 * Expressions:
 *
 *     expression
 *     argumentList
 *
 * Audit does NOT import or reproduce:
 *
 *     generic expression grammar
 *     generic type grammar
 *     provenance grammar
 *     policy grammar
 *     capability grammar
 *     authorization grammar
 *
 * References to those systems remain symbolic/open-world references.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Audit syntax is target-independent.
 *
 * It MUST NOT encode:
 *
 *     a fixed number of audit records;
 *     a fixed number of events;
 *     a fixed number of traces;
 *     a fixed number of principals;
 *     a fixed number of devices;
 *     a fixed number of nodes;
 *     a fixed number of CPUs;
 *     a fixed number of GPUs;
 *     a fixed number of FPGAs;
 *     a fixed number of QPUs;
 *     a fixed amount of memory;
 *     a fixed storage capacity;
 *     a fixed retention capacity.
 *
 * No language-level constants such as:
 *
 *     MAX_AUDIT_EVENTS
 *     MAX_AUDIT_RECORDS
 *     MAX_TRACE_ENTRIES
 *     MAX_EVIDENCE
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * are permitted here.
 *
 * Practical implementation limits belong to:
 *
 *     resource policy
 *     runtime configuration
 *     deployment policy
 *     storage policy
 *     execution environment
 *
 * A target may reject an unsatisfiable audit requirement without changing
 * the source program.
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * Audit targets, evidence sources, policies, provenance objects, security
 * authorities, trace providers, verification mechanisms, and storage systems
 * are open-world.
 *
 * The grammar MUST NOT enumerate:
 *
 *     vendors
 *     logging systems
 *     SIEM systems
 *     databases
 *     ledger technologies
 *     cloud providers
 *     hardware monitors
 *     security providers
 *     attestation technologies
 *     verification algorithms
 *     future audit mechanisms
 *
 * Symbolic qualified names and expressions represent these concepts.
 *
 * ============================================================================
 * SECRET-MATERIAL BOUNDARY
 * ============================================================================
 *
 * Audit syntax MUST NOT provide a mechanism for embedding:
 *
 *     passwords
 *     private keys
 *     secret keys
 *     bearer tokens
 *     API secrets
 *     authentication credentials
 *     recovery secrets
 *
 * Audit records may reference security objects symbolically.
 *
 * Actual secrets belong to secure runtime/key-management infrastructure.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * This grammar describes intent only.
 *
 * It does NOT:
 *
 *     authenticate;
 *     authorize;
 *     verify;
 *     evaluate policy;
 *     persist audit records;
 *     transmit audit records;
 *     encrypt audit records;
 *     discover hardware;
 *     discover resources;
 *     allocate storage;
 *     select targets;
 *     select logging providers;
 *     execute tracing;
 *     execute telemetry;
 *     modify program state.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must preserve:
 *
 *     - declaration kind;
 *     - declaration name;
 *     - member ordering;
 *     - reference structure;
 *     - expression structure;
 *     - source spans;
 *     - annotations;
 *     - attributes;
 *     - source-level spelling where required for diagnostics.
 *
 * Conceptual semantic nodes include:
 *
 *     AuditDeclaration
 *     AuditRequirement
 *     AuditTarget
 *     AuditTrace
 *     AuditEvidence
 *     AuditVerification
 *     AuditDecision
 *     AuditProvenance
 *     AuditConstraint
 *     AuditPreference
 *     AuditHint
 *     AuditProperty
 *     AuditAttachment
 *
 * The grammar does NOT define Rust AST structs.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis resolves:
 *
 *     - audit references;
 *     - referenced security objects;
 *     - referenced provenance objects;
 *     - referenced policies;
 *     - referenced capabilities;
 *     - referenced resources;
 *     - effect implications;
 *     - contract interactions;
 *     - target feasibility;
 *     - audit obligations;
 *     - conflicts between audit requirements and policies;
 *     - provenance requirements;
 *     - retention/security requirements.
 *
 * None of these decisions occur during parsing.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Audit expressions use the canonical expression/type system.
 *
 * This file MUST NOT define:
 *
 *     audit-specific primitive types;
 *     audit-specific containers;
 *     audit-specific numeric types;
 *     audit-specific string types.
 *
 * Any audit value is represented by the normal Zamani expression/type system.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Audit intent may semantically introduce or require effects such as:
 *
 *     security::audit
 *     security::trace
 *     io
 *     network
 *     foreign
 *     distributed
 *
 * This grammar does not redefine the effect system.
 *
 * Effect ownership remains:
 *
 *     grammar/effects/
 *
 * The semantic layer determines the actual effect set.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Audit declarations may semantically require capabilities such as:
 *
 *     security::audit
 *     security::audit.record
 *     security::audit.trace
 *     security::provenance.record
 *     security::integrity
 *
 * These names remain open-world references.
 *
 * Capability ownership remains with:
 *
 *     grammar/core/capabilities.g4
 *     grammar/resources/
 *     grammar/security/capabilities.g4
 *
 * This file does not define capability resolution.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Audit requirements may imply resources such as:
 *
 *     storage;
 *     durable storage;
 *     network connectivity;
 *     execution capacity;
 *     secure memory;
 *     provenance storage;
 *     trace collection.
 *
 * Resource feasibility is downstream.
 *
 * The grammar MUST NOT define finite resource capacities.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Audit requirements may participate in universal:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * semantics.
 *
 * This file only owns audit-specific use of:
 *
 *     REQUIRES
 *     CONSTRAINT
 *     PREFER
 *     HINT
 *
 * Universal contract semantics remain owned by validation/contract systems.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Audit declarations may reference policies symbolically:
 *
 *     security::audit_policy
 *     retention::policy
 *     privacy::policy
 *     compliance::policy
 *
 * Policy declaration and policy evaluation remain outside this grammar.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Audit is a consumer of provenance, not a competing provenance system.
 *
 * General provenance:
 *
 *     grammar/data/provenance.g4
 *
 * Security provenance:
 *
 *     grammar/security/provenance.g4
 *
 * Audit may reference provenance objects and may require provenance capture.
 *
 * It MUST NOT redefine:
 *
 *     provenanceDeclaration
 *     provenanceExpression
 *     lineage
 *     source maps
 *     transformation graphs
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Audit may apply to:
 *
 *     quantum programs;
 *     quantum measurements;
 *     quantum-classical control;
 *     QEC metadata;
 *     execution decisions;
 *     quantum resource negotiation.
 *
 * This grammar MUST NOT define:
 *
 *     qubits;
 *     gates;
 *     quantum operation catalogues;
 *     topology;
 *     calibration;
 *     routing;
 *     QEC;
 *     ZQN.
 *
 * Security/audit metadata associated with quantum computation eventually
 * accompanies:
 *
 *     quantum::ir
 *
 * ============================================================================
 * HDL BOUNDARY
 * ============================================================================
 *
 * Audit may apply to:
 *
 *     HDL source;
 *     simulation;
 *     verification;
 *     synthesis intent;
 *     hardware realization metadata.
 *
 * This grammar does not define:
 *
 *     signals;
 *     registers;
 *     timing;
 *     synthesis;
 *     placement;
 *     routing.
 *
 * ============================================================================
 * BACKEND BOUNDARY
 * ============================================================================
 *
 * Audit semantics may be attached to:
 *
 *     classical IR;
 *     quantum::ir;
 *     HDL/hardware representations;
 *     distributed execution plans;
 *     deployment metadata.
 *
 * Lowering MUST preserve mandatory audit/security requirements.
 *
 * Optimization MUST NOT silently remove mandatory audit intent.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source tokens;
 *     grammar version;
 *     composed grammar;
 *     explicitly selected dialect/compatibility configuration.
 *
 * Parsing MUST NOT depend on:
 *
 *     current time;
 *     randomness;
 *     hardware availability;
 *     network state;
 *     filesystem state;
 *     credentials;
 *     runtime state;
 *     policy evaluation results.
 *
 * ============================================================================
 */

parser grammar SecurityAudit;

options {
    tokenVocab = ZamaniLexer;
}

import
    Core,
    Types,
    Expressions
;


/*
 * ============================================================================
 * 1. STANDALONE AUDIT FILE
 * ============================================================================
 *
 * Used for:
 *
 *     - isolated grammar tests;
 *     - editor tooling;
 *     - documentation tooling;
 *     - security grammar validation.
 *
 * The complete Zamani parser enters through its canonical parser root.
 */

auditFile
    : auditTopLevelConstruct*
      EOF
    ;


auditTopLevelConstruct
    : securityAuditDeclaration
    | auditStatement
    ;


/*
 * ============================================================================
 * 2. AUDIT DECLARATION
 * ============================================================================
 *
 * Canonical declaration form:
 *
 *     audit security_execution {
 *         requires security::audit;
 *         target security::execution;
 *         trace security::decision;
 *         evidence security::attestation;
 *         verify security::integrity;
 *         provenance security::compile;
 *     }
 *
 * A declaration has an explicit name and body.
 *
 * This prevents an empty anonymous audit construct from becoming valid
 * language syntax.
 */

securityAuditDeclaration
    : attributes?
      visibility?
      AUDIT
      identifier
      auditBody
      SEMI?
    ;


auditBody
    : LBRACE
      auditMember+
      RBRACE
    ;


/*
 * ============================================================================
 * 3. AUDIT MEMBERS
 * ============================================================================
 *
 * At least one member is required by auditBody.
 *
 * This prevents:
 *
 *     audit empty {}
 *
 * from being accepted as a semantically meaningless audit declaration.
 */

auditMember
    : auditRequirement
    | auditTarget
    | auditTrace
    | auditEvidence
    | auditVerification
    | auditDecision
    | auditProvenance
    | auditConstraint
    | auditPreference
    | auditHint
    | auditProperty
    | annotation
    ;


/*
 * ============================================================================
 * 4. REQUIREMENT
 * ============================================================================
 *
 * A requirement is mandatory semantic intent.
 *
 * Example:
 *
 *     requires security::audit;
 *
 *     requires capability("security.audit.record");
 *
 * The expression remains canonical Zamani expression syntax.
 */

auditRequirement
    : REQUIRES
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 5. TARGET
 * ============================================================================
 *
 * Identifies the logical subject/artifact/operation to which audit intent
 * applies.
 *
 * This is deliberately an expression rather than a physical target.
 *
 * Examples:
 *
 *     target security::execution;
 *     target quantum::measurement;
 *     target computation;
 *     target result;
 */

auditTarget
    : TARGET
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 6. TRACE
 * ============================================================================
 *
 * Expresses tracing intent.
 *
 * Trace semantics remain downstream.
 */

auditTrace
    : TRACE
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 7. EVIDENCE
 * ============================================================================
 *
 * Identifies evidence associated with an audit obligation.
 *
 * Evidence verification is downstream.
 */

auditEvidence
    : EVIDENCE
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 8. VERIFICATION
 * ============================================================================
 *
 * Expresses verification intent.
 *
 * The VERIFY expression itself remains part of the universal expression
 * vocabulary where applicable.
 */

auditVerification
    : VERIFY
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 9. DECISION
 * ============================================================================
 *
 * Associates an audit record with a decision or decision record.
 *
 * This does not evaluate the decision.
 */

auditDecision
    : DECISION
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 10. PROVENANCE
 * ============================================================================
 *
 * Associates audit intent with a provenance reference/expression.
 *
 * General provenance semantics remain outside this grammar.
 */

auditProvenance
    : PROVENANCE
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 11. CONSTRAINT
 * ============================================================================
 *
 * A constraint restricts otherwise valid audit realization.
 *
 * Example:
 *
 *     constraint security::integrity;
 */

auditConstraint
    : CONSTRAINT
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 12. PREFERENCE
 * ============================================================================
 *
 * A preference expresses desirable rather than mandatory realization intent.
 */

auditPreference
    : PREFER
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 13. HINT
 * ============================================================================
 *
 * A hint is advisory metadata.
 *
 * It MUST NOT be interpreted as a mandatory requirement.
 */

auditHint
    : HINT
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 14. OPEN-WORLD PROPERTY
 * ============================================================================
 *
 * Properties provide extensibility without requiring a new universal keyword
 * for every future audit concept.
 *
 * Examples:
 *
 *     retention_policy = security::retention;
 *     classification = security::restricted;
 *     provider = security::audit_provider;
 *     custom::property = expression;
 *
 * Property names are qualified names.
 */

auditProperty
    : auditPropertyName
      ASSIGN
      expression
      SEMI
    ;


auditPropertyName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 15. AUDIT ATTACHMENT
 * ============================================================================
 *
 * An attachment associates an existing expression with audit metadata.
 *
 * Example:
 *
 *     audit(result) {
 *         trace security::decision;
 *         provenance compile::result;
 *     };
 *
 * This is deliberately separate from a named audit declaration.
 */

auditAttachment
    : AUDIT
      LPAREN
      expression
      RPAREN
      auditBody
      SEMI?
    ;


/*
 * ============================================================================
 * 16. AUDIT STATEMENT
 * ============================================================================
 *
 * A lightweight source-level audit request.
 *
 * Examples:
 *
 *     audit result;
 *     audit security::decision;
 *
 * The expression is optional to preserve a generic audit operation form:
 *
 *     audit;
 *
 * Such a form is interpreted semantically as a request for the current
 * security/audit context and may be rejected downstream if no applicable
 * context exists.
 *
 * Named declarations remain the preferred form for reusable audit intent.
 */

auditStatement
    : AUDIT
      expression?
      SEMI
    ;


/*
 * ============================================================================
 * 17. AUDIT REFERENCE
 * ============================================================================
 *
 * A symbolic audit reference is an open-world qualified name.
 *
 * The semantic layer resolves the referenced object.
 */

auditReference
    : qualifiedName
    ;


auditReferenceList
    : auditReference
      (COMMA auditReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * 18. OPTIONAL COMPOSITION FORM
 * ============================================================================
 *
 * This rule permits an audit property or extension to refer to a list of
 * logical audit objects without creating a new closed-world catalogue.
 */

auditReferenceExpression
    : auditReference
    | expression
    ;


/*
 * ============================================================================
 * 19. SOURCE-LEVEL NORMALIZATION
 * ============================================================================
 *
 * The parser-level constructs normalize semantically to:
 *
 *     requirement
 *     target
 *     trace
 *     evidence
 *     verification
 *     decision
 *     provenance
 *     constraint
 *     preference
 *     hint
 *     property
 *
 * The semantic layer may map them to the universal security/provenance model.
 *
 * No audit-specific IR is created here.
 */


/*
 * ============================================================================
 * 20. NO RUNTIME AUDIT IMPLEMENTATION
 * ============================================================================
 *
 * The following are explicitly outside this grammar:
 *
 *     log writing
 *     event persistence
 *     log rotation
 *     retention enforcement
 *     log transport
 *     encryption
 *     signatures
 *     integrity verification
 *     SIEM integration
 *     telemetry collection
 *     metrics collection
 *     network transmission
 *     database insertion
 *     filesystem writes
 *     secure storage
 *     hardware monitoring
 *
 * Those are implementation/runtime concerns.
 */


/*
 * ============================================================================
 * 21. SECURITY INTEGRATION
 * ============================================================================
 *
 * Audit can consume references from:
 *
 *     security/authorization.g4
 *     security/capabilities.g4
 *     security/trust.g4
 *     security/provenance.g4
 *     security/security-constraints.g4
 *     policies/
 *
 * These references remain symbolic at parse time.
 *
 * No dependency on concrete security implementation is introduced.
 */


/*
 * ============================================================================
 * 22. EFFECT INTEGRATION
 * ============================================================================
 *
 * Audit intent may result in semantic effects such as:
 *
 *     security::audit
 *     security::trace
 *     io
 *     network
 *     distributed
 *
 * Effect ownership remains:
 *
 *     grammar/effects/
 *
 * This grammar does not redefine effect syntax.
 */


/*
 * ============================================================================
 * 23. RESOURCE INTEGRATION
 * ============================================================================
 *
 * Audit requirements may result in resource requirements.
 *
 * Examples include:
 *
 *     storage;
 *     secure storage;
 *     network;
 *     durable provenance;
 *     trace capacity.
 *
 * Resource resolution is performed downstream.
 *
 * No resource capacity is hard-coded here.
 */


/*
 * ============================================================================
 * 24. POLICY INTEGRATION
 * ============================================================================
 *
 * Audit may be constrained by:
 *
 *     security policies;
 *     privacy policies;
 *     retention policies;
 *     deployment policies;
 *     provenance policies;
 *     execution policies.
 *
 * Policy declaration and evaluation remain outside this grammar.
 */


/*
 * ============================================================================
 * 25. PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Audit is a provenance consumer.
 *
 * Security provenance:
 *
 *     grammar/security/provenance.g4
 *
 * General provenance:
 *
 *     grammar/data/provenance.g4
 *
 * Compilation provenance:
 *
 *     grammar/compile/provenance.g4
 *
 * Audit must not create a competing provenance grammar.
 */


/*
 * ============================================================================
 * 26. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Valid semantic audit targets include, for example:
 *
 *     quantum::measurement
 *     quantum::execution
 *     quantum::resource_negotiation
 *     quantum::decision
 *
 * These remain qualified names/expressions.
 *
 * No quantum operation catalogue is embedded here.
 *
 * Security metadata eventually accompanies:
 *
 *     quantum::ir
 */


/*
 * ============================================================================
 * 27. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Audit may apply to:
 *
 *     HDL verification;
 *     simulation;
 *     synthesis;
 *     deployment;
 *     hardware security properties.
 *
 * This grammar remains independent of the physical implementation.
 */


/*
 * ============================================================================
 * 28. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Audit may apply to:
 *
 *     actors;
 *     tasks;
 *     messages;
 *     services;
 *     nodes;
 *     distributed execution;
 *     recovery;
 *     failover.
 *
 * No finite node/device/event count is encoded.
 */


/*
 * ============================================================================
 * 29. ADAPTIVE / RESILIENT EXECUTION
 * ============================================================================
 *
 * Audit metadata may survive:
 *
 *     retry;
 *     recovery;
 *     rerouting;
 *     rescheduling;
 *     backend migration;
 *     checkpoint restoration;
 *     recompilation;
 *     failover.
 *
 * Mandatory audit requirements must survive lowering and resilience
 * transformations.
 *
 * If a target cannot preserve a mandatory requirement, downstream analysis
 * must reject the realization or select another valid realization.
 */


/*
 * ============================================================================
 * 30. COMPATIBILITY
 * ============================================================================
 *
 * The canonical production spelling is:
 *
 *     audit
 *     trace
 *     evidence
 *     verify
 *     decision
 *     provenance
 *
 * Compatibility aliases MUST be handled by:
 *
 *     grammar/compatibility/
 *
 * rather than by adding duplicate parser productions here.
 *
 * Deprecated syntax must remain explicitly classified in the compatibility
 * specification before removal.
 */


/*
 * ============================================================================
 * 31. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics MUST identify:
 *
 *     - missing audit name;
 *     - missing audit body;
 *     - empty audit body;
 *     - malformed requirement;
 *     - malformed target;
 *     - malformed trace;
 *     - malformed evidence;
 *     - malformed verification;
 *     - malformed decision;
 *     - malformed provenance reference;
 *     - malformed constraint;
 *     - malformed preference;
 *     - malformed hint;
 *     - malformed property;
 *     - missing expression;
 *     - missing semicolon;
 *     - malformed attachment.
 *
 * Semantic diagnostics belong downstream and include:
 *
 *     - unresolved audit target;
 *     - unresolved security reference;
 *     - unresolved provenance reference;
 *     - unsatisfied capability;
 *     - unsatisfied resource requirement;
 *     - incompatible policy;
 *     - invalid audit/provenance relation;
 *     - invalid security effect;
 *     - unsupported target realization;
 *     - inability to preserve mandatory audit intent.
 *
 * All diagnostics MUST retain source spans.
 */


/*
 * ============================================================================
 * 32. POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The following forms MUST parse:
 *
 *     audit execution_audit {
 *         requires security::audit;
 *         target security::execution;
 *         trace security::decision;
 *         evidence security::attestation;
 *         verify security::integrity;
 *         decision security::decision_record;
 *         provenance compile::result;
 *     }
 *
 *
 *     audit quantum_audit {
 *         target quantum::measurement;
 *         trace quantum::decision;
 *         requires capability("quantum.audit");
 *         provenance quantum::execution;
 *     }
 *
 *
 *     audit distributed_audit {
 *         target distributed::execution;
 *         trace distributed::message;
 *         evidence distributed::provenance;
 *         prefer storage::durable;
 *         hint execution::streaming;
 *     }
 *
 *
 *     audit execution_audit {
 *         constraint security::integrity;
 *         prefer security::durable_audit;
 *         hint execution::reproducible;
 *         custom::property = security::value;
 *     }
 *
 *
 *     audit(result) {
 *         trace security::decision;
 *         provenance compile::result;
 *     };
 *
 *
 *     audit result;
 *
 *
 *     audit security::decision;
 */


/*
 * ============================================================================
 * 33. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The following MUST be rejected:
 *
 *     audit;
 *
 *     audit execution_audit {}
 *
 *     audit {
 *     }
 *
 *     audit execution_audit {
 *         requires;
 *     }
 *
 *     audit execution_audit {
 *         target;
 *     }
 *
 *     audit execution_audit {
 *         trace;
 *     }
 *
 *     audit execution_audit {
 *         evidence;
 *     }
 *
 *     audit execution_audit {
 *         verify;
 *     }
 *
 *     audit execution_audit {
 *         decision;
 *     }
 *
 *     audit execution_audit {
 *         provenance;
 *     }
 *
 *     audit execution_audit {
 *         custom::property;
 *     }
 *
 *     audit(result);
 *
 * The distinction is intentional:
 *
 *     audit;
 *
 * is not a valid reusable declaration or statement because the lightweight
 * statement requires its expression.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 34. BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test at least:
 *
 *     - one audit member;
 *     - many audit members;
 *     - many declarations;
 *     - deeply qualified references;
 *     - nested expressions;
 *     - generic expressions;
 *     - function calls;
 *     - quantum references;
 *     - hardware references;
 *     - distributed references;
 *     - AI/learning references;
 *     - provenance references;
 *     - security policy references;
 *     - capability references;
 *     - resource references;
 *     - annotations;
 *     - visibility modifiers;
 *     - audit attachment;
 *     - lightweight audit statement.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 35. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Scaling the number of:
 *
 *     audit declarations;
 *     audit members;
 *     targets;
 *     evidence references;
 *     provenance references;
 *     trace references;
 *     properties;
 *
 * MUST NOT require:
 *
 *     a grammar modification;
 *     a new keyword;
 *     a new fixed-size production;
 *     a new hardware-specific production;
 *     a new target-specific token.
 *
 * Grammar repetition and ordinary expressions provide the syntactic
 * scalability mechanism.
 */


/*
 * ============================================================================
 * 36. DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Identical:
 *
 *     source
 *     token stream
 *     grammar version
 *     compatibility configuration
 *
 * MUST produce the same parse structure.
 *
 * Parsing MUST NOT inspect:
 *
 *     hardware;
 *     filesystem;
 *     network;
 *     runtime state;
 *     credentials;
 *     wall-clock time;
 *     randomness.
 */


/*
 * ============================================================================
 * 37. DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/lexer/tokens.g4
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/
 *     grammar/types/
 *     grammar/expressions/
 *
 * EXPORTS:
 *
 *     auditFile
 *     auditTopLevelConstruct
 *     securityAuditDeclaration
 *     auditBody
 *     auditMember
 *     auditRequirement
 *     auditTarget
 *     auditTrace
 *     auditEvidence
 *     auditVerification
 *     auditDecision
 *     auditProvenance
 *     auditConstraint
 *     auditPreference
 *     auditHint
 *     auditProperty
 *     auditAttachment
 *     auditStatement
 *     auditReference
 *     auditReferenceList
 *
 * CONSUMED_BY:
 *
 *     grammar/security/security.g4
 *     grammar/effects/security.g4
 *     grammar/security/sandbox.g4 where audit intent is consumed
 *     grammar/statements/ where audit statements are composed
 *     grammar/antlr/ZamaniParser.g4
 *
 * AST_OWNER:
 *
 *     existing domain-neutral frontend AST
 *
 * SEMANTIC_OWNER:
 *
 *     security/audit semantic analysis
 *
 * EFFECT_OWNER:
 *
 *     grammar/effects/
 *
 * CAPABILITY_OWNER:
 *
 *     grammar/core/capabilities.g4
 *     grammar/resources/
 *     grammar/security/capabilities.g4
 *
 * RESOURCE_OWNER:
 *
 *     grammar/resources/
 *
 * POLICY_OWNER:
 *
 *     grammar/policies/
 *     security policy subsystem
 *
 * PROVENANCE_OWNER:
 *
 *     grammar/data/provenance.g4
 *     grammar/security/provenance.g4
 *
 * IR_OWNER:
 *
 *     canonical semantic IR
 *     classical IR
 *     quantum::ir where audit metadata accompanies quantum semantics
 *
 * SPEC_OWNER:
 *
 *     grammar/specification/security.md
 *     grammar/spec/provenance.md
 *     grammar/spec/policies.md
 *     grammar/spec/effects.md
 *
 * TEST_OWNER:
 *
 *     grammar/tests/security/audit/
 *
 * COMPATIBILITY_OWNER:
 *
 *     grammar/compatibility/
 */


/*
 * ============================================================================
 * 38. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] it is the sole parser-level owner of security audit declarations;
 *     [ ] it is the sole parser-level owner of audit statements;
 *     [ ] it uses the canonical ZamaniLexer vocabulary;
 *     [ ] it defines no lexical tokens;
 *     [ ] it does not duplicate generic expression syntax;
 *     [ ] it does not duplicate type syntax;
 *     [ ] it does not duplicate provenance syntax;
 *     [ ] it does not duplicate policy syntax;
 *     [ ] it does not duplicate authorization syntax;
 *     [ ] it does not duplicate capability syntax;
 *     [ ] it does not duplicate trust syntax;
 *     [ ] audit declarations require a name and nonempty body;
 *     [ ] audit members are explicit and deterministic;
 *     [ ] audit properties remain open-world;
 *     [ ] arbitrary audit member counts are supported;
 *     [ ] no finite audit capacity is encoded;
 *     [ ] no hardware capacity is encoded;
 *     [ ] no secret material is accepted as a special audit construct;
 *     [ ] no runtime enforcement is performed;
 *     [ ] no audit persistence is performed;
 *     [ ] no target selection is performed;
 *     [ ] quantum integration terminates at semantic quantum::ir;
 *     [ ] HDL integration remains downstream;
 *     [ ] effects remain owned by effects/;
 *     [ ] resources remain owned by resources/;
 *     [ ] policies remain owned by the policy subsystem;
 *     [ ] provenance remains owned by provenance subsystems;
 *     [ ] source spans remain available;
 *     [ ] positive tests exist;
 *     [ ] negative tests exist;
 *     [ ] boundary tests exist;
 *     [ ] scalability tests exist;
 *     [ ] determinism tests exist;
 *     [ ] compatibility tests exist;
 *     [ ] generated Rust requires no unsafe;
 *     [ ] Rust 1.97+ compatibility is maintained.
 *
 * ============================================================================
 */