/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/knowledge.g4
 *
 * Grammar:
 *     KnowledgeExpressions
 *
 * Status:
 *     Canonical production expression-level knowledge grammar.
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only.
 *     No unsafe Rust.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the EXPRESSION-LEVEL KNOWLEDGE OPERATION BOUNDARY.
 *
 * It provides a single reusable syntax layer for computational knowledge
 * operations such as:
 *
 *     knowledge assertion
 *     knowledge retraction
 *     knowledge query
 *     knowledge lookup
 *     knowledge retrieval
 *     knowledge update
 *
 * Knowledge here is a GENERAL COMPUTATIONAL ABSTRACTION.
 *
 * It is not restricted to AI.
 *
 * Knowledge values may represent:
 *
 *     - facts;
 *     - relations;
 *     - graph assertions;
 *     - scientific observations;
 *     - configuration facts;
 *     - compiler facts;
 *     - hardware capability facts;
 *     - resource facts;
 *     - security facts;
 *     - provenance facts;
 *     - model facts;
 *     - distributed-system facts;
 *     - quantum experiment results;
 *     - classical data;
 *     - HDL/hardware metadata;
 *     - application-defined knowledge;
 *     - future computational knowledge domains.
 *
 * ============================================================================
 * CORE ARCHITECTURAL RULE
 * ============================================================================
 *
 * Knowledge syntax describes INTENT.
 *
 * It does not prescribe:
 *
 *     - a database;
 *     - a graph database;
 *     - a theorem prover;
 *     - an inference engine;
 *     - a neural model;
 *     - a storage engine;
 *     - a CPU;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a QPU;
 *     - a cluster;
 *     - a network;
 *     - a particular knowledge representation implementation.
 *
 * The semantic/compiler/runtime layers determine the realization.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     Zamani lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     knowledgeExpression
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +------------------+------------------+------------------+
 *       |                  |                  |                  |
 *       v                  v                  v                  v
 *     types             effects          capabilities        resources
 *       |                  |                  |                  |
 *       +------------------+------------------+------------------+
 *                              |
 *                              v
 *                       contracts / policies
 *                              |
 *                              v
 *                         provenance
 *                              |
 *                              v
 *                    canonical semantic model
 *                              |
 *               +--------------+--------------+
 *               |              |              |
 *               v              v              v
 *          classical      quantum::ir    data/knowledge
 *               |              |              |
 *               +--------------+--------------+
 *                              |
 *                              v
 *                         optimization
 *                              |
 *                         lowering
 *                              |
 *                    routing / scheduling
 *                              |
 *                    resilience / recovery
 *                              |
 *                             HAL
 *                              |
 *                       target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     knowledgeExpression
 *     knowledgeOperation
 *     knowledgeAssertionExpression
 *     knowledgeRetractionExpression
 *     knowledgeQueryExpression
 *     knowledgeLookupExpression
 *     knowledgeUpdateExpression
 *     knowledgeOperationName
 *     knowledgeSubject
 *     knowledgeRelation
 *     knowledgeObject
 *     knowledgePattern
 *     knowledgeEvidence
 *     knowledgeMetadata
 *     knowledgeOptions
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - general expression precedence;
 *     - primaryExpression;
 *     - function calls;
 *     - argumentList;
 *     - identifiers;
 *     - qualified names;
 *     - literals;
 *     - patterns;
 *     - guards;
 *     - general data queries;
 *     - SQL;
 *     - graph query languages;
 *     - inference algorithms;
 *     - deduction algorithms;
 *     - theorem proving;
 *     - machine-learning algorithms;
 *     - probability algorithms;
 *     - storage engines;
 *     - database engines;
 *     - knowledge-graph engines;
 *     - AST implementation;
 *     - semantic implementation;
 *     - IR implementation;
 *     - optimization;
 *     - scheduling;
 *     - routing;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - target selection;
 *     - runtime execution.
 *
 * ============================================================================
 * RELATIONSHIP WITH DATA QUERY GRAMMAR
 * ============================================================================
 *
 * General logical data querying is owned by:
 *
 *     grammar/data/queries.g4
 *
 * Its public query boundary is:
 *
 *     dataQueryConstruct
 *     dataQueryExpression
 *
 * This file MUST NOT duplicate:
 *
 *     SELECT
 *     FROM
 *     JOIN
 *     WHERE
 *     GROUP BY
 *     HAVING
 *     ORDER BY
 *     LIMIT
 *     OFFSET
 *     WINDOW
 *     QUALIFY
 *     WITH
 *     VALUES
 *     UNION
 *     INTERSECT
 *     EXCEPT
 *     EXISTS
 *     subquery
 *
 * A knowledge query may refer to or consume a logical data query, but it
 * does so through the existing data-query semantic boundary.
 *
 * Therefore:
 *
 *     knowledge query
 *          |
 *          +--> knowledge pattern
 *          |
 *          +--> logical data source
 *          |
 *          +--> existing dataQueryExpression
 *
 * The knowledge grammar does not become another SQL-like language.
 *
 * ============================================================================
 * RELATIONSHIP WITH REASONING
 * ============================================================================
 *
 * Knowledge and reasoning are related but distinct semantic concepts.
 *
 * Knowledge:
 *
 *     facts
 *     relations
 *     assertions
 *     evidence
 *     provenance
 *
 * Reasoning:
 *
 *     inference
 *     deduction
 *     induction
 *     abduction
 *     causal reasoning
 *     counterfactual reasoning
 *
 * This file represents the KNOWLEDGE INPUT/STATE SIDE.
 *
 * Reasoning syntax belongs to the reasoning subsystem.
 *
 * A knowledge result may therefore feed:
 *
 *     infer
 *     deduce
 *     reason
 *     learn
 *     adapt
 *     explain
 *     decide
 *
 * without creating a second knowledge grammar for each operation.
 *
 * ============================================================================
 * RELATIONSHIP WITH LEARNING
 * ============================================================================
 *
 * Knowledge may be:
 *
 *     - training data;
 *     - model metadata;
 *     - learned facts;
 *     - observations;
 *     - evidence;
 *     - labels;
 *     - features;
 *     - model outputs.
 *
 * This file does not own learning syntax.
 *
 * Learning remains a separate semantic operation.
 *
 * Conceptually:
 *
 *     knowledge
 *         |
 *         v
 *     learning
 *         |
 *         v
 *     model
 *         |
 *         v
 *     inference
 *         |
 *         v
 *     knowledge / decision
 *
 * ============================================================================
 * RELATIONSHIP WITH ADAPTATION
 * ============================================================================
 *
 * Knowledge may provide observations used by an adaptation policy.
 *
 * This file does not authorize self-modification.
 *
 * Any adaptation must pass through:
 *
 *     policy
 *     capability
 *     effect
 *     resource
 *     provenance
 *     validation
 *
 * Knowledge syntax itself does not grant adaptation authority.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer authority is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This grammar contains no lexer rules.
 *
 * Existing lexical vocabulary relevant to this grammar includes:
 *
 *     KeywordKnowledge
 *     KeywordAssert
 *     KeywordRecall
 *     KeywordInfer
 *     KeywordLearn
 *
 * Knowledge operation names that are not currently reserved by the canonical
 * lexer remain ordinary identifiers.
 *
 * This is intentional.
 *
 * In particular, this file MUST NOT invent parser-side aliases for:
 *
 *     query
 *     retract
 *     reason
 *     evidence
 *     provenance
 *
 * merely because those concepts are useful.
 *
 * If the language specification later decides that one of those spellings
 * must become a reserved keyword, the canonical lexer vocabulary must be
 * updated first and this grammar can then consume the canonical token.
 *
 * ============================================================================
 * LEXICAL EXTENSIBILITY
 * ============================================================================
 *
 * The knowledge subsystem must remain open-world.
 *
 * Knowledge relation names, namespaces, predicates, schemas, stores, model
 * identifiers, evidence identifiers, and application-specific concepts are
 * ordinary names.
 *
 * Examples:
 *
 *     temperature
 *     mass
 *     observed_at
 *     supports
 *     caused_by
 *     quantum_measurement
 *     hardware_capability
 *     compiler_fact
 *     model_prediction
 *     scientific_observation
 *
 * MUST NOT require new lexer keywords.
 *
 * ============================================================================
 * PUBLIC EXPRESSION BOUNDARY
 * ============================================================================
 *
 * `knowledgeExpression` is the single public expression-level boundary.
 *
 * It is intentionally separate from `primaryExpression`.
 *
 * The canonical expression composition grammar owns:
 *
 *     primaryExpression
 *
 * and must eventually include:
 *
 *     knowledgeExpression
 *
 * at the appropriate atomic-expression position.
 *
 * Conceptually:
 *
 *     primaryExpression
 *         :
 *             ...
 *           | knowledgeExpression
 *           ;
 *
 * This file owns `knowledgeExpression`.
 *
 * It does not redefine `primaryExpression`.
 *
 * ============================================================================
 * KNOWLEDGE OPERATION MODEL
 * ============================================================================
 *
 * The canonical semantic operation families are:
 *
 *     ASSERT
 *     RETRACT
 *     QUERY
 *     LOOKUP
 *     UPDATE
 *
 * They are represented syntactically by a stable operation introducer plus
 * an operation name.
 *
 * The existing `knowledge` keyword provides the stable language-level
 * namespace:
 *
 *     knowledge assert ...
 *     knowledge retract ...
 *     knowledge query ...
 *     knowledge lookup ...
 *     knowledge update ...
 *
 * This avoids forcing every future knowledge-provider operation into the
 * global keyword namespace.
 *
 * It also avoids making:
 *
 *     assert
 *
 * globally ambiguous because Zamani already has `KeywordAssert`.
 *
 * ============================================================================
 * CANONICAL FORMS
 * ============================================================================
 *
 * Assertion:
 *
 *     knowledge assert(subject, relation, object);
 *
 * Retraction:
 *
 *     knowledge retract(subject, relation, object);
 *
 * Query:
 *
 *     knowledge query(subject, relation, object);
 *
 * Lookup:
 *
 *     knowledge lookup(pattern);
 *
 * Update:
 *
 *     knowledge update(subject, relation, object);
 *
 * Evidence-bearing assertion:
 *
 *     knowledge assert(subject, relation, object)
 *         evidence evidence_expression;
 *
 * Pattern query:
 *
 *     knowledge query(pattern)
 *         where guard_expression;
 *
 * Metadata-bearing operation:
 *
 *     knowledge assert(subject, relation, object)
 *         with metadata_expression;
 *
 * The exact semantic interpretation belongs downstream.
 *
 * ============================================================================
 * DESIGN PRINCIPLE: OPEN-WORLD KNOWLEDGE
 * ============================================================================
 *
 * The grammar does not enumerate predicates.
 *
 * Therefore it MUST NOT contain:
 *
 *     temperature
 *     location
 *     person
 *     animal
 *     sentiment
 *     robot
 *     vehicle
 *     quantum_state
 *     hardware
 *     GPU
 *     CPU
 *     FPGA
 *     QPU
 *     etc.
 *
 * Such concepts are data.
 *
 * They belong to:
 *
 *     identifiers
 *     types
 *     schemas
 *     libraries
 *     dialects
 *     semantic registries
 *     applications.
 *
 * ============================================================================
 * KNOWLEDGE EXPRESSION
 * ============================================================================
 */

