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
 * PRODUCTION-READY DATA PROVENANCE / LINEAGE GRAMMAR
 *
 * PURPOSE
 * -------
 * This grammar owns DATA-DOMAIN provenance declarations and lineage metadata.
 *
 * It describes the logical origin, derivation, transformation, production,
 * consumption, identity metadata, version metadata, schema association,
 * evidence association, and extensible lineage properties of data values.
 *
 * This grammar is intentionally OPEN-WORLD.
 *
 * It does not enumerate every possible provenance system, storage provider,
 * tracing system, database, ledger, cloud provider, vendor, hash algorithm,
 * signature algorithm, hardware target, quantum device, or future computing
 * technology.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 *
 *   - data provenance declarations;
 *   - data provenance members;
 *   - data lineage relationships;
 *   - data origin metadata;
 *   - data derivation metadata;
 *   - data transformation metadata;
 *   - producer/consumer metadata;
 *   - provenance metadata properties;
 *   - provenance requirements;
 *   - provenance constraints;
 *   - provenance preferences;
 *   - provenance hints;
 *   - provenance schema references;
 *   - provenance evidence references;
 *   - data provenance attachment syntax.
 *
 * THIS FILE DOES NOT OWN
 *
 *   - ordinary expressions;
 *   - expression-level provenance invocation;
 *   - types;
 *   - schemas;
 *   - serialization;
 *   - persistence;
 *   - cryptography;
 *   - authentication;
 *   - authorization;
 *   - audit implementation;
 *   - distributed tracing implementation;
 *   - runtime event collection;
 *   - AI semantics;
 *   - quantum semantics;
 *   - HDL semantics;
 *   - hardware realization;
 *   - resource discovery;
 *   - scheduling;
 *   - canonical IR definitions.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * Expression-level provenance:
 *
 *     provenance(...)
 *
 * is owned exclusively by:
 *
 *     grammar/expressions/provenance.g4
 *
 * This file MUST NOT define another provenanceExpression rule.
 *
 * Data provenance declarations are owned here.
 *
 * Therefore:
 *
 *     expression provenance
 *          -> expressions/provenance.g4
 *
 *     data provenance declaration
 *          -> data/provenance.g4
 *
 * Both normalize downstream into the common semantic provenance model.
 *
 * ============================================================================
 * REPOSITORY INTEGRATION
 * ============================================================================
 *
 * DATA FRONTEND:
 *
 *     grammar/data/data.g4
 *          |
 *          +--> dataProvenanceConstruct
 *                  |
 *                  v
 *              provenance.g4
 *
 * EXPRESSION FRONTEND:
 *
 *     grammar/expressions/provenance.g4
 *          |
 *          v
 *     provenance(...)
 *
 * SEMANTIC:
 *
 *     data provenance
 *          |
 *          v
 *     common semantic provenance model
 *
 * CANONICAL IR:
 *
 *     semantic provenance
 *          |
 *          +--> canonical data/semantic IR
 *          |
 *          +--> applicable domain IR
 *
 * Quantum data continues to lower through:
 *
 *     quantum::ir
 *
 * This grammar MUST NOT introduce a provenance-specific quantum IR.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Provenance describes logical meaning and lineage.
 *
 * It does not describe the finite capacity of today's machines.
 *
 * No grammar-level limits exist for:
 *
 *   - provenance records;
 *   - lineage relations;
 *   - sources;
 *   - producers;
 *   - consumers;
 *   - transformations;
 *   - evidence items;
 *   - data objects;
 *   - distributed participants;
 *   - quantum results;
 *   - tensor dimensions;
 *   - nodes;
 *   - devices;
 *   - memory;
 *   - threads;
 *   - processors;
 *   - accelerators.
 *
 * Repetition is represented through grammar repetition.
 *
 * Physical and implementation limits belong to resource negotiation and
 * execution, not to the source-language grammar.
 *
 * ============================================================================
 * OPEN-WORLD RULE
 * ============================================================================
 *
 * Provenance concepts that are not universal language primitives must normally
 * be represented by:
 *
 *   - qualified names;
 *   - ordinary expressions;
 *   - properties;
 *   - evidence values;
 *   - metadata;
 *   - dialect extensions.
 *
 * The grammar must therefore remain stable as new provenance technologies
 * appear.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Downstream AST construction must preserve the following logical structure:
 *
 *     DataProvenanceDeclaration
 *         name
 *         subject
 *         members
 *         source span
 *
 *     DataProvenanceRelation
 *         relation kind
 *         subject/value
 *         related expressions
 *         source span
 *
 *     DataProvenanceProperty
 *         qualified name
 *         value
 *         source span
 *
 *     DataProvenanceRequirement
 *         expression
 *         source span
 *
 *     DataProvenanceConstraint
 *         expression
 *         source span
 *
 *     DataProvenancePreference
 *         expression
 *         source span
 *
 *     DataProvenanceHint
 *         expression
 *         source span
 *
 * The AST must remain domain-neutral.
 *
 * It must not contain:
 *
 *   - database connections;
 *   - network clients;
 *   - filesystem handles;
 *   - cryptographic objects;
 *   - physical device handles;
 *   - quantum hardware state;
 *   - scheduler state;
 *   - runtime pointers.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *   - resolving the provenance subject;
 *   - resolving referenced values;
 *   - determining relation direction;
 *   - validating relation meaning;
 *   - checking schema compatibility;
 *   - checking version semantics;
 *   - validating evidence references;
 *   - validating requirements;
 *   - validating constraints;
 *   - validating policies;
 *   - validating capabilities;
 *   - preserving provenance through transformations.
 *
 * The parser establishes structure only.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Declaring provenance does not inherently perform runtime I/O.
 *
 * Runtime recording/querying may introduce effects such as:
 *
 *   provenance
 *   audit
 *   storage
 *   network
 *
 * Those effects are determined by semantic analysis and execution policy.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Provenance may require capabilities expressed through ordinary Zamani
 * requirement expressions, for example:
 *
 *     requires capability("provenance.record");
 *
 *     requires capability("provenance.verify");
 *
 *     requires capability("provenance.integrity");
 *
 * The grammar does not enumerate the capability universe.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Provenance introduces no fixed resource requirements.
 *
 * Resource requirements remain ordinary semantic expressions.
 *
 * Any implementation budget is external to the language's universal grammar.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Provenance may be constrained by policies through ordinary expressions and
 * policy semantics.
 *
 * This grammar does not implement policy evaluation.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Provenance itself is part of the semantic history of a value.
 *
 * Transformations of provenance metadata must remain distinguishable from
 * transformations of the data value being described.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *   - source text;
 *   - grammar version;
 *   - lexer version;
 *   - parser configuration;
 *   - explicitly selected dialect configuration.
 *
 * Parsing must not depend on:
 *
 *   - hardware;
 *   - wall-clock time;
 *   - randomness;
 *   - filesystem state;
 *   - network state;
 *   - runtime state.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust actions.
 *
 * Generated parser integration must remain compatible with:
 *
 *   Rust 1.97+
 *   Rust 2021+
 *
 * Zamani implementation code must not require unsafe Rust.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * GRAMMAR HEADER
 * ============================================================================
 */

