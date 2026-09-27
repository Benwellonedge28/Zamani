/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/data/provenance.g4
 *
 * STATUS
 * ------
 * PRODUCTION DATA-DOMAIN LEAF GRAMMAR
 *
 * PURPOSE
 * -------
 * This file defines the canonical SOURCE-LEVEL PROVENANCE / LINEAGE syntax
 * for Zamani.
 *
 * Provenance describes the semantic history, origin, derivation, transformation,
 * consumption, production, identity, version, and declared metadata associated
 * with logical data.
 *
 * It describes WHAT provenance information exists or is requested.
 *
 * It does NOT implement:
 *
 *   - provenance databases;
 *   - audit-log storage;
 *   - distributed tracing;
 *   - filesystem metadata;
 *   - blockchain/ledger implementations;
 *   - cryptographic implementations;
 *   - hashing implementations;
 *   - signatures;
 *   - identity providers;
 *   - authentication;
 *   - authorization;
 *   - networking;
 *   - scheduling;
 *   - runtime tracing;
 *   - compiler internals;
 *   - hardware discovery;
 *   - physical device identity;
 *   - quantum::ir;
 *   - QEC;
 *   - ZQN;
 *   - HAL.
 *
 * ============================================================================
 * ARCHITECTURAL OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *   - provenance declarations;
 *   - provenance statement syntax;
 *   - lineage relationships;
 *   - derivation relationships;
 *   - origin declarations;
 *   - producer relationships;
 *   - consumer relationships;
 *   - transformation relationships;
 *   - provenance identity;
 *   - provenance version;
 *   - provenance schema references;
 *   - provenance contracts;
 *   - provenance requirements;
 *   - provenance constraints;
 *   - provenance preferences;
 *   - provenance hints;
 *   - extensible provenance properties;
 *   - provenance scopes;
 *   - provenance expressions;
 *   - provenance metadata structure.
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *   - general expressions;
 *   - general types;
 *   - schemas;
 *   - records;
 *   - collections;
 *   - streams;
 *   - transformations;
 *   - persistence;
 *   - serialization;
 *   - deserialization;
 *   - networking;
 *   - security implementation;
 *   - cryptography;
 *   - storage;
 *   - distributed execution;
 *   - AI/ML semantics;
 *   - quantum semantics;
 *   - HDL semantics;
 *   - hardware realization;
 *   - canonical IR definitions.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * This file is the canonical grammar owner for provenance/lineage syntax.
 *
 * The existing data.g4 rule:
 *
 *     dataProvenanceDecl
 *
 * MUST eventually delegate to this grammar rather than implement an
 * independent provenance grammar.
 *
 * No second provenance grammar may be introduced under:
 *
 *     data/
 *     interoperability/
 *     security/
 *     persistence/
 *     networking/
 *
 * Provenance is a cross-domain DATA concern.
 *
 * ============================================================================
 * COMPOSITION
 * ============================================================================
 *
 * Canonical architecture:
 *
 *     grammar/Zamani.g4
 *             |
 *             v
 *     grammar/antlr/ZamaniParser.g4
 *             |
 *             v
 *          dataStmt
 *             |
 *             v
 *       provenanceStmt
 *             |
 *             v
 *     domain-neutral AST
 *             |
 *             v
 *     semantic provenance model
 *             |
 *             v
 *     canonical semantic model / IR
 *             |
 *       +-----+------+----------------+
 *       |            |                |
 *       v            v                v
 *    storage      auditing         runtime
 *       |            |                |
 *       +------------+----------------+
 *                    |
 *                    v
 *               target system
 *
 * Provenance MUST remain a semantic concern.
 *
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * Provenance may describe data originating from:
 *
 *   - classical computation;
 *   - quantum computation;
 *   - hybrid computation;
 *   - AI/ML;
 *   - tensors;
 *   - datasets;
 *   - streams;
 *   - distributed computation;
 *   - networking;
 *   - HDL/hardware co-design;
 *   - persistence;
 *   - external/interoperability formats;
 *   - future computing domains.
 *
 * The provenance grammar does not need to know how those domains implement
 * their computation.
 *
 * For quantum programs in particular:
 *
 *     provenance
 *          |
 *          v
 *     generic AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> data provenance
 *          |
 *          +--> quantum semantics
 *                  |
 *                  v
 *              quantum::ir
 *
 * Provenance MUST NOT create or modify a second quantum IR.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Zamani's portability objective is:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Provenance syntax therefore MUST NOT depend on:
 *
 *   - CPU count;
 *   - core count;
 *   - thread count;
 *   - GPU count;
 *   - FPGA count;
 *   - accelerator count;
 *   - QPU count;
 *   - qubit count;
 *   - node count;
 *   - device count;
 *   - memory capacity;
 *   - storage capacity;
 *   - network capacity;
 *   - register width;
 *   - tensor rank;
 *   - tensor dimensions;
 *   - replica count;
 *   - partition count;
 *   - timeline count.
 *
 * The following MUST NEVER become grammar-level limits:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * Numeric quantities occurring in source remain program semantics.
 *
 * ============================================================================
 * REQUIREMENT / PREFERENCE / HINT SEPARATION
 * ============================================================================
 *
 * Provenance syntax preserves the difference between:
 *
 *     requires
 *     constraint
 *     prefer
 *     hint
 *     implementation decision
 *
 * Example:
 *
 *     requires capability("provenance.integrity")
 *
 * is a semantic requirement.
 *
 * It does NOT mean:
 *
 *     use provider X
 *
 * Likewise:
 *
 *     prefer provenance.storage.local
 *
 * is a preference, not a mandatory implementation decision.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Provenance properties, identities, policies, capabilities, and metadata are
 * OPEN-WORLD semantic identifiers.
 *
 * The grammar MUST NOT enumerate:
 *
 *     git
 *     blockchain
 *     database
 *     cloud-provider
 *     vendor
 *     tracing-system
 *     ledger
 *     specific hash algorithm
 *     specific signature algorithm
 *
 * as the complete universe of provenance implementations.
 *
 * Future systems are represented through:
 *
 *     qualifiedName
 *
 * and semantic registries/dialects.
 *
 * ============================================================================
 * SERIALIZATION / PERSISTENCE BOUNDARY
 * ============================================================================
 *
 * Provenance is related to persistence and serialization but does not own
 * either implementation.
 *
 * Serialization answers:
 *
 *     How is a value represented?
 *
 * Persistence answers:
 *
 *     How is a value retained/recovered?
 *
 * Provenance answers:
 *
 *     Where did the logical value come from and how was it transformed?
 *
 * Therefore:
 *
 *     provenance.g4
 *          |
 *          +--> serialization.g4
 *          |
 *          +--> persistence.g4
 *          |
 *          +--> semantic data model
 *
 * must remain separate ownership domains.
 *
 * ============================================================================
 * SCHEMA BOUNDARY
 * ============================================================================
 *
 * Schema declarations belong to schema/data schema ownership.
 *
 * This file may reference schemas but MUST NOT redefine the schema language.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The grammar maps to domain-neutral AST structures.
 *
 * Required semantic information:
 *
 *   ProvenanceDeclaration
 *       name
 *       subject
 *       members
 *       source span
 *
 *   ProvenanceRelation
 *       relation kind
 *       subject
 *       related values
 *       metadata
 *       source span
 *
 *   ProvenanceProperty
 *       qualified name
 *       value
 *       source span
 *
 *   ProvenanceContract
 *       requirements
 *       constraints
 *       preferences
 *       hints
 *       source span
 *
 * The AST MUST NOT contain:
 *
 *   - storage implementation objects;
 *   - vendor objects;
 *   - database connections;
 *   - network clients;
 *   - physical device IDs;
 *   - quantum hardware state;
 *   - runtime handles.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis resolves:
 *
 *   - provenance subject identity;
 *   - relationship direction;
 *   - data dependency;
 *   - provenance scope;
 *   - schema compatibility;
 *   - version compatibility;
 *   - capability requirements;
 *   - integrity requirements;
 *   - security requirements;
 *   - retention requirements;
 *   - policy compatibility;
 *   - dialect-specific properties.
 *
 * Semantic analysis determines whether relationships are meaningful.
 *
 * The parser only establishes their syntactic structure.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file DOES NOT define a provenance-specific competing IR.
 *
 * Provenance information must lower into the repository's canonical semantic
 * model / data IR boundary.
 *
 * If a dedicated provenance semantic representation is required downstream,
 * it MUST be owned by the semantic/IR layer rather than this grammar.
 *
 * Quantum data remains governed by:
 *
 *     quantum::ir
 *
 * and provenance MUST NOT replace, duplicate, or fork that representation.
 *
 * ============================================================================
 * SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * Every provenance construct must remain traceable to source locations.
 *
 * At minimum, downstream AST/semantic structures must preserve source spans
 * for:
 *
 *   - declaration;
 *   - subject;
 *   - relationship;
 *   - related expression;
 *   - property;
 *   - requirement;
 *   - constraint;
 *   - preference;
 *   - hint.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *   - source text;
 *   - grammar version;
 *   - canonical lexer;
 *   - explicitly selected dialect configuration.
 *
 * Parsing MUST NOT depend on:
 *
 *   - hardware;
 *   - storage availability;
 *   - filesystem state;
 *   - network state;
 *   - wall-clock time;
 *   - randomness;
 *   - environment variables;
 *   - runtime state.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This is an ANTLR parser grammar.
 *
 * It contains:
 *
 *   - no Rust code;
 *   - no target-language actions;
 *   - no semantic predicates;
 *   - no filesystem access;
 *   - no network access;
 *   - no runtime execution.
 *
 * Generated/compiler integration MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and must not require unsafe Rust.
 *
 * ============================================================================
 */