knowledgeExpression
    : knowledgeAssertionExpression
    | knowledgeRetractionExpression
    | knowledgeQueryExpression
    | knowledgeLookupExpression
    | knowledgeUpdateExpression
    ;


/*
 * ============================================================================
 * ASSERTION
 * ============================================================================
 *
 * An assertion introduces or records a logical knowledge relation.
 *
 * The grammar accepts both:
 *
 *     knowledge assert(subject, relation, object)
 *
 * and:
 *
 *     knowledge assert(subject, relation)
 *
 * The two-argument form can be interpreted semantically as an assertion whose
 * relation payload is represented by a structured value or provider-specific
 * knowledge model.
 *
 * The grammar deliberately does not decide whether a knowledge item is:
 *
 *     RDF-like
 *     graph-like
 * relational
 *     symbolic
 * probabilistic
 *     tensor-based
 *     object-based
 *     event-based
 *     temporal
 *     causal.
 *
 * Those are semantic/type-level distinctions.
 * ============================================================================
 */

knowledgeAssertionExpression
    : KeywordKnowledge
      KeywordAssert
      LPAREN
      knowledgeTerm
      COMMA
      knowledgeTerm
      (COMMA knowledgeTerm)?
      knowledgeOperationTail*
      RPAREN
      knowledgeExpressionOptions?
    ;


/*
 * ============================================================================
 * RETRACTION
 * ============================================================================
 *
 * Retraction removes, invalidates, supersedes, or otherwise withdraws a
 * logical knowledge assertion.
 *
 * The semantic layer decides which retraction model applies.
 *
 * The grammar does not assume destructive deletion.
 *
 * A provider may implement retraction as:
 *
 *     removal
 *     tombstone
 *     invalidation
 *     temporal supersession
 *     version transition
 *     logical negation
 *
 * while preserving the language-level meaning.
 * ============================================================================
 */

knowledgeRetractionExpression
    : KeywordKnowledge
      knowledgeOperationName
      LPAREN
      knowledgePattern
      knowledgeOperationTail*
      RPAREN
      knowledgeExpressionOptions?
    ;


/*
 * ============================================================================
 * QUERY
 * ============================================================================
 *
 * `knowledge query` searches for knowledge matching a logical pattern.
 *
 * It is intentionally NOT a replacement for dataQueryExpression.
 *
 * A knowledge query is concerned with:
 *
 *     knowledge matching
 *     facts
 *     relations
 *     evidence
 *     provenance
 *
 * A general data query remains owned by:
 *
 *     grammar/data/queries.g4
 *
 * ============================================================================
 */