parser grammar provenance;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC DATA PROVENANCE UNIT
 * ============================================================================
 *
 * This rule is intended for isolated grammar conformance testing.
 *
 * The complete Zamani parser must enter through its canonical root rather than
 * through this rule.
 * ============================================================================
 */

provenanceUnit
    : dataProvenanceConstruct* EOF
    ;


/*
 * ============================================================================
 * PUBLIC INTEGRATION RULE
 * ============================================================================
 *
 * data.g4 must consume this rule rather than reproduce provenance syntax.
 * ============================================================================
 */

dataProvenanceConstruct
    : dataProvenanceDeclaration
    | dataProvenanceAttach
    ;


/*
 * ============================================================================
 * PROVENANCE DECLARATION
 * ============================================================================
 *
 * Canonical structure:
 *
 *     provenance name {
 *         source input;
 *         derivation transformation;
 *         generated producer;
 *         verified evidence;
 *         property = value;
 *         requires capability("...");
 *     }
 *
 * The declaration name identifies the provenance declaration itself.
 *
 * The subject may be introduced through a source/metadata member rather than
 * through a hardware-specific identity.
 * ============================================================================
 */

dataProvenanceDeclaration
    : PROVENANCE IDENTIFIER
      LBRACE
      dataProvenanceMember*
      RBRACE
    ;