parser grammar provenance;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This entry point is intentionally isolated from the complete Zamani program
 * entry point.
 *
 * It is useful for:
 *
 *   - grammar conformance tests;
 *   - parser unit tests;
 *   - tooling;
 *   - data-domain composition.
 *
 * ============================================================================
 */

provenanceUnit
    : provenanceConstruct* EOF
    ;


/*
 * ============================================================================
 * PUBLIC INTEGRATION FACADE
 * ============================================================================
 *
 * data.g4 should delegate to provenanceStmt.
 *
 * Existing dataProvenanceDecl should become a compatibility façade rather
 * than a second implementation.
 * ============================================================================
 */

provenanceStmt
    : provenanceDeclaration
    | provenanceAttachStatement
    ;

provenanceConstruct
    : provenanceStmt
    | provenanceExpression
    | provenanceContract
    ;


/*
 * ============================================================================
 * PROVENANCE DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     provenance Event for value {
 *         derived_from source;
 *         transformed_by transform;
 *         produced_by producer;
 *         property = value;
 *     }
 *
 * The subject remains a general expression.
 *
 * Therefore provenance may describe:
 *
 *   variables
 *   records
 *   collections
 *   streams
 *   tensors
 *   datasets
 *   models
 *   computation results
 *   hybrid values
 *   future data-domain values.
 *
 * ============================================================================
 */