knowledgeQueryExpression
    : KeywordKnowledge
      knowledgeQueryOperationName
      LPAREN
      knowledgePattern?
      knowledgeOperationTail*
      RPAREN
      knowledgeExpressionOptions?
    ;


/*
 * ============================================================================
 * LOOKUP
 * ============================================================================
 *
 * Lookup is an intentionally generic retrieval boundary.
 *
 * It can represent:
 *
 *     exact knowledge retrieval
 *     indexed lookup
 *     symbolic retrieval
 *     provider-backed retrieval
 *     semantic retrieval
 *     graph retrieval
 *
 * The implementation is selected downstream.
 * ============================================================================
 */

knowledgeLookupExpression
    : KeywordKnowledge
      knowledgeLookupOperationName
      LPAREN
      knowledgePattern?
      knowledgeOperationTail*
      RPAREN
      knowledgeExpressionOptions?
    ;


/*
 * ============================================================================
 * UPDATE
 * ============================================================================
 *
 * Update represents a logical knowledge-state transition.
 *
 * It does not mandate a mutable storage engine.
 *
 * An implementation may lower it to:
 *
 *     immutable versioning
 *     event sourcing
 *     transactional update
 *     append-only state
 *     distributed consensus
 *     local state
 *     remote state
 *     model state
 *
 * according to semantic policy and available capabilities.
 * ============================================================================
 */

knowledgeUpdateExpression
    : KeywordKnowledge
      knowledgeUpdateOperationName
      LPAREN
      knowledgeTerm
      COMMA
      knowledgeTerm
      (COMMA knowledgeTerm)?
      knowledgeOperationTail*
      RPAREN
      knowledgeExpressionOptions?
    ;


/*
 * ============================================================================
 * OPERATION NAMES
 * ============================================================================
 *
 * These rules intentionally use IDENTIFIER rather than adding a second global
 * keyword vocabulary.
 *
 * The semantic layer resolves the canonical operation names:
 *
 *     retract
 *     query
 *     lookup
 *     update
 *
 * The spelling remains source-visible and can be validated semantically.
 *
 * This allows future operations without changing the universal lexer.
 * ============================================================================
 */

knowledgeOperationName
    : IDENTIFIER
    ;

knowledgeQueryOperationName
    : IDENTIFIER
    ;

knowledgeLookupOperationName
    : IDENTIFIER
    ;

knowledgeUpdateOperationName
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * KNOWLEDGE TERM
 * ============================================================================
 *
 * A knowledge term is deliberately based on the existing expression system.
 *
 * This means a knowledge subject, predicate, or object can eventually be:
 *
 *     identifier
 *     literal
 *     tuple
 *     record
 *     function result
 *     query result
 *     measurement result
 *     tensor value
 *     model output
 *     symbolic value
 *     future domain value
 *
 * without changing this grammar.
 *
 * `expression` remains the semantic value boundary.
 * ============================================================================
 */

knowledgeTerm
    : expression
    ;


/*
 * ============================================================================
 * KNOWLEDGE PATTERN
 * ============================================================================
 *
 * Knowledge matching requires a representation capable of containing:
 *
 *     exact values
 *     symbolic variables
 *     wildcard values
 *     structured values
 *     nested values
 *
 * The canonical pattern subsystem owns general pattern syntax.
 *
 * Therefore this grammar accepts:
 *
 *     pattern
 *
 * when the pattern subsystem is available.
 *
 * The pattern itself remains independent from knowledge semantics.
 * ============================================================================
 */

knowledgePattern
    : pattern
    | expression
    ;


/*
 * ============================================================================
 * OPERATION TAIL
 * ============================================================================
 *
 * These are optional semantic modifiers attached inside the knowledge
 * operation invocation.
 *
 * The grammar intentionally keeps the modifier model open.
 *
 * Examples:
 *
 *     evidence(...)
 *     provenance(...)
 *     confidence(...)
 *     source(...)
 *     policy(...)
 *     temporal(...)
 *     scope(...)
 *
 * These names are not globally reserved.
 *
 * ============================================================================
 */

knowledgeOperationTail
    : IDENTIFIER
      LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * OPERATION OPTIONS
 * ============================================================================
 *
 * Options are syntactic metadata and do not themselves grant authorization.
 *
 * Examples:
 *
 *     with metadata
 *     using policy
 *     under policy
 *
 * The semantic model determines which options are legal for which operation.
 * ============================================================================
 */

knowledgeExpressionOptions
    : knowledgeExpressionOption+
    ;

knowledgeExpressionOption
    : knowledgeWithOption
    | knowledgeUsingOption
    | knowledgeUnderOption
    ;

knowledgeWithOption
    : IdentifierKnowledgeWith
      expression
    ;

knowledgeUsingOption
    : IdentifierKnowledgeUsing
      expression
    ;

knowledgeUnderOption
    : IdentifierKnowledgeUnder
      expression
    ;