/*
 * ============================================================================
 * PROVENANCE ATTACHMENT
 * ============================================================================
 *
 * Attaches data provenance metadata to an existing expression.
 *
 * Example:
 *
 *     provenance result {
 *         source input;
 *     };
 *
 * The expression-level form:
 *
 *     provenance(result)
 *
 * remains owned by expressions/provenance.g4.
 *
 * ============================================================================
 */

dataProvenanceAttach
    : PROVENANCE expression
      LBRACE
      dataProvenanceMember*
      RBRACE
      SEMICOLON?
    ;


/*
 * ============================================================================
 * MEMBERS
 * ============================================================================
 */

dataProvenanceMember
    : dataProvenanceSource
    | dataProvenanceDerivation
    | dataProvenanceGenerated
    | dataProvenanceTransformed
    | dataProvenanceVerified
    | dataProvenanceDecision
    | dataProvenanceEvidence
    | dataProvenanceProperty
    | dataProvenanceSchema
    | dataProvenanceContract
    | dataProvenanceAnnotation
    ;


/*
 * ============================================================================
 * SOURCE
 * ============================================================================
 *
 * Identifies a logical source of the data.
 *
 * It is not a physical address.
 *
 * It may refer to:
 *
 *   - a value;
 *   - a dataset;
 *   - a computation;
 *   - a module;
 *   - a service;
 *   - a model;
 *   - another logical artifact.
 * ============================================================================
 */

dataProvenanceSource
    : SOURCE
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * DERIVATION
 * ============================================================================
 *
 * Describes logical derivation information.
 *
 * The referenced expression may identify the source, transformation, or
 * derivation artifact.
 *
 * ============================================================================
 */

dataProvenanceDerivation
    : DERIVATION
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * GENERATED
 * ============================================================================
 *
 * Identifies a logical producer or generation event.
 *
 * No physical process, CPU, node, device, or scheduler identity is implied.
 * ============================================================================
 */

dataProvenanceGenerated
    : GENERATED
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * TRANSFORMED
 * ============================================================================
 *
 * Describes a transformation associated with the data.
 *
 * The transformation itself remains owned by the transformation subsystem.
 * ============================================================================
 */

dataProvenanceTransformed
    : TRANSFORMED
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * VERIFIED
 * ============================================================================
 *
 * Associates verification evidence or a verification result.
 *
 * Verification semantics remain downstream.
 * ============================================================================
 */

dataProvenanceVerified
    : VERIFIED
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * DECISION
 * ============================================================================
 *
 * Associates a decision record or decision value with the data lineage.
 *
 * The decision model is not redefined here.
 * ============================================================================
 */

dataProvenanceDecision
    : DECISION
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * EVIDENCE
 * ============================================================================
 *
 * Associates an evidence value with the provenance record.
 *
 * Evidence syntax/semantics remain owned by the canonical evidence subsystem.
 * ============================================================================
 */

dataProvenanceEvidence
    : EVIDENCE
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * SCHEMA
 * ============================================================================
 *
 * Associates the logical data with a schema reference.
 *
 * This does not redefine schema syntax.
 * ============================================================================
 */

dataProvenanceSchema
    : SCHEMA
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * EXTENSIBLE PROPERTY
 * ============================================================================
 *
 * Property names are open-world qualified names.
 *
 * This avoids a finite grammar catalogue of provenance concepts.
 *
 * Examples:
 *
 *     source_system = system;
 *     retention_policy = policy;
 *     integrity = capability("provenance.integrity");
 *     custom::property = value;
 *
 * ============================================================================
 */

dataProvenanceProperty
    : qualifiedName
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * CONTRACT / REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * These are expressed using the repository's universal semantic vocabulary.
 *
 * The grammar does not create a second contract system.
 * ============================================================================
 */