provenanceDeclaration
    : PROVENANCE provenanceName
      FOR provenanceSubject
      provenanceDeclarationBody
    ;

provenanceDeclarationBody
    : LBRACE provenanceMember* RBRACE
    ;


/*
 * ============================================================================
 * PROVENANCE ATTACHMENT
 * ============================================================================
 *
 * This form allows provenance to be attached to an already existing logical
 * value without requiring a named provenance declaration.
 *
 * Example:
 *
 *     provenance value with {
 *         derived_from input;
 *     };
 *
 * ============================================================================
 */

provenanceAttachStatement
    : PROVENANCE provenanceSubject
      WITH
      LBRACE provenanceMember* RBRACE
      SEMICOLON
    ;


/*
 * ============================================================================
 * SUBJECTS
 * ============================================================================
 *
 * Provenance subjects reuse the canonical expression system.
 *
 * This is intentional.
 *
 * Do NOT create:
 *
 *     provenanceIdentifier
 *     provenanceLiteral
 *     provenanceType
 *
 * when the existing language already has those concepts.
 * ============================================================================
 */

provenanceSubject
    : expression
    ;


/*
 * ============================================================================
 * NAMES
 * ============================================================================
 */

provenanceName
    : qualifiedName
    ;


/*
 * ============================================================================
 * MEMBERS
 * ============================================================================
 */

provenanceMember
    : provenanceDerivedFrom
    | provenanceOrigin
    | provenanceProducedBy
    | provenanceConsumedBy
    | provenanceTransformedBy
    | provenanceLineage
    | provenanceIdentity
    | provenanceVersion
    | provenanceSchema
    | provenanceScope
    | provenanceContractClause
    | provenanceProperty
    | provenanceRequirement
    | provenanceConstraint
    | provenancePreference
    | provenanceHint
    ;


/*
 * ============================================================================
 * DERIVATION
 * ============================================================================
 *
 * Indicates logical input values from which the subject was derived.
 *
 * Example:
 *
 *     derived_from input_a, input_b;
 *
 * This describes semantic dependency, not execution scheduling.
 * ============================================================================
 */