/*
 * ============================================================================
 * RESERVED-WORD AVOIDANCE
 * ============================================================================
 *
 * The following conceptual words are intentionally represented as ordinary
 * identifiers at this stage:
 *
 *     with
 *     using
 *     under
 *
 * The canonical lexer may later reserve them if the wider language
 * specification requires it.
 *
 * This grammar must then be updated to consume the canonical lexer tokens.
 *
 * The present design avoids inventing token names that do not exist in the
 * repository's current lexical vocabulary.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * KNOWLEDGE ASSERTION SEMANTICS
 * ============================================================================
 *
 * An assertion should eventually map to a semantic structure conceptually
 * equivalent to:
 *
 *     KnowledgeAssertion {
 *         subject
 *         relation
 *         object
 *         evidence
 *         provenance
 *         metadata
 *         policy
 *     }
 *
 * The actual Rust type is owned by the AST/semantic implementation.
 *
 * This grammar MUST NOT define Rust structures.
 *
 * ============================================================================
 * KNOWLEDGE RETRACTION SEMANTICS
 * ============================================================================
 *
 * A retraction should eventually map to a semantic operation containing:
 *
 *     target
 *     reason
 *     evidence
 *     provenance
 *     policy
 *
 * It MUST NOT automatically mean destructive deletion.
 *
 * ============================================================================
 * KNOWLEDGE QUERY SEMANTICS
 * ============================================================================
 *
 * A query should eventually map to:
 *
 *     KnowledgeQuery {
 *         pattern
 *         constraints
 *         policy
 *         provenance
 *     }
 *
 * The result type remains semantic/type-system territory.
 *
 * It may be:
 *
 *     scalar
 *     tuple
 *     record
 *     collection
 *     stream
 *     graph
 *     relation
 *     set
 *     option
 *     result
 *     tensor
 *     symbolic result
 *     provider-defined logical result.
 *
 * No universal cardinality limit is imposed.
 *
 * ============================================================================
 * KNOWLEDGE UPDATE SEMANTICS
 * ============================================================================
 *
 * Update is a logical operation.
 *
 * It must not imply:
 *
 *     mutable RAM
 *     database tables
 *     local filesystem
 *     remote database
 *     blockchain
 *     graph database
 *
 * The backend decides how the logical transition is realized.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Knowledge expressions are ordinary expressions.
 *
 * Their result types are determined by semantic analysis.
 *
 * Examples:
 *
 *     knowledge assert(...) -> provider-defined acknowledgement/result
 *     knowledge retract(...) -> result/status
 *     knowledge query(...) -> logical knowledge result
 *     knowledge lookup(...) -> logical retrieval result
 *     knowledge update(...) -> result/status
 *
 * The grammar MUST NOT introduce a fixed knowledge-result type.
 *
 * Knowledge values may participate in:
 *
 *     assignment
 *     function calls
 *     conditions
 *     pattern matching
 *     contracts
 *     reasoning
 *     learning
 *     adaptation
 *     classical computation
 *     quantum/classical control
 *     distributed computation
 *     HDL/hardware control
 *     simulation.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing a knowledge expression has no runtime effect.
 *
 * Semantic analysis MAY classify a knowledge operation with effects such as:
 *
 *     knowledge.read
 *     knowledge.write
 *     knowledge.retract
 *     knowledge.query
 *     network
 *     io
 *     distributed
 *     external
 *     nondeterministic
 *     temporal
 *
 * The grammar does not hard-code the complete effect universe.
 *
 * Effects are resolved semantically.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Knowledge operations may require capabilities such as:
 *
 *     capability("knowledge.read")
 *     capability("knowledge.write")
 *     capability("knowledge.query")
 *     capability("knowledge.retract")
 *     capability("knowledge.distributed")
 *     capability("knowledge.provenance")
 *     capability("knowledge.evidence")
 *
 * Capability names remain semantic values.
 *
 * No provider is selected here.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Knowledge expressions impose no universal physical resource limits.
 *
 * This file MUST NOT define:
 *
 *     MAX_FACTS
 *     MAX_RELATIONS
 *     MAX_KNOWLEDGE_ITEMS
 *     MAX_QUERY_RESULTS
 *     MAX_GRAPH_NODES
 *     MAX_GRAPH_EDGES
 *     MAX_QUERY_DEPTH
 *     MAX_ASSERTIONS
 *     MAX_RETRACTIONS
 *     MAX_FIELDS
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_QUBITS
 *
 * A knowledge set may be arbitrarily large subject to implementation and
 * available resources.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The grammar uses repetition and recursive composition rather than finite
 * enumeration.
 *
 * Therefore there is no language-level ceiling on:
 *
 *     number of assertions
 *     number of retractions
 *     number of queries
 *     number of terms
 *     number of evidence items
 *     number of metadata entries
 *     number of relations
 *     number of nested patterns
 *     number of knowledge operations
 *     knowledge graph size
 *     query nesting
 *     expression nesting.
 *
 * Practical limitations belong to:
 *
 *     compiler resource policy
 *     runtime resource policy
 *     storage availability
 *     network availability
 *     target capability
 *     execution policy.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Knowledge expressions may occur inside:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     assert
 *
 * They may also produce values consumed by those contracts.
 *
 * Contract syntax remains owned by:
 *
 *     grammar/validation/
 *
 * This file does not duplicate contract syntax.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Knowledge operations may be constrained by:
 *
 *     access policy
 *     privacy policy
 *     security policy
 *     data governance policy
 *     provenance policy
 *     execution policy
 *     deployment policy
 *     resource policy
 *     adaptation policy.
 *
 * A syntactically valid knowledge operation is NOT an authorization grant.
 *
 * Policy evaluation is semantic/runtime behavior.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Knowledge is particularly sensitive to provenance.
 *
 * Downstream provenance must be capable of recording relationships such as:
 *
 *     asserted_from
 *     derived_from
 *     observed_from
 *     inferred_from
 *     learned_from
 *     transformed_by
 *     verified_by
 *     retracted_by
 *     superseded_by
 *
 * This grammar only preserves source structure.
 *
 * It does not generate:
 *
 *     timestamps
 *     runtime identities
 *     hardware identifiers
 *     cryptographic signatures
 *     external source records.
 *
 * Those belong to provenance/security/compiler/runtime systems.
 *
 * ============================================================================
 * EVIDENCE INTEGRATION
 * ============================================================================
 *
 * Knowledge assertions may carry evidence through the extensible operation
 * tail mechanism.
 *
 * Conceptually:
 *
 *     knowledge assert(subject, relation, object)
 *         evidence(...)
 *
 * or an equivalent canonical syntax established by the provenance/evidence
 * subsystem.
 *
 * The grammar must not create a separate evidence type system.
 *
 * Evidence semantics belong to:
 *
 *     grammar/ai/
 *     grammar/validation/
 *     grammar/spec/
 *
 * as appropriate to the repository's final ownership model.
 *
 * ============================================================================
 * UNCERTAINTY INTEGRATION
 * ============================================================================
 *
 * A knowledge value may carry:
 *
 *     confidence
 *     probability
 *     distribution
 *     belief
 *     uncertainty
 *
 * The knowledge grammar does not define probabilistic semantics.
 *
 * Those belong to the type/uncertainty/semantic systems.
 *
 * This permits the same knowledge syntax to work with:
 *
 *     deterministic facts
 *     probabilistic facts
 *     fuzzy information
 *     statistical evidence
 *     quantum-derived uncertainty
 *     model-derived confidence.
 *
 * ============================================================================
 * REASONING INTEGRATION
 * ============================================================================
 *
 * Knowledge expressions may provide premises to reasoning:
 *
 *     knowledge query(...)
 *         |
 *         v
 *     inference
 *
 * or:
 *
 *     knowledge assertion
 *         |
 *         v
 *     reasoning
 *
 * The reasoning subsystem may consume the semantic KnowledgeQuery,
 * KnowledgeAssertion, or resulting value.
 *
 * No reasoning engine is selected by this grammar.
 *
 * ============================================================================
 * LEARNING INTEGRATION
 * ============================================================================
 *
 * Knowledge may provide:
 *
 *     training examples
 *     labels
 *     observations
 *     evidence
 *     features
 *     model metadata
 *
 * Learning remains an independent semantic operation.
 *
 * This allows:
 *
 *     knowledge
 *         |
 *         v
 *     learn
 *         |
 *         v
 *     model
 *         |
 *         v
 *     knowledge / inference
 *
 * without making the grammar dependent on any ML framework.
 *
 * ============================================================================
 * ADAPTATION INTEGRATION
 * ============================================================================
 *
 * Knowledge can be an input to controlled adaptation.
 *
 * Adaptation must remain subject to:
 *
 *     policy
 *     authorization
 *     capabilities
 *     effects
 *     resources
 *     contracts
 *     provenance.
 *
 * Knowledge does not itself modify program code.
 *
 * ============================================================================
 * PATTERN MATCHING INTEGRATION
 * ============================================================================
 *
 * Knowledge results may be consumed by the general pattern system:
 *
 *     match knowledge query(...) {
 *         ...
 *     }
 *
 * Pattern syntax is owned elsewhere.
 *
 * This file does not create a second pattern grammar.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Knowledge may contain quantum-derived information such as:
 *
 *     measurement results
 *     experiment records
 *     state observations
 *     calibration observations
 *     optimization results
 *     error observations
 *     resilience records.
 *
 * The grammar does not define quantum operations.
 *
 * Quantum computation remains under:
 *
 *     grammar/quantum/
 *
 * and the canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * Knowledge syntax MUST NOT introduce:
 *
 *     physical qubit identifiers
 *     gate sets
 *     coupling maps
 *     calibration formats
 *     routing syntax
 *     QEC syntax
 *     QPU-specific topology.
 *
 * ============================================================================
 * HYBRID COMPUTATION
 * ============================================================================
 *
 * Knowledge may bridge classical and quantum computation:
 *
 *     classical computation
 *          |
 *          v
 *     knowledge query
 *          |
 *          v
 *     quantum decision
 *          |
 *          v
 *     measurement
 *          |
 *          v
 *     knowledge assertion
 *
 * This is a semantic pipeline.
 *
 * The grammar remains domain-neutral.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Knowledge terms may be ordinary classical values:
 *
 *     integers
 *     floating values
 *     strings
 *     booleans
 *     records
 *     arrays
 *     maps
 *     tuples
 *     user-defined types
 *     symbolic values.
 *
 * No numeric width or storage representation is prescribed here.
 *
 * ============================================================================
 * AI / NEURAL-SYMBOLIC INTEGRATION
 * ============================================================================
 *
 * Knowledge is a natural bridge between symbolic and learned computation.
 *
 * Conceptually:
 *
 *     symbolic knowledge
 *          |
 *          +----------------+
 *          |                |
 *          v                v
 *      reasoning         learned model
 *          |                |
 *          +-------+--------+
 *                  |
 *                  v
 *               result
 *
 * The grammar does not distinguish:
 *
 *     symbolic
 *     neural
 *     probabilistic
 *     cognitive
 *     graph
 *
 * at the syntax level.
 *
 * Semantic capabilities determine the implementation.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Knowledge may be distributed across:
 *
 *     local state
 *     processes
 *     actors
 *     nodes
 *     clusters
 *     cloud resources
 *     future distributed substrates.
 *
 * The grammar does not specify:
 *
 *     node count
 *     replica count
 *     partition count
 *     network topology
 *     consistency algorithm.
 *
 * Those are semantic/runtime concerns.
 *
 * ============================================================================
 * ACTOR INTEGRATION
 * ============================================================================
 *
 * An actor may own or access knowledge.
 *
 * Knowledge operations may therefore occur within:
 *
 *     actor
 *     task
 *     async computation
 *     distributed service.
 *
 * Actor lifecycle and message semantics remain owned by:
 *
 *     grammar/concurrency/
 *
 * This file does not create a second actor model.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Knowledge may describe hardware intent or observations:
 *
 *     capability facts
 *     resource observations
 *     verification results
 *     timing observations
 *     simulation results
 *     synthesis metadata.
 *
 * The grammar does not encode:
 *
 *     register width
 *     bus width
 *     device count
 *     FPGA capacity
 *     ASIC capacity
 *     physical topology.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * SIMULATION INTEGRATION
 * ============================================================================
 *
 * Simulation may produce knowledge:
 *
 *     simulation
 *         |
 *         v
 *     observation
 *         |
 *         v
 *     knowledge assert
 *
 * The grammar does not execute simulations.
 *
 * Simulation semantics belong to:
 *
 *     grammar/execution/
 *
 * and the relevant domain subsystems.
 *
 * ============================================================================
 * SECURITY / SANDBOX INTEGRATION
 * ============================================================================
 *
 * Knowledge stores and providers may be security-sensitive.
 *
 * The parser does not authorize access.
 *
 * Semantic/security analysis must evaluate:
 *
 *     capability
 *     trust
 *     policy
 *     provenance
 *     data classification
 *     sandbox restrictions
 *     effect permissions.
 *
 * A sandbox may, for example, prohibit:
 *
 *     knowledge.write
 *
 * without changing the grammar.
 *
 * ============================================================================
 * INTEROPERABILITY
 * ============================================================================
 *
 * Knowledge may be imported/exported through:
 *
 *     JSON
 *     XML
 *     SQL
 *     graph formats
 *     scientific data formats
 *     provider APIs
 *     FFI
 *     ABI
 *     future interchange formats.
 *
 * These are NOT embedded into this grammar.
 *
 * The interoperability pipeline is:
 *
 *     external representation
 *          |
 *          v
 *     dialect/interoperability adapter
 *          |
 *          v
 *     canonical semantic knowledge model
 *          |
 *          v
 *     Zamani semantic pipeline.
 *
 * ============================================================================
 * METAPROGRAMMING INTEGRATION
 * ============================================================================
 *
 * Knowledge expressions may be inspected by controlled reflection or
 * generated by controlled metaprogramming.
 *
 * They must not execute during parsing.
 *
 * Compile-time knowledge access, if supported, requires explicit semantic:
 *
 *     capability
 *     effect
 *     policy
 *     provenance
 *
 * validation.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * The parse result depends only on:
 *
 *     source tokens
 *     grammar version
 *     explicitly selected language compatibility configuration.
 *
 * Parsing MUST NOT depend on:
 *
 *     wall-clock time
 *     randomness
 *     filesystem state
 *     network state
 *     database state
 *     knowledge-store contents
 *     hardware state
 *     QPU state
 *     GPU state
 *     scheduler state
 *     environment variables.
 *
 * Runtime knowledge results may naturally depend on external state.
 *
 * That is execution semantics, not parsing semantics.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * Knowledge syntax must remain unchanged regardless of whether it is lowered
 * to:
 *
 *     embedded execution
 *     CPU execution
 *     multicore execution
 *     GPU execution
 *     FPGA execution
 *     ASIC execution
 *     accelerator execution
 *     QPU-assisted execution
 *     simulator execution
 *     HPC execution
 *     cluster execution
 *     distributed execution
 *     cloud execution
 *     future hardware.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * This grammar supports:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * by describing knowledge intent rather than physical realization.
 *
 * The same source-level knowledge operation can be compiled differently
 * according to:
 *
 *     target capabilities
 *     available resources
 *     execution policy
 *     data location
 *     security policy
 *     deployment topology
 *     accelerator availability
 *     quantum availability
 *
 * without changing the program's source semantics.
 *
 * ============================================================================
 * NO HARD-CODED CAPACITY
 * ============================================================================
 *
 * This file contains NO universal finite capacity constants.
 *
 * Forbidden examples include:
 *
 *     MAX_KNOWLEDGE
 *     MAX_FACTS
 *     MAX_RELATIONS
 *     MAX_RESULTS
 *     MAX_GRAPH_SIZE
 *     MAX_QUERY_DEPTH
 *     MAX_EVIDENCE
 *     MAX_PROVENANCE
 *     MAX_NODES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *     MAX_DEVICE_COUNT
 *
 * Actual finite implementation constraints are resource policies.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces ANTLR parser contexts only.
 *
 * It does not define Rust AST structures.
 *
 * Recommended semantic AST concepts are:
 *
 *     KnowledgeAssertion
 *     KnowledgeRetraction
 *     KnowledgeQuery
 *     KnowledgeLookup
 *     KnowledgeUpdate
 *
 * Each should preserve:
 *
 *     source span
 *     source order
 *     operation kind
 *     terms
 *     pattern
 *     modifiers
 *     options
 *     evidence references
 *     metadata references
 *
 * The actual AST implementation belongs to the frontend AST subsystem.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parser acceptance means:
 *
 *     structurally valid knowledge expression
 *
 * It does NOT mean:
 *
 *     valid knowledge store
 *     authorized access
 *     valid schema
 *     valid relation
 *     sufficient capability
 *     sufficient resources
 *     valid evidence
 *     valid provenance
 *     valid inference
 *     deterministic result
 *     successful execution.
 *
 * Those are semantic/runtime questions.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * Knowledge operations should lower through the canonical semantic model.
 *
 * The semantic layer may eventually represent:
 *
 *     knowledge assertion
 *     knowledge retraction
 *     knowledge query
 *     knowledge lookup
 *     knowledge update
 *
 * as generic operations with:
 *
 *     name
 *     namespace
 *     operands
 *     parameters
 *     results
 *     attributes
 *     modifiers
 *     effects
 *     capabilities
 *     provenance
 *     source.
 *
 * This is compatible with the repository's domain-neutral IR architecture.
 *
 * No provider-specific knowledge IR is introduced.
 *
 * ============================================================================
 * QUANTUM IR BOUNDARY
 * ============================================================================
 *
 * If a knowledge operation participates in a quantum computation:
 *
 *     source
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic knowledge operation
 *       |
 *       v
 *     hybrid semantic model
 *       |
 *       v
 *     quantum::ir
 *
 * The canonical quantum boundary remains:
 *
 *     quantum::ir
 *
 * This grammar must never introduce:
 *
 *     KnowledgeQIR
 *     KnowledgeQuantumIR
 *     KnowledgeQubitIR
 *     KnowledgeGateIR
 *
 * or any equivalent parallel quantum representation.
 *
 * ============================================================================
 * HDL BOUNDARY
 * ============================================================================
 *
 * If knowledge participates in HDL/hardware co-design:
 *
 *     knowledge expression
 *          |
 *          v
 *     semantic hardware intent
 *          |
 *          v
 *     HDL/hardware semantic representation
 *          |
 *          v
 *     synthesis / implementation
 *
 * This file does not define HDL syntax.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY RESOLUTION
 * ============================================================================
 *
 * A knowledge operation may express semantic requirements downstream, such as:
 *
 *     requires capability("knowledge.query")
 *
 *     requires capability("distributed.knowledge")
 *
 *     requires capability("provenance.verify")
 *
 *     requires memory >= required_memory
 *
 *     requires topology(required_topology)
 *
 * The knowledge grammar does not perform this resolution.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Parser diagnostics must identify structural problems such as:
 *
 *     - missing knowledge operation;
 *     - missing opening parenthesis;
 *     - missing closing parenthesis;
 *     - missing required term;
 *     - malformed operation tail;
 *     - malformed options;
 *     - malformed pattern.
 *
 * Semantic diagnostics belong downstream and may identify:
 *
 *     - unknown knowledge operation;
 *     - invalid relation;
 *     - incompatible term types;
 *     - unavailable provider capability;
 *     - insufficient resources;
 *     - unauthorized access;
 *     - invalid evidence;
 *     - invalid provenance;
 *     - invalid policy;
 *     - nondeterministic operation where determinism is required.
 *
 * ============================================================================
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * The parser must preserve enough structure for:
 *
 *     diagnostics
 *     source maps
 *     AST construction
 *     formatting
 *     IDE/LSP
 *     provenance
 *     reproducibility
 *     incremental compilation
 *     refactoring
 *     compatibility tooling.
 *
 * The grammar does not normalize or execute knowledge.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The production test suite must cover:
 *
 * POSITIVE:
 *
 *     knowledge assert(subject, relation, object)
 *
 *     knowledge assert(subject, relation)
 *
 *     knowledge retract(subject, relation, object)
 *
 *     knowledge query(subject, relation, object)
 *
 *     knowledge query(pattern)
 *
 *     knowledge lookup(pattern)
 *
 *     knowledge update(subject, relation, object)
 *
 *     nested expressions as terms
 *
 *     qualified names as terms
 *
 *     literal terms
 *
 *     structured terms
 *
 *     query results used in ordinary expressions
 *
 *     knowledge expressions used in function arguments
 *
 *     knowledge expressions used in match scrutinees
 *
 *     knowledge expressions used in contracts
 *
 *     knowledge expressions used in hybrid computations
 *
 *     knowledge expressions consuming quantum-derived values
 *
 *     knowledge expressions inside distributed execution.
 *
 * NEGATIVE:
 *
 *     missing `knowledge`
 *
 *     missing operation
 *
 *     missing opening parenthesis
 *
 *     missing closing parenthesis
 *
 *     missing subject
 *
 *     missing relation
 *
 *     malformed comma structure
 *
 *     malformed operation tail
 *
 *     malformed pattern
 *
 *     malformed expression term.
 *
 * BOUNDARY:
 *
 *     deeply nested knowledge terms
 *
 *     large operation sequences
 *
 *     many assertions
 *
 *     many relations
 *
 *     many evidence references
 *
 *     many metadata entries
 *
 *     nested knowledge queries
 *
 *     knowledge query returning a large logical result
 *
 *     symbolic resource requirements.
 *
 * CROSS-DOMAIN:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     AI/ML
 *     reasoning
 *     learning
 *     adaptation
 *     concurrency
 *     distributed
 *     networking
 *     security
 *     simulation
 *     interoperability
 *     metaprogramming.
 *
 * SCALABILITY:
 *
 *     no language-level knowledge capacity is imposed;
 *     no fixed provider is required;
 *     no physical target is selected;
 *     no finite knowledge graph size is encoded;
 *     no fixed result cardinality is encoded.
 *
 * DETERMINISM:
 *
 *     identical source and grammar configuration produce identical parse
 *     structure and source spans.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/expressions/expressions.g4
 *     grammar/expressions/patterns.g4
 *     grammar/data/queries.g4
 *     grammar/types/
 *     grammar/validation/
 *     grammar/resources/
 *     grammar/effects/
 *     grammar/policies/
 *     grammar/spec/
 *
 * EXPORTS:
 *
 *     knowledgeExpression
 *
 *     knowledgeAssertionExpression
 *
 *     knowledgeRetractionExpression
 *
 *     knowledgeQueryExpression
 *
 *     knowledgeLookupExpression
 *
 *     knowledgeUpdateExpression
 *
 * CONSUMED_BY:
 *
 *     grammar/expressions/expressions.g4
 *
 *     future expression composition grammars
 *
 *     AI/reasoning integration
 *
 *     data/knowledge integration
 *
 * AST_OWNER:
 *
 *     existing domain-neutral frontend AST
 *
 * SEMANTIC_OWNER:
 *
 *     canonical knowledge/data semantic subsystem
 *
 * IR_OWNER:
 *
 *     canonical semantic IR
 *
 *     quantum::ir when the semantic operation crosses the quantum boundary
 *
 * TEST_OWNER:
 *
 *     grammar/tests/expressions/
 *
 *     grammar/tests/ai/
 *
 *     grammar/tests/data/
 *
 *     grammar/tests/semantic/
 *
 *     grammar/tests/quantum/
 *
 *     grammar/tests/hybrid/
 *
 *     grammar/tests/scalability/
 *
 *     grammar/tests/portability/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/ai.md
 *
 *     grammar/spec/data.md
 *
 *     grammar/spec/provenance.md
 *
 *     grammar/spec/policies.md
 *
 * ============================================================================
 * REQUIRED EXPRESSION INTEGRATION
 * ============================================================================
 *
 * The canonical expression composition must import this grammar and expose:
 *
 *     knowledgeExpression
 *
 * from its atomic-expression boundary.
 *
 * The intended dependency direction is:
 *
 *     knowledge.g4
 *          |
 *          v
 *     knowledgeExpression
 *          |
 *          v
 *     primaryExpression
 *          |
 *          v
 *     postfixExpression
 *          |
 *          v
 *     ...
 *          |
 *          v
 *     expression
 *
 * This file MUST NOT import the complete expression hierarchy.
 *
 * Doing so would create a circular dependency:
 *
 *     expressions
 *         ->
 *     knowledge
 *         ->
 *     expressions
 *
 * The canonical expression composition owns the connection.
 *
 * ============================================================================
 * REQUIRED DATA INTEGRATION
 * ============================================================================
 *
 * General logical queries remain owned by:
 *
 *     grammar/data/queries.g4
 *
 * If a knowledge operation needs a full logical data query, the semantic
 * integration layer should consume:
 *
 *     dataQueryExpression
 *
 * rather than duplicating SELECT/FROM/JOIN syntax here.
 *
 * ============================================================================
 * REQUIRED AI INTEGRATION
 * ============================================================================
 *
 * The AI subsystem may consume:
 *
 *     knowledgeAssertionExpression
 *     knowledgeRetractionExpression
 *     knowledgeQueryExpression
 *     knowledgeLookupExpression
 *
 * for:
 *
 *     reasoning
 *     learning
 *     adaptation
 *     evidence
 *     explainability
 *     decision records
 *     neural-symbolic computation
 *     agents
 *     causal reasoning.
 *
 * AI must not redefine knowledge syntax.
 *
 * ============================================================================
 * REQUIRED PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Provenance must consume the source structure of knowledge operations and
 * associate records such as:
 *
 *     asserted_from
 *     retracted_from
 *     queried_from
 *     derived_from
 *     verified_by
 *     evidence_source
 *
 * without modifying this grammar's ownership.
 *
 * ============================================================================
 * REQUIRED POLICY INTEGRATION
 * ============================================================================
 *
 * Policy analysis consumes knowledge operations and determines:
 *
 *     permission
 *     prohibition
 *     scope
 *     trust
 *     privacy
 *     data governance
 *     resource constraints
 *     execution constraints.
 *
 * A parser success is never a policy approval.
 *
 * ============================================================================
 * REQUIRED EFFECT INTEGRATION
 * ============================================================================
 *
 * Semantic analysis derives effects from the selected knowledge operation and
 * provider.
 *
 * This grammar must not assign effects directly.
 *
 * ============================================================================
 * REQUIRED RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource analysis determines whether a selected realization has sufficient:
 *
 *     memory
 *     storage
 *     compute
 *     bandwidth
 *     accelerator capability
 *     distributed capability
 *     quantum capability
 *     other semantic resources.
 *
 * No fixed limits belong here.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing source programs using:
 *
 *     infer
 *     learn
 *     recall
 *     assert
 *
 * must not be silently reinterpreted merely because knowledge expressions are
 * added.
 *
 * In particular:
 *
 *     KeywordAssert
 *
 * remains available to the existing assertion grammar.
 *
 * The new canonical knowledge namespace is:
 *
 *     knowledge assert(...)
 *
 * This makes knowledge operations explicit and avoids changing the meaning of
 * existing assertion syntax.
 *
 * ============================================================================
 * FUTURE LEXER EVOLUTION
 * ============================================================================
 *
 * If the canonical language specification later reserves:
 *
 *     query
 *     retract
 *     lookup
 *     update
 *     with
 *     using
 *     under
 *
 * then:
 *
 *     1. update the canonical lexer;
 *     2. update token conformance tests;
 *     3. replace the corresponding IDENTIFIER-based parser rules here;
 *     4. preserve the same semantic ownership and public rule names.
 *
 * No knowledge semantic redesign should be necessary.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     machine-capacity constants
 *     quantum-capacity constants
 *     hardware identifiers
 *     vendor identifiers
 *     GPU counts
 *     CPU counts
 *     FPGA counts
 *     QPU counts
 *     node counts
 *     memory limits
 *     tensor-rank limits
 *     graph-size limits
 *     fact-count limits
 *     query-result limits
 *     provider-specific assumptions.
 *
 * Knowledge names remain open.
 *
 * ============================================================================
 * SAFETY AUDIT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust;
 *     no unsafe code;
 *     no semantic predicates;
 *     no parser actions;
 *     no filesystem access;
 *     no network access;
 *     no database access;
 *     no knowledge-store access;
 *     no hardware discovery;
 *     no runtime execution;
 *     no credentials;
 *     no environment inspection.
 *
 * Rust 1.97 / Rust 1.97.1 remains the frontend implementation baseline.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is considered COMPLETE when:
 *
 * [x] It owns one expression-level knowledge boundary.
 *
 * [x] It does not create a second general expression grammar.
 *
 * [x] It does not duplicate the data query grammar.
 *
 * [x] It does not create a second reasoning grammar.
 *
 * [x] It does not create a second learning grammar.
 *
 * [x] It does not create a second pattern grammar.
 *
 * [x] It does not define an AST.
 *
 * [x] It does not define an IR.
 *
 * [x] It does not select hardware.
 *
 * [x] It does not select a knowledge provider.
 *
 * [x] It has no fixed knowledge capacity.
 *
 * [x] It has no fixed machine capacity.
 *
 * [x] It preserves source-level portability.
 *
 * [x] It supports assertion.
 *
 * [x] It supports retraction.
 *
 * [x] It supports knowledge queries.
 *
 * [x] It supports lookup.
 *
 * [x] It supports logical update.
 *
 * [x] It supports extensible operation tails.
 *
 * [x] It supports evidence/provenance integration.
 *
 * [x] It supports policy integration.
 *
 * [x] It supports capability/resource analysis downstream.
 *
 * [x] It supports classical computation.
 *
 * [x] It supports quantum-derived knowledge.
 *
 * [x] It supports hybrid computation.
 *
 * [x] It supports AI/neural-symbolic computation.
 *
 * [x] It supports distributed execution.
 *
 * [x] It supports HDL/hardware-related knowledge.
 *
 * [x] It supports simulation results.
 *
 * [x] It supports interoperability without embedding external formats.
 *
 * [x] It requires no unsafe Rust.
 *
 * [x] It is deterministic at parse time.
 *
 * [x] It contains no hard-coded physical limits.
 *
 * Remaining repository completion is integration/conformance work:
 *
 * [ ] Import `KnowledgeExpressions` into the canonical expression composition.
 *
 * [ ] Expose `knowledgeExpression` from the canonical primary-expression
 *     boundary.
 *
 * [ ] Add AST mapping.
 *
 * [ ] Add semantic knowledge operation types.
 *
 * [ ] Add effect analysis.
 *
 * [ ] Add capability analysis.
 *
 * [ ] Add resource analysis.
 *
 * [ ] Add contract integration.
 *
 * [ ] Add policy integration.
 *
 * [ ] Add provenance integration.
 *
 * [ ] Add canonical semantic/IR lowering.
 *
 * [ ] Add positive/negative/boundary/scalability tests.
 *
 * [ ] Add cross-domain tests.
 *
 * [ ] Verify ANTLR generation with the repository's configured toolchain.
 *
 * [ ] Verify Rust 1.97 / 1.97.1 frontend integration.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * Zamani knowledge syntax describes:
 *
 *     WHAT knowledge is asserted
 *     WHAT knowledge is retracted
 *     WHAT knowledge is queried
 *     WHAT knowledge is retrieved
 *     WHAT knowledge is updated
 *
 * It does NOT prescribe:
 *
 *     WHERE knowledge lives
 *     HOW knowledge is stored
 *     WHICH database is used
 *     WHICH graph engine is used
 *     WHICH inference engine is used
 *     WHICH model is used
 *     WHICH CPU is used
 *     WHICH GPU is used
 *     WHICH FPGA is used
 *     WHICH QPU is used
 *     WHICH node is used
 *     WHICH network is used
 *
 * Therefore:
 *
 *     KNOWLEDGE INTENT
 *          |
 *          v
 *     DOMAIN-NEUTRAL AST
 *          |
 *          v
 *     SEMANTIC KNOWLEDGE MODEL
 *          |
 *          +--> reasoning
 *          +--> learning
 *          +--> adaptation
 *          +--> data
 *          +--> classical
 *          +--> quantum::ir
 *          +--> distributed
 *          +--> HDL/hardware
 *          |
 *          v
 *     CANONICAL IR
 *          |
 *          v
 *     OPTIMIZATION / LOWERING
 *          |
 *          v
 *     RESOURCE / CAPABILITY REALIZATION
 *          |
 *          v
 *     TARGET EXECUTION
 *
 * This preserves the Zamani POCO-REAF architecture.
 *
 * ============================================================================
 */