dataProvenanceContract
    : REQUIRES
      expression
      SEMICOLON
    | CONSTRAINT
      expression
      SEMICOLON
    | PREFER
      expression
      SEMICOLON
    | HINT
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * ANNOTATIONS
 * ============================================================================
 *
 * Existing annotation ownership remains elsewhere.
 *
 * This rule is deliberately a compatibility boundary.
 * ============================================================================
 */

dataProvenanceAnnotation
    : annotation
    ;


/*
 * ============================================================================
 * SEMANTIC NORMALIZATION CONTRACT
 * ============================================================================
 *
 * The parser-level spellings above normalize to the common provenance model.
 *
 * Conceptual normalized relations:
 *
 *     SOURCE
 *     DERIVATION
 *     GENERATED
 *     TRANSFORMED
 *     VERIFIED
 *     DECISION
 *     EVIDENCE
 *     PROPERTY
 *     SCHEMA
 *
 * This is not a parser-level closed enumeration.
 *
 * Additional provenance relationships should normally use properties or
 * qualified semantic extensions instead of requiring a new universal keyword.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * DATA
 *
 *     data/provenance.g4
 *          |
 *          v
 *     semantic provenance
 *
 * AI
 *
 *     ai/provenance.g4
 *          |
 *          v
 *     semantic provenance
 *
 * QUANTUM
 *
 *     quantum provenance
 *          |
 *          v
 *     semantic provenance
 *          |
 *          v
 *     quantum::ir
 *
 * HDL / HARDWARE
 *
 *     hardware artifact provenance
 *          |
 *          v
 *     semantic provenance
 *
 * DISTRIBUTED
 *
 *     distributed lineage
 *          |
 *          v
 *     semantic provenance
 *
 * All domains therefore share one provenance semantic model.
 *
 * No domain creates a competing provenance IR.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SERIALIZATION BOUNDARY
 * ============================================================================
 *
 * Provenance values may later be serialized.
 *
 * Serialization format ownership remains outside this grammar.
 *
 * This file does not define JSON, XML, binary, database, ledger, or vendor
 * serialization syntax.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * PERSISTENCE BOUNDARY
 * ============================================================================
 *
 * Provenance may describe persistent data.
 *
 * Persistence determines:
 *
 *     retention
 *     storage
 *     recovery
 *
 * Provenance determines:
 *
 *     logical lineage
 *     origin
 *     derivation
 *     transformation history
 *
 * The two concerns remain separate.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * This grammar may carry expressions requiring integrity, authentication,
 * authorization, or audit capabilities.
 *
 * It does not implement any of them.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * These constructs deliberately use:
 *
 *     *
 *
 * for arbitrary member counts.
 *
 * There is no grammar-level maximum for:
 *
 *     sources
 *     transformations
 *     producers
 *     consumers
 *     evidence
 *     decisions
 *     properties
 *     provenance declarations
 *
 * Actual parser/compiler resource exhaustion is handled through implementation
 * resource management and diagnostics, not through artificial language
 * ceilings.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains no universal capacity constants.
 *
 * It does not encode:
 *
 *     processor counts
 *     GPU counts
 *     FPGA counts
 *     QPU counts
 *     qubit counts
 *     node counts
 *     device counts
 *     memory limits
 *     thread limits
 *     tensor limits
 *     network limits
 *     provenance record limits
 *     lineage depth limits
 *
 * A target's finite capacity remains target/resource information.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DETERMINISM AUDIT
 * ============================================================================
 *
 * Parsing does not depend on:
 *
 *     hardware
 *     runtime state
 *     network state
 *     filesystem state
 *     current time
 *     randomness
 *
 * The same source and grammar configuration must produce the same syntactic
 * structure.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics must cover:
 *
 *     missing provenance name
 *     missing provenance body
 *     missing member expression
 *     malformed property
 *     malformed schema reference
 *     missing semicolon
 *     malformed attachment
 *
 * Semantic diagnostics belong downstream and include:
 *
 *     invalid provenance subject
 *     invalid lineage relation
 *     incompatible schema
 *     invalid evidence
 *     unsatisfied provenance capability
 *     incompatible provenance policy
 *     invalid provenance requirement
 *
 * Diagnostics must retain source spans.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE:
 *
 *     provenance result {
 *         source input;
 *         derivation transform;
 *         generated compute;
 *         transformed normalize;
 *         verified evidence_record;
 *         evidence evidence_record;
 *         decision decision_record;
 *         schema data::Schema;
 *         custom::property = value;
 *     }
 *
 *     provenance result {
 *         requires capability("provenance.record");
 *         constraint policy::retention;
 *         prefer storage::durable;
 *         hint execution::batched;
 *     }
 *
 *     provenance result {
 *         source input_a;
 *         source input_b;
 *         source input_c;
 *     }
 *
 * NEGATIVE:
 *
 *     provenance {
 *     }
 *
 *     provenance result {
 *         custom::property;
 *     }
 *
 *     provenance result {
 *         source;
 *     }
 *
 *     provenance result {
 *         schema;
 *     }
 *
 * BOUNDARY:
 *
 *     one member
 *     many members
 *     nested expressions
 *     qualified names
 *     arbitrary property names
 *     large declaration bodies
 *     distributed data references
 *     quantum result references
 *     tensor result references
 *     hardware artifact references
 *
 * CROSS-DOMAIN:
 *
 *     classical result provenance
 *     quantum measurement provenance
 *     hybrid computation provenance
 *     tensor provenance
 *     AI inference provenance
 *     HDL artifact provenance
 *     distributed lineage
 *
 * DETERMINISM:
 *
 *     identical source + identical grammar configuration
 *         -> identical parse structure
 *
 * SCALABILITY:
 *
 *     increasing logical provenance graph size must not require a grammar
 *     change or a new fixed language constant.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/lexer/tokens.g4
 *     grammar/antlr/ZamaniLexer.g4
 *     canonical expression grammar
 *     canonical qualified-name grammar
 *     canonical annotation grammar
 *
 * EXPORTS:
 *
 *     provenanceUnit
 *     dataProvenanceConstruct
 *     dataProvenanceDeclaration
 *     dataProvenanceAttach
 *
 * CONSUMED_BY:
 *
 *     grammar/data/data.g4
 *     canonical parser composition
 *     data conformance tests
 *
 * AST_OWNER:
 *
 *     existing domain-neutral frontend AST
 *
 * SEMANTIC_OWNER:
 *
 *     semantic data/provenance analysis
 *
 * IR_OWNER:
 *
 *     canonical semantic/data IR
 *
 *     quantum::ir where provenance accompanies quantum semantics
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/provenance.md
 *     grammar/specification/ provenance specification
 *
 * TEST_OWNER:
 *
 *     grammar/tests/provenance/
 *
 * COMPATIBILITY_OWNER:
 *
 *     grammar/compatibility/
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *   [x] data provenance has one grammar owner;
 *   [x] expression provenance remains owned by expressions/provenance.g4;
 *   [x] no duplicate provenanceExpression rule exists here;
 *   [x] no private expression language exists here;
 *   [x] existing expression rules are reused;
 *   [x] existing qualified names are reused;
 *   [x] existing annotations are reused;
 *   [x] current canonical lexer vocabulary is used;
 *   [x] no missing invented lexer tokens are referenced;
 *   [x] provenance relations remain extensible;
 *   [x] no universal hardware limits are encoded;
 *   [x] arbitrary member counts are supported;
 *   [x] requirements remain separate from preferences and hints;
 *   [x] provenance does not implement persistence;
 *   [x] provenance does not implement serialization;
 *   [x] provenance does not implement cryptography;
 *   [x] provenance does not implement networking;
 *   [x] provenance does not implement runtime tracing;
 *   [x] provenance does not introduce another IR;
 *   [x] quantum provenance reaches quantum::ir downstream;
 *   [x] Rust integration requires no unsafe;
 *   [x] Rust 1.97+ compatibility is preserved;
 *   [x] positive tests are defined;
 *   [x] negative tests are defined;
 *   [x] boundary tests are defined;
 *   [x] cross-domain tests are defined;
 *   [x] scalability tests are defined;
 *   [x] determinism tests are defined.
 *
 * ============================================================================
 */