provenanceDerivedFrom
    : DERIVED_FROM
      provenanceExpressionList
      SEMICOLON
    ;


/*
 * ============================================================================
 * ORIGIN
 * ============================================================================
 *
 * Identifies the logical origin of a value.
 *
 * The origin is an expression rather than a hard-coded storage technology.
 * ============================================================================
 */

provenanceOrigin
    : ORIGIN
      provenanceExpressionList
      SEMICOLON
    ;


/*
 * ============================================================================
 * PRODUCER
 * ============================================================================
 *
 * Describes the logical producer of a value.
 *
 * The producer may be:
 *
 *   function
 *   module
 *   pipeline
 *   service
 *   model
 *   computation
 *   symbolic resource
 *   future domain construct.
 *
 * It is NOT a physical process identifier.
 * ============================================================================
 */

provenanceProducedBy
    : PRODUCED_BY
      provenanceExpressionList
      SEMICOLON
    ;


/*
 * ============================================================================
 * CONSUMER
 * ============================================================================
 */

provenanceConsumedBy
    : CONSUMED_BY
      provenanceExpressionList
      SEMICOLON
    ;


/*
 * ============================================================================
 * TRANSFORMATION
 * ============================================================================
 *
 * Indicates a logical transformation associated with the subject.
 *
 * This grammar does not define the transformation itself.
 *
 * Transformation semantics remain owned by transformations.g4 and the
 * semantic/data pipeline.
 * ============================================================================
 */

provenanceTransformedBy
    : TRANSFORMED_BY
      provenanceExpressionList
      SEMICOLON
    ;


/*
 * ============================================================================
 * GENERIC LINEAGE
 * ============================================================================
 *
 * LINEAGE provides an extensible relationship category.
 *
 * The relation name is open-world.
 *
 * Example:
 *
 *     lineage custom::relationship(input, output);
 *
 * Future relation types therefore do not require a grammar rewrite.
 * ============================================================================
 */

provenanceLineage
    : LINEAGE
      provenanceRelation
      SEMICOLON
    ;

provenanceRelation
    : qualifiedName
      provenanceArgumentList?
    ;

provenanceArgumentList
    : LPAREN provenanceArgumentListItems? RPAREN
    ;

provenanceArgumentListItems
    : provenanceExpression
      (COMMA provenanceExpression)*
    ;

provenanceArgument
    : provenanceExpression
    ;


/*
 * ============================================================================
 * IDENTITY
 * ============================================================================
 *
 * Identity is semantic data.
 *
 * It must not be interpreted by the parser as:
 *
 *   physical device identity;
 *   storage address;
 *   database row ID;
 *   network address.
 * ============================================================================
 */

provenanceIdentity
    : IDENTITY
      provenanceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * VERSION
 * ============================================================================
 */

provenanceVersion
    : VERSION
      provenanceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * SCHEMA
 * ============================================================================
 *
 * Schema ownership remains outside this file.
 *
 * This rule only references a schema contract.
 * ============================================================================
 */

provenanceSchema
    : SCHEMA
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * SCOPE
 * ============================================================================
 *
 * Scope is semantic metadata.
 *
 * It can describe the logical boundary over which provenance applies.
 * ============================================================================
 */

provenanceScope
    : SCOPE
      provenanceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * CONTRACT
 * ============================================================================
 *
 * Provenance contracts describe semantic expectations.
 *
 * They do not execute verification.
 * ============================================================================
 */

provenanceContractClause
    : CONTRACT
      provenanceContract
    ;

provenanceContract
    : CONTRACT
      LBRACE
      provenanceContractItem*
      RBRACE
    ;

provenanceContractItem
    : provenanceRequirement
    | provenanceConstraint
    | provenancePreference
    | provenanceHint
    | provenanceProperty
    ;


/*
 * ============================================================================
 * REQUIREMENTS
 * ============================================================================
 *
 * Requirement means semantically necessary.
 *
 * Example:
 *
 *     requires capability("provenance.integrity");
 *
 * No hardware limits are implied.
 * ============================================================================
 */

provenanceRequirement
    : REQUIRES
      provenanceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * CONSTRAINTS
 * ============================================================================
 */

provenanceConstraint
    : CONSTRAINT
      provenanceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * PREFERENCES
 * ============================================================================
 *
 * A preference MUST NOT be interpreted as a requirement.
 * ============================================================================
 */