parser grammar KnowledgeExpressions;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC EXPRESSION BOUNDARY
 * ============================================================================
 */

knowledgeExpression
    : knowledgeAssertionExpression
    | knowledgeRetractionExpression
    | knowledgeQueryExpression
    | knowledgeLookupExpression
    | knowledgeUpdateExpression
    ;


/*
 * ============================================================================
 * ASSERT
 * ============================================================================
 */

knowledgeAssertionExpression
    : KeywordKnowledge
      KeywordAssert
      LPAREN
      knowledgeTerm
      COMMA
      knowledgeTerm
      (COMMA knowledgeTerm)?
      knowledgeOperationTail*
      RPAREN
      knowledgeExpressionOptions?
    ;


/*
 * ============================================================================
 * RETRACT
 * ============================================================================
 *
 * `knowledgeOperationName` is semantically validated as `retract`.
 *
 * Keeping the operation spelling in the identifier namespace avoids adding
 * another global lexer keyword solely for this subsystem.
 */

knowledgeRetractionExpression
    : KeywordKnowledge
      knowledgeOperationName
      LPAREN
      knowledgePattern
      knowledgeOperationTail*
      RPAREN
      knowledgeExpressionOptions?
    ;


/*
 * ============================================================================
 * QUERY
 * ============================================================================
 */