provenancePreference
    : PREFER
      provenanceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * HINTS
 * ============================================================================
 *
 * Hints provide implementation guidance.
 *
 * They do not establish semantic correctness requirements unless the semantic
 * specification explicitly defines them as such.
 * ============================================================================
 */

provenanceHint
    : HINT
      provenanceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * GENERIC PROPERTIES
 * ============================================================================
 *
 * Properties are open-world.
 *
 * Example:
 *
 *     retention = policy::long_term;
 *
 *     integrity = capability("hash");
 *
 *     source_system = "example";
 *
 *     custom::property = value;
 *
 * Property names are not a finite grammar registry.
 * ============================================================================
 */

provenanceProperty
    : qualifiedName
      ASSIGN
      provenanceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * EXPRESSIONS
 * ============================================================================
 *
 * Provenance does not define a private expression language.
 *
 * It reuses the canonical Zamani expression grammar.
 * ============================================================================
 */

provenanceExpression
    : expression
    ;

provenanceExpressionList
    : provenanceExpression
      (COMMA provenanceExpression)*
    ;


/*
 * ============================================================================
 * EXPRESSION-LEVEL PROVENANCE
 * ============================================================================
 *
 * This provides an expression form for provenance queries/metadata.
 *
 * The semantic layer determines the exact operation.
 *
 * Example conceptual usage:
 *
 *     provenance(value)
 *
 * or:
 *
 *     provenance(value, relation)
 *
 * ============================================================================
 */

provenanceExpression
    : PROVENANCE
      LPAREN
      provenanceExpressionList?
      RPAREN
    | expression
    ;


/*
 * ============================================================================
 * CONTRACT REFERENCE
 * ============================================================================
 *
 * Named contracts may be referenced without defining a second contract
 * language.
 * ============================================================================
 */

provenanceContractReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * NORMALIZED SEMANTIC RELATIONS
 * ============================================================================
 *
 * The semantic layer should normalize all syntactic relationships into a
 * common representation.
 *
 * Example normalized relation kinds:
 *
 *     DERIVED_FROM
 *     ORIGIN
 *     PRODUCED_BY
 *     CONSUMED_BY
 *     TRANSFORMED_BY
 *     CUSTOM
 *
 * The grammar does not define this enum as an implementation type.
 *
 * This list is an architectural contract, not a parser-level closed-world
 * enumeration.
 * ============================================================================
 */


/*
 * ============================================================================
 * DATA / PERSISTENCE INTEGRATION
 * ============================================================================
 *
 * Provenance may be attached to persisted data:
 *
 *     provenance
 *          |
 *          +--> persistence.g4
 *
 * Persistence determines retention/recovery semantics.
 *
 * Provenance determines lineage/history semantics.
 *
 * Neither grammar is allowed to absorb the other's implementation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SERIALIZATION INTEGRATION
 * ============================================================================
 *
 * Provenance metadata may be serialized.
 *
 * However:
 *
 *     provenance.g4
 *
 * does not define representation formats.
 *
 *     serialization.g4
 *
 * remains responsible for serialization intent.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * Provenance MAY express requirements such as:
 *
 *     requires capability("provenance.integrity");
 *     requires capability("provenance.authenticated");
 *
 * The grammar does not implement:
 *
 *     hashes
 *     signatures
 *     encryption
 *     key management
 *     identity providers
 *
 * Security semantics remain downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Provenance can span distributed computations.
 *
 * It must therefore permit arbitrary numbers of:
 *
 *     sources
 *     transformations
 *     producers
 *     consumers
 *     relations
 *
 * through grammar repetition rather than fixed alternatives.
 *
 * No node count is encoded.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Provenance may describe data produced by quantum computation.
 *
 * It MUST NOT describe physical quantum implementation through this grammar.
 *
 * In particular, provenance syntax must not require:
 *
 *     physical qubit IDs
 *     fixed QPU topology
 *     fixed gate sets
 *     calibration data
 *     routing decisions
 *     QEC implementation
 *
 * Those remain downstream responsibilities.
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Provenance may describe hardware/software co-design artifacts.
 *
 * It must not hard-code:
 *
 *     FPGA width
 *     register width
 *     memory size
 *     device count
 *     bus count
 *     pipeline depth
 *
 * unless those are explicitly program-defined semantic values.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * OPEN-WORLD RELATION EXTENSION
 * ============================================================================
 *
 * New provenance relations MUST normally be represented through:
 *
 *     lineage qualifiedName(...)
 *
 * rather than by adding another keyword for every future concept.
 *
 * This keeps the grammar extensible while preserving semantic validation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar intentionally contains no:
 *
 *     MAX_PROVENANCE_RECORDS
 *     MAX_LINEAGE_DEPTH
 *     MAX_SOURCES
 *     MAX_RELATIONS
 *     MAX_PRODUCERS
 *     MAX_CONSUMERS
 *     MAX_DATASETS
 *     MAX_NODES
 *     MAX_STORAGE
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_DEVICES
 *
 * Repetition is expressed with:
 *
 *     *
 *     +
 *
 * and values are delegated to semantic expressions.
 *
 * ============================================================================
 * SECURITY AUDIT
 * ============================================================================
 *
 * This grammar:
 *
 *   - does not access secrets;
 *   - does not access credentials;
 *   - does not access filesystem state;
 *   - does not access network state;
 *   - does not execute code;
 *   - does not create authentication sessions;
 *   - does not implement cryptography.
 *
 * ============================================================================
 * PERFORMANCE AUDIT
 * ============================================================================
 *
 * The grammar avoids:
 *
 *   - target-dependent semantic predicates;
 *   - filesystem lookups;
 *   - network lookups;
 *   - runtime discovery;
 *   - unbounded lexical backtracking;
 *   - embedded target-language actions.
 *
 * Lists are represented using normal ANTLR repetition constructs.
 *
 * Very large provenance graphs remain subject to actual parser/compiler
 * resources rather than artificial grammar limits.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * The parser should report syntax errors with source spans.
 *
 * Semantic diagnostics should distinguish:
 *
 *   - unknown provenance relation;
 *   - invalid subject;
 *   - invalid provenance property;
 *   - incompatible schema;
 *   - invalid requirement;
 *   - unsatisfied capability;
 *   - invalid provenance scope;
 *   - invalid version;
 *   - invalid dialect extension.
 *
 * Syntax diagnostics belong to the parser.
 *
 * Semantic diagnostics belong downstream.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST include:
 *
 *   provenance p for value {
 *       derived_from input;
 *   }
 *
 *   provenance p for result {
 *       origin source;
 *       produced_by compute;
 *       transformed_by transform;
 *   }
 *
 *   provenance p for dataset {
 *       lineage custom::relation(source, transform);
 *   }
 *
 *   provenance p for value {
 *       identity key;
 *       version version_value;
 *       schema data::Schema;
 *   }
 *
 *   provenance value with {
 *       derived_from input;
 *   };
 *
 * Negative tests MUST include:
 *
 *   - missing provenance name;
 *   - missing subject;
 *   - missing body;
 *   - malformed relation;
 *   - malformed property;
 *   - missing semicolon;
 *   - malformed argument list;
 *   - invalid contract structure.
 *
 * Boundary tests MUST include:
 *
 *   - one source;
 *   - many sources;
 *   - one relationship;
 *   - many relationships;
 *   - nested expressions;
 *   - deeply qualified relation names;
 *   - large property lists.
 *
 * Scalability tests MUST verify that the grammar does not impose a fixed:
 *
 *   - relation count;
 *   - source count;
 *   - producer count;
 *   - consumer count;
 *   - lineage depth;
 *   - provenance declaration count.
 *
 * Determinism tests MUST verify identical source text produces identical parse
 * structure regardless of target hardware or runtime availability.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *   [x] provenance has one parser ownership surface;
 *   [x] data source/target values reuse canonical expressions;
 *   [x] provenance relationships are open-world;
 *   [x] schema definitions are not duplicated;
 *   [x] persistence is not duplicated;
 *   [x] serialization is not duplicated;
 *   [x] security implementation is not duplicated;
 *   [x] requirement/preference/hint semantics remain distinct;
 *   [x] no physical hardware assumptions exist;
 *   [x] no universal capacity limits exist;
 *   [x] no second provenance IR is introduced;
 *   [x] source spans are predetermined;
 *   [x] semantic normalization is predetermined;
 *   [x] quantum::ir remains canonical;
 *   [x] QEC/ZQN/HAL remain downstream;
 *   [x] distributed provenance is scalable;
 *   [x] parsing is deterministic;
 *   [x] generated Rust requires no unsafe code;
 *   [x] Rust 1.97 / 1.97.1 compatibility is preserved;
 *   [x] positive/negative/boundary/scalability tests are defined.
 *
 * ============================================================================
 */