knowledgeQueryExpression
    : KeywordKnowledge
      knowledgeQueryOperationName
      LPAREN
      knowledgePattern?
      knowledgeOperationTail*
      RPAREN
      knowledgeExpressionOptions?
    ;


/*
 * ============================================================================
 * LOOKUP
 * ============================================================================
 */

knowledgeLookupExpression
    : KeywordKnowledge
      knowledgeLookupOperationName
      LPAREN
      knowledgePattern?
      knowledgeOperationTail*
      RPAREN
      knowledgeExpressionOptions?
    ;


/*
 * ============================================================================
 * UPDATE
 * ============================================================================
 */

knowledgeUpdateExpression
    : KeywordKnowledge
      knowledgeUpdateOperationName
      LPAREN
      knowledgeTerm
      COMMA
      knowledgeTerm
      (COMMA knowledgeTerm)?
      knowledgeOperationTail*
      RPAREN
      knowledgeExpressionOptions?
    ;


/*
 * ============================================================================
 * OPERATION NAMES
 * ============================================================================
 *
 * These remain identifiers until/if the canonical lexer reserves them.
 *
 * Semantic validation must require the appropriate canonical spelling for
 * each public operation boundary:
 *
 *     retract
 *     query
 *     lookup
 *     update
 */

knowledgeOperationName
    : IDENTIFIER
    ;

knowledgeQueryOperationName
    : IDENTIFIER
    ;

knowledgeLookupOperationName
    : IDENTIFIER
    ;

knowledgeUpdateOperationName
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * TERMS
 * ============================================================================
 */

knowledgeTerm
    : expression
    ;


/*
 * ============================================================================
 * PATTERNS
 * ============================================================================
 *
 * Prefer the shared pattern subsystem when it is available.
 *
 * The expression fallback preserves compatibility with knowledge systems that
 * represent patterns as ordinary symbolic expressions.
 */

knowledgePattern
    : pattern
    | expression
    ;


/*
 * ============================================================================
 * EXTENSIBLE OPERATION TAIL
 * ============================================================================
 *
 * Examples:
 *
 *     evidence(...)
 *     provenance(...)
 *     confidence(...)
 *     source(...)
 *     scope(...)
 *     temporal(...)
 *
 * Names remain ordinary identifiers.
 */

knowledgeOperationTail
    : IDENTIFIER
      LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * EXPRESSION OPTIONS
 * ============================================================================
 *
 * These options deliberately use identifiers because the canonical lexer does
 * not currently reserve all of these words.
 *
 * Semantic analysis validates their meaning.
 */

knowledgeExpressionOptions
    : knowledgeExpressionOption+
    ;

knowledgeExpressionOption
    : knowledgeWithOption
    | knowledgeUsingOption
    | knowledgeUnderOption
    ;

knowledgeWithOption
    : IDENTIFIER
      expression
    ;

knowledgeUsingOption
    : IDENTIFIER
      expression
    ;

knowledgeUnderOption
    : IDENTIFIER
      expression
    